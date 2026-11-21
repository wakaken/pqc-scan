//! Bounded relative ESM facts from files selected by the current scan.
//! Never runs code, resolves packages, follows links or guesses dynamic exports.
use super::{
    context::{children, literal, unwrap_expression, Context},
    javascript, jwt, Facts,
};
use pqc_scan_core::{RepositoryContext, ScannableFile};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
};
use tree_sitter::{Node, Parser};

#[derive(Clone)]
enum Export {
    Module,
    Constant(String),
    Signer(String),
}
struct Resolver<'a> {
    repository: &'a RepositoryContext,
    stack: Vec<PathBuf>,
    files: usize,
    parses: usize,
    discovery_probes: usize,
    discovery_files: usize,
    discovery_bytes: usize,
    validated: BTreeSet<(PathBuf, Vec<u8>)>,
    loaded: BTreeMap<(PathBuf, String), Option<ScannableFile>>,
    exports: BTreeMap<(Vec<PathBuf>, PathBuf, String, String), Option<Export>>,
    bytes: usize,
    limited: bool,
    limit_reason: Option<(&'static str, usize, usize)>,
}
fn nodes(root: Node<'_>) -> Option<Vec<Node<'_>>> {
    let mut result = Vec::new();
    let mut stack = vec![root];
    while let Some(node) = stack.pop() {
        if result.len() + stack.len() >= 100_000 || node.named_child_count() > 100_000 {
            return None;
        }
        result.push(node);
        let mut cursor = node.walk();
        stack.extend(node.named_children(&mut cursor));
    }
    Some(result)
}
fn imports<'a>(all: &[Node<'a>], text: &str) -> BTreeMap<String, (String, String, usize)> {
    let mut out = BTreeMap::new();
    for &node in all.iter().filter(|n| n.kind() == "import_statement") {
        let Some(source) = node
            .child_by_field_name("source")
            .and_then(|n| literal(text, n))
        else {
            continue;
        };
        if !source.starts_with("./") && !source.starts_with("../") {
            continue;
        }
        for spec in nodes(node)
            .unwrap_or_default()
            .into_iter()
            .filter(|n| n.kind() == "import_specifier")
        {
            if text[spec.byte_range()].trim_start().starts_with("type ")
                || text[node.byte_range()]
                    .trim_start()
                    .starts_with("import type ")
            {
                continue;
            }
            let Some(name) = spec.child_by_field_name("name") else {
                continue;
            };
            let alias = spec.child_by_field_name("alias").unwrap_or(name);
            out.insert(
                text[alias.byte_range()].to_string(),
                (source.into(), text[name.byte_range()].into(), alias.id()),
            );
        }
    }
    out
}
impl Resolver<'_> {
    fn discover(&mut self, from: &Path, relative: &str) -> Option<ScannableFile> {
        // Exact source suffix or unambiguous extensionless sibling only.
        if relative.contains(['\\', '\0', '?', '#']) {
            return None;
        }
        let path = Path::new(relative);
        let choices = if path.extension().is_some() {
            if !matches!(path.extension()?.to_str()?, "js" | "ts" | "mjs" | "mts") {
                return None;
            }
            vec![relative.to_string()]
        } else {
            ["js", "ts", "mjs", "mts"]
                .iter()
                .map(|ext| format!("{relative}.{ext}"))
                .collect()
        };
        let key = (from.to_path_buf(), relative.to_string());
        if let Some(file) = self.loaded.get(&key) {
            return file.clone();
        }
        let mut found = None;
        for candidate in choices {
            if self.discovery_probes >= 256 {
                self.limited = true;
                self.limit_reason = Some(("relative_discovery_probes", 256, self.discovery_probes));
                return None;
            }
            if self.discovery_files >= 64 {
                self.limited = true;
                self.limit_reason = Some(("relative_discovery_files", 64, self.discovery_files));
                return None;
            }
            self.discovery_probes += 1;
            if let Some(file) = self.repository.read_relative(from, Path::new(&candidate)) {
                self.discovery_files += 1;
                if file.bytes.len() > (4 * 1024 * 1024usize).saturating_sub(self.discovery_bytes) {
                    self.limited = true;
                    self.limit_reason = Some((
                        "relative_discovery_bytes",
                        4 * 1024 * 1024,
                        self.discovery_bytes,
                    ));
                    self.loaded.insert(key, None);
                    return None;
                }
                self.discovery_bytes += file.bytes.len();
                if found.is_some() {
                    self.loaded.insert(key, None);
                    return None;
                }
                found = Some(file);
            }
        }
        self.loaded.insert(key, found.clone());
        found
    }
    fn load(&mut self, from: &Path, relative: &str) -> Option<ScannableFile> {
        let file = self.discover(from, relative)?;
        if file.bytes.len() > 64 * 1024 {
            self.limited = true;
            self.limit_reason = Some(("relative_module_bytes", 64 * 1024, 0));
            return None;
        }
        let snapshot = (file.path.clone(), file.bytes.clone());
        if !self.validated.contains(&snapshot) {
            if self.files >= 16 {
                self.limited = true;
                self.limit_reason = Some(("relative_module_reads", 16, self.files));
                return None;
            }
            if file.bytes.len() > (256 * 1024usize).saturating_sub(self.bytes) {
                self.limited = true;
                self.limit_reason = Some(("relative_total_bytes", 256 * 1024, self.bytes));
                return None;
            }
            self.files += 1;
            self.bytes += file.bytes.len();
            self.validated.insert(snapshot);
        }
        Some(file)
    }
    fn signer(&mut self, from: &Path, relative: &str, name: &str) -> Option<String> {
        let file = self.discover(from, relative)?;
        // Supported wrappers must directly import jsonwebtoken. This is only
        // an exclusion preflight: a substring match never establishes a fact.
        // Constants and module-initialization validation do not use this gate.
        if !file.text.as_deref()?.contains("jsonwebtoken") {
            return None;
        }
        match self.export(from, relative, name)? {
            Export::Signer(value) => Some(value),
            _ => None,
        }
    }
    fn export(&mut self, from: &Path, relative: &str, name: &str) -> Option<Export> {
        let key = (
            self.stack.clone(),
            from.to_path_buf(),
            relative.to_string(),
            name.to_string(),
        );
        if let Some(value) = self.exports.get(&key) {
            return value.clone();
        }
        if self.exports.len() >= 128 {
            self.limited = true;
            self.limit_reason = Some(("relative_export_cache", 128, self.exports.len()));
            return None;
        }
        let result = self.export_inner(from, relative, name);
        self.exports.insert(key, result.clone());
        result
    }
    fn export_inner(&mut self, from: &Path, relative: &str, name: &str) -> Option<Export> {
        if self.stack.len() >= 4 {
            self.limited = true;
            self.limit_reason = Some(("relative_module_depth", 4, self.stack.len()));
            return None;
        }
        let file = self.load(from, relative)?;
        if self.stack.contains(&file.path) {
            return None;
        }
        let text = file.text.as_deref()?;
        let (language, language_name) =
            crate::tree_sitter_detector::resolve_language(&file.path, text)?;
        if self.parses >= 32 {
            self.limited = true;
            self.limit_reason = Some(("relative_module_parses", 32, self.parses));
            return None;
        }
        self.parses += 1;
        let mut parser = Parser::new();
        parser.set_language(&language).ok()?;
        let tree = parser.parse(text, None)?;
        if tree.root_node().has_error() {
            return None;
        }
        let Some(all) = nodes(tree.root_node()) else {
            self.limited = true;
            self.limit_reason = Some(("relative_module_nodes", 100_000, 100_000));
            return None;
        };
        // Pure module surface: imports, immutable primitive declarations and
        // function declarations only. Calls/assignments/classes/re-exports at
        // module scope make initialization effects unknown.
        for top in children(tree.root_node()) {
            if top.kind() == "comment" {
                continue;
            }
            if top.kind() == "import_statement" {
                let source = top
                    .child_by_field_name("source")
                    .and_then(|n| literal(text, n))?;
                if source != "jsonwebtoken"
                    && !source.starts_with("./")
                    && !source.starts_with("../")
                {
                    return None;
                }
                if !children(top).iter().any(|n| n.kind() == "import_clause") {
                    return None;
                }
                continue;
            }
            let declaration = if top.kind() == "export_statement" {
                top.child_by_field_name("declaration")?
            } else {
                top
            };
            if declaration.kind() == "function_declaration" {
                continue;
            }
            if declaration.kind() != "lexical_declaration"
                || !text[declaration.byte_range()]
                    .trim_start()
                    .starts_with("const ")
            {
                return None;
            }
            for item in children(declaration)
                .into_iter()
                .filter(|n| n.kind() != "comment")
            {
                let value = item.child_by_field_name("value")?;
                if literal(text, value).is_none() && value.kind() != "identifier" {
                    return None;
                }
            }
        }
        let mut ctx = Context::new(
            tree.root_node(),
            text,
            language_name,
            crate::DetectorBudget::default(),
        );
        let mut ignored = Facts::default();
        javascript::inspect(&mut ctx, &mut ignored, &pqc_scan_rules::RuleSet::default());
        if ctx.truncated.get() {
            self.limited = true;
            return None;
        }
        let import_map = imports(&all, text);
        self.stack.push(file.path.clone());
        let result = (|| {
            // ESM imports initialize even when their named binding is unused.
            // Validate every relative import's pure surface, not just values
            // requested by the selected signing expression.
            for top in children(tree.root_node())
                .into_iter()
                .filter(|n| n.kind() == "import_statement")
            {
                let source = top
                    .child_by_field_name("source")
                    .and_then(|n| literal(text, n))?;
                if source.starts_with("./") || source.starts_with("../") {
                    self.export(&file.path, source, "")?;
                }
            }
            for top in children(tree.root_node()) {
                let declaration = if top.kind() == "export_statement" {
                    top.child_by_field_name("declaration")?
                } else {
                    top
                };
                if declaration.kind() == "lexical_declaration" {
                    for item in children(declaration) {
                        self.value(
                            &ctx,
                            &file.path,
                            &import_map,
                            item.child_by_field_name("value")?,
                        )?;
                    }
                }
            }
            if name.is_empty() {
                return Some(Export::Module);
            }
            let mut result = None;
            for export in children(tree.root_node())
                .into_iter()
                .filter(|n| n.kind() == "export_statement")
            {
                let declaration = export.child_by_field_name("declaration")?;
                if declaration.kind() == "lexical_declaration" {
                    for item in children(declaration) {
                        let Some(binding) = item.child_by_field_name("name") else {
                            continue;
                        };
                        if &text[binding.byte_range()] == name {
                            if result.is_some()
                                || ctx
                                    .lookup(binding, name)
                                    .is_none_or(|b| b.id != binding.id() || !ctx.unchanged(b))
                            {
                                return None;
                            }
                            result = Some(Export::Constant(self.value(
                                &ctx,
                                &file.path,
                                &import_map,
                                item.child_by_field_name("value")?,
                            )?));
                        }
                    }
                } else if declaration.kind() == "function_declaration"
                    && declaration
                        .child_by_field_name("name")
                        .is_some_and(|n| &text[n.byte_range()] == name)
                {
                    if result.is_some() || jwt::namespace_may_change(&ctx) {
                        return None;
                    }
                    let declared = declaration.child_by_field_name("name")?;
                    if ctx
                        .lookup(declared, name)
                        .is_none_or(|b| b.id != declared.id() || !ctx.unchanged(b))
                    {
                        return None;
                    }

                    let params = children(declaration.child_by_field_name("parameters")?);
                    if params.len() != 2 {
                        return None;
                    }
                    let params: Option<Vec<_>> = params
                        .into_iter()
                        .map(|n| {
                            let n = if n.kind() == "required_parameter" {
                                n.child_by_field_name("pattern")?
                            } else {
                                n
                            };
                            (n.kind() == "identifier").then(|| text[n.byte_range()].to_string())
                        })
                        .collect();
                    let params = params?;
                    if params[0] == params[1] {
                        return None;
                    }
                    let statements: Vec<_> = children(declaration.child_by_field_name("body")?)
                        .into_iter()
                        .filter(|n| n.kind() != "comment")
                        .collect();
                    if statements.len() != 1 || statements[0].kind() != "return_statement" {
                        return None;
                    }
                    let call = statements[0].named_child(0)?;
                    if call.kind() != "call_expression"
                        || !jwt::signing(&ctx, call.child_by_field_name("function")?, 0)
                    {
                        return None;
                    }
                    let args = children(call.child_by_field_name("arguments")?);
                    if args.len() != 3
                        || args[..2].iter().zip(params).any(|(n, name)| {
                            n.kind() != "identifier" || text[n.byte_range()] != name
                        })
                    {
                        return None;
                    }
                    result = Some(Export::Signer(self.algorithm(
                        &ctx,
                        &file.path,
                        &import_map,
                        args[2],
                    )?));
                }
            }
            result
        })();
        self.stack.pop();
        result
    }
    fn value(
        &mut self,
        ctx: &Context<'_, '_>,
        from: &Path,
        imports: &BTreeMap<String, (String, String, usize)>,
        node: Node<'_>,
    ) -> Option<String> {
        let node = unwrap_expression(node);
        if let Some(s) = literal(ctx.text, node) {
            return Some(s.into());
        }
        if node.kind() != "identifier" {
            return None;
        }
        let binding = ctx
            .lookup(node, ctx.source(node))
            .filter(|b| ctx.unchanged(b))?;
        let (path, name, import_id) = imports.get(ctx.source(node))?;
        if binding.id != *import_id {
            return None;
        }
        if binding.value.is_some() || binding.imported.is_some() {
            return None;
        }
        match self.export(from, path, name)? {
            Export::Constant(s) => Some(s),
            _ => None,
        }
    }
    fn algorithm(
        &mut self,
        ctx: &Context<'_, '_>,
        from: &Path,
        imports: &BTreeMap<String, (String, String, usize)>,
        options: Node<'_>,
    ) -> Option<String> {
        let name = self.value(
            ctx,
            from,
            imports,
            jwt::property(ctx, options, "algorithm")??,
        )?;
        if !matches!(
            name.as_str(),
            "RS256" | "RS384" | "RS512" | "ES256" | "ES384" | "ES512" | "EdDSA"
        ) {
            return None;
        }
        if let Some(header) = jwt::property(ctx, options, "header")? {
            if let Some(alg) = jwt::property(ctx, header, "alg")? {
                if self.value(ctx, from, imports, alg)? != name {
                    return None;
                }
            }
        }
        Some(name)
    }
}

pub(super) fn inspect(
    ctx: &Context<'_, '_>,
    facts: &mut Facts,
    file: &Path,
    repository: &RepositoryContext,
) {
    if jwt::namespace_may_change(ctx) {
        return;
    }
    let import_map = imports(&ctx.nodes, ctx.text);
    if import_map.is_empty() {
        return;
    }
    let mut resolver = Resolver {
        repository,
        stack: vec![file.to_path_buf()],
        files: 0,
        parses: 0,
        discovery_probes: 0,
        discovery_files: 0,
        discovery_bytes: 0,
        validated: BTreeSet::new(),
        loaded: BTreeMap::new(),
        exports: BTreeMap::new(),
        bytes: 0,
        limited: false,
        limit_reason: None,
    };
    for (i, &call) in ctx
        .nodes
        .iter()
        .filter(|n| n.kind() == "call_expression")
        .enumerate()
    {
        if i >= ctx.budget.max_match_candidates {
            facts.truncated = true;
            break;
        }
        let Some(function) = call.child_by_field_name("function") else {
            continue;
        };
        let Some(args) = call.child_by_field_name("arguments") else {
            continue;
        };
        if args.end_byte() - args.start_byte() > ctx.budget.max_ast_match_bytes {
            facts.truncated = true;
            break;
        }
        let name = if args.named_child_count() == 3 && jwt::signing(ctx, function, 0) {
            args.named_child(2)
                .and_then(|n| resolver.algorithm(ctx, file, &import_map, n))
        } else if args.named_child_count() == 2
            && function.kind() == "identifier"
            && ctx
                .lookup(function, ctx.source(function))
                .is_some_and(|b| ctx.unchanged(b) && b.value.is_none() && b.imported.is_none())
        {
            import_map
                .get(ctx.source(function))
                .and_then(|(path, name, import_id)| {
                    if ctx
                        .lookup(function, ctx.source(function))
                        .is_some_and(|b| b.id == *import_id)
                    {
                        resolver.signer(file, path, name)
                    } else {
                        None
                    }
                })
        } else {
            None
        };
        if let Some(name) = name {
            // Literal same-file signing is already emitted by jwt::inspect.
            if facts
                .hits
                .get("TS_NODE_JOSE_USAGE")
                .is_some_and(|hits| hits.iter().any(|h| h.start == call.start_byte()))
            {
                continue;
            }
            let next = facts.hits.get("TS_NODE_JOSE_USAGE").map_or(0, Vec::len);
            facts.callee(
                "TS_NODE_JOSE_USAGE",
                call,
                "bounded_relative_esm_signing_reference",
            );
            if let Some(hit) = facts
                .hits
                .get_mut("TS_NODE_JOSE_USAGE")
                .and_then(|v| v.get_mut(next))
            {
                hit.algorithm_references.push(name);
            }
        }
    }
    facts.truncated |= resolver.limited;
    facts.limit_reason = facts.limit_reason.or(resolver.limit_reason);
}
