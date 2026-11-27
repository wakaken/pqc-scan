//! Case-sensitive, file-local C# names. No project-wide lookup or using expansion.
use super::super::context::{children, Context};
use super::super::{PartialCause, PartialCauses};
use std::{
    cell::Cell,
    collections::{HashMap, HashSet},
};
use tree_sitter::Node;
struct Import {
    scope: usize,
    at: usize,
    alias: Option<String>,
    target: String,
    static_: bool,
}
pub(super) struct Names<'a, 's, 'c> {
    pub ctx: &'c Context<'a, 's>,
    imports: Vec<Import>,
    types: HashSet<String>,
    shadows: HashSet<(usize, String)>,
    vars: HashSet<(usize, String)>,
    declared_types: HashMap<(usize, String), Option<Node<'a>>>,
    file_namespace: Option<(usize, String)>,
    remaining: Cell<usize>,
    partial_causes: Cell<PartialCauses>,
    pub references: Vec<(Node<'a>, String)>,
}
impl<'a, 's, 'c> Names<'a, 's, 'c> {
    pub fn mark_partial(&self, cause: PartialCause) {
        let mut causes = self.partial_causes.get();
        causes.insert(cause);
        self.partial_causes.set(causes);
    }
    pub fn partial_causes(&self) -> PartialCauses {
        self.partial_causes.get()
    }
    pub fn new(ctx: &'c Context<'a, 's>) -> Self {
        let mut out = Self {
            ctx,
            imports: vec![],
            types: HashSet::new(),
            shadows: HashSet::new(),
            vars: HashSet::new(),
            declared_types: HashMap::new(),
            file_namespace: None,
            remaining: Cell::new(ctx.budget.max_ast_candidates),
            partial_causes: Cell::new(PartialCauses::default()),
            references: vec![],
        };
        for n in &ctx.nodes {
            if !out.charge() {
                return out;
            }
            if n.kind().starts_with("preproc_if") || n.kind() == "preproc_else" {
                out.mark_partial(PartialCause::ConditionalSource);
            }
            if in_error(*n, ctx) {
                continue;
            }
            if matches!(
                n.kind(),
                "property_declaration" | "event_declaration" | "variable_declarator" | "parameter"
            ) {
                if let Some(name) = n.child_by_field_name("name").and_then(|v| out.path(v)) {
                    let mut ty = n.child_by_field_name("type").or_else(|| {
                        (n.kind() == "variable_declarator")
                            .then(|| n.parent())
                            .flatten()
                            .and_then(|p| p.child_by_field_name("type"))
                    });
                    if ty.is_some_and(|t| t.kind() == "implicit_type") {
                        ty = children(*n)
                            .into_iter()
                            .find(|v| v.kind() == "object_creation_expression")
                            .and_then(|v| v.child_by_field_name("type"));
                    }
                    out.declared_types
                        .entry((ctx.scope(*n), name))
                        .and_modify(|v| *v = None)
                        .or_insert(ty);
                }
            }
            if matches!(
                n.kind(),
                "declaration_pattern"
                    | "var_pattern"
                    | "recursive_pattern"
                    | "list_pattern"
                    | "parenthesized_variable_designation"
                    | "tuple_pattern"
            ) {
                let mut cursor = n.walk();
                for name in n.children_by_field_name("name", &mut cursor) {
                    if let Some(name) = out.path(name) {
                        out.vars.insert((ctx.scope(*n), name));
                    }
                }
            }
            if n.kind() == "implicit_parameter" {
                if let Some(name) = out.path(*n) {
                    out.vars.insert((ctx.scope(*n), name));
                }
            }
            if n.kind() == "assignment_expression" {
                if let Some(left) = n.child_by_field_name("left") {
                    let mut stack = vec![left];
                    while let Some(node) = stack.pop() {
                        if !out.charge() {
                            return out;
                        }
                        if node.kind() == "identifier" {
                            if let Some(name) = out.path(node) {
                                out.vars.insert((ctx.scope(*n), name));
                            }
                        } else if matches!(
                            node.kind(),
                            "tuple_expression" | "tuple_pattern" | "argument"
                        ) {
                            stack.extend(children(node));
                        }
                    }
                }
            }
            match n.kind() {
                "file_scoped_namespace_declaration" => {
                    if let Some(name) = n.child_by_field_name("name").and_then(|n| out.path(n)) {
                        out.file_namespace = Some((n.end_byte(), name));
                    }
                }
                "class_declaration"
                | "struct_declaration"
                | "interface_declaration"
                | "record_declaration"
                | "enum_declaration"
                | "delegate_declaration" => {
                    if let Some(name) = n.child_by_field_name("name").and_then(|n| out.path(n)) {
                        out.types
                            .insert(format!("{}{}", out.namespace(*n, true), name));
                        out.shadows.insert((ctx.parent_scope(*n), name));
                    }
                }
                "type_parameter" => {
                    if let Some(name) = n.child_by_field_name("name").and_then(|n| out.path(n)) {
                        out.shadows.insert((ctx.scope(*n), name));
                    }
                }
                "property_declaration" | "event_declaration" => {
                    if let Some(name) = n.child_by_field_name("name").and_then(|n| out.path(n)) {
                        out.vars.insert((ctx.scope(*n), name));
                    }
                }
                "variable_declarator"
                | "parameter"
                | "catch_declaration"
                | "declaration_expression" => {
                    if let Some(name) = n.child_by_field_name("name").and_then(|n| out.path(n)) {
                        out.vars.insert((ctx.scope(*n), name));
                    }
                }
                "foreach_statement" => {
                    if let Some(name) = n.child_by_field_name("left").and_then(|n| out.path(n)) {
                        out.vars.insert((ctx.scope(*n), name));
                    }
                }
                "method_declaration" | "local_function_statement" => {
                    if let Some(name) = n.child_by_field_name("name").and_then(|n| out.path(n)) {
                        out.shadows.insert((ctx.parent_scope(*n), name));
                    }
                }
                "using_directive" => {
                    let alias_node = n.child_by_field_name("name");
                    let target = children(*n)
                        .into_iter()
                        .rfind(|c| alias_node.is_none_or(|a| a.id() != c.id()));
                    if let Some(target_node) = target {
                        if let Some(target) = out.path(target_node) {
                            let alias = alias_node.and_then(|n| out.path(n));
                            out.references.push((target_node, target.clone()));
                            if out.imports.len() >= 64 {
                                ctx.limit("csharp_import_entries", 64, 64);
                                return out;
                            }
                            out.imports.push(Import {
                                scope: ctx.scope(*n),
                                at: n.end_byte(),
                                alias,
                                target,
                                static_: ctx.source(*n).split_whitespace().any(|s| s == "static"),
                            });
                        }
                    }
                }
                _ => {}
            }
        }
        out
    }
    pub fn charge(&self) -> bool {
        let n = self.remaining.get();
        if n == 0 {
            self.ctx.limit(
                "csharp_source_candidates",
                self.ctx.budget.max_ast_candidates,
                self.ctx.budget.max_ast_candidates,
            );
            false
        } else {
            self.remaining.set(n - 1);
            true
        }
    }
    pub fn path(&self, node: Node<'_>) -> Option<String> {
        let mut pending = vec![node];
        while let Some(part) = pending.pop() {
            if !self.charge() {
                return None;
            }
            let qualifier = match part.kind() {
                "identifier" | "implicit_parameter" | "predefined_type" => continue,
                "qualified_name" => "qualifier",
                "alias_qualified_name" => "alias",
                "member_access_expression" => "expression",
                _ => return None,
            };
            if part.named_child_count() != 2 {
                return None;
            }
            pending.push(part.child_by_field_name(qualifier)?);
            pending.push(part.child_by_field_name("name")?);
        }
        let raw = self.ctx.source(node);
        if raw.len() > 512 {
            self.ctx.limit("csharp_identifier_bytes", 512, 512);
            return None;
        }
        if raw.contains('\\') {
            self.mark_partial(PartialCause::OtherSourceUncertainty);
            return None;
        }
        let raw: String = raw.chars().filter(|c| !c.is_whitespace()).collect();
        if raw.is_empty()
            || !raw
                .chars()
                .all(|c| c.is_alphanumeric() || "_.:@".contains(c))
        {
            return None;
        }
        let raw = if let Some(rest) = raw.strip_prefix("global::") {
            rest.to_owned()
        } else if let Some((alias, rest)) = raw.split_once("::") {
            if alias.contains('.') || rest.contains(':') {
                return None;
            }
            format!("{alias}.{rest}")
        } else {
            raw
        };
        if raw.contains(':') {
            return None;
        }
        Some(
            raw.split('.')
                .map(|s| s.strip_prefix('@').unwrap_or(s))
                .collect::<Vec<_>>()
                .join("."),
        )
    }
    pub fn visible(&self, outer: usize, mut inner: usize) -> bool {
        for _ in 0..64 {
            if inner == outer {
                return true;
            }
            let Some(p) = self.ctx.scope_parent(inner) else {
                return false;
            };
            inner = p;
        }
        self.ctx.limit("csharp_scope_depth", 64, 64);
        false
    }
    pub(super) fn namespace(&self, node: Node<'_>, classes: bool) -> String {
        let mut parts = vec![];
        let mut p = node.parent();
        for _ in 0..64 {
            let Some(n) = p else {
                break;
            };
            if n.kind() == "namespace_declaration"
                || (classes
                    && matches!(
                        n.kind(),
                        "class_declaration" | "struct_declaration" | "record_declaration"
                    ))
            {
                if let Some(name) = n.child_by_field_name("name").and_then(|n| self.path(n)) {
                    parts.push(name);
                }
            }
            p = n.parent();
        }
        if p.is_some() {
            self.ctx.limit("csharp_namespace_depth", 64, 64);
            return String::new();
        }
        parts.reverse();
        // A file-scoped namespace is a preceding sibling, not an AST ancestor.
        if let Some((at, name)) = &self.file_namespace {
            if *at <= node.start_byte() {
                parts.insert(0, name.clone());
            }
        }
        if parts.is_empty() {
            String::new()
        } else {
            format!("{}.", parts.join("."))
        }
    }
    pub fn resolve(&self, node: Node<'_>, ignore_vars: bool) -> Vec<String> {
        if !self.charge() {
            return vec![];
        }
        let Some(raw) = self.path(node) else {
            return vec![];
        };
        let compact: String = self
            .ctx
            .source(node)
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();
        let global = compact.starts_with("global::");
        let alias_qualified = !global && compact.contains("::");
        let first = raw.split('.').next().unwrap_or("");
        let scope = self.ctx.scope(node);
        if !global && !alias_qualified {
            let mut current = Some(scope);
            for _ in 0..64 {
                let Some(s) = current else {
                    break;
                };
                if self.shadows.contains(&(s, first.to_owned())) {
                    return vec![];
                }
                if !ignore_vars && self.vars.contains(&(s, first.to_owned())) {
                    let prefix = if raw == first {
                        Some(node)
                    } else {
                        node.child_by_field_name("expression")
                            .filter(|n| self.path(*n).as_deref() == Some(first))
                    };
                    if !prefix.is_some_and(|n| self.identical_declared_type(n, s, first)) {
                        return vec![];
                    }
                    current = None;
                    break;
                }
                current = self.ctx.scope_parent(s);
            }
            if current.is_some() {
                self.ctx.limit("csharp_scope_depth", 64, 64);
                return vec![];
            }
        }
        let mut result = vec![];
        if global {
            result.push(raw);
        } else {
            let mut current = Some(scope);
            let mut alias_found = false;
            for _ in 0..64 {
                let Some(s) = current else {
                    break;
                };
                let aliases: Vec<_> = self
                    .imports
                    .iter()
                    .filter(|i| i.scope == s && i.alias.as_deref() == Some(first))
                    .collect();
                if !aliases.is_empty() {
                    alias_found = true;
                    if aliases.len() != 1 || node.start_byte() < aliases[0].at {
                        self.mark_partial(PartialCause::OtherSourceUncertainty);
                        return vec![];
                    }
                    if alias_qualified
                        && (super::class_info(&aliases[0].target)
                            .is_some_and(|k| k.family().is_some())
                            || super::parameter_family(&format!("{}._", aliases[0].target))
                                .is_some()
                            || self.types.contains(&aliases[0].target))
                    {
                        self.mark_partial(PartialCause::OtherSourceUncertainty);
                        return vec![];
                    }
                    let suffix = &raw[first.len()..];
                    result.push(format!("{}{suffix}", aliases[0].target));
                    break;
                }
                current = self.ctx.scope_parent(s);
            }
            if !alias_found && current.is_some() {
                self.ctx.limit("csharp_scope_depth", 64, 64);
                return vec![];
            }
            if alias_qualified && !alias_found {
                self.mark_partial(PartialCause::OtherSourceUncertainty);
                return vec![];
            }
            if !alias_found {
                result.push(raw.clone());
                result.push(format!("{}{}", self.namespace(node, false), raw));
                for import in &self.imports {
                    if import.alias.is_none()
                        && self.visible(import.scope, scope)
                        && import.at <= node.start_byte()
                    {
                        result.push(format!("{}.{}", import.target, raw));
                    }
                }
            }
        }
        result.retain(|n| self.external_path(n));
        result.sort();
        result.dedup();
        result
    }
    pub fn external_path(&self, name: &str) -> bool {
        !self.types.contains(name)
            && !name
                .char_indices()
                .filter(|(_, c)| *c == '.')
                .any(|(i, _)| self.types.contains(&name[..i]))
    }
    // C# §12.8.7.2 permits E.I for a value E whose declared type is the
    // identical canonical type denoted by E. Family equivalence is insufficient.
    fn identical_declared_type(&self, node: Node<'_>, scope: usize, name: &str) -> bool {
        let Some(Some(ty)) = self.declared_types.get(&(scope, name.to_owned())) else {
            return false;
        };
        let canonical = |n| {
            self.resolve(n, true)
                .into_iter()
                .filter(|p| super::class_info(p).is_some())
                .collect::<Vec<_>>()
        };
        let expression_types = canonical(node);
        let declared = canonical(*ty);
        expression_types.len() == 1 && declared == expression_types
    }
    // An opaque chain/XML object's declared class survives mutations to its
    // settings, but no key algorithm, policy success or parameter is inferred.
    pub fn opaque_receiver(&self, node: Node<'_>) -> Option<super::Kind> {
        if node.kind() != "identifier" {
            return None;
        }
        let raw = self.path(node)?;
        let mut scope = Some(self.ctx.scope(node));
        for _ in 0..64 {
            if !self.charge() {
                return None;
            }
            let s = scope?;
            if self.vars.contains(&(s, raw.clone())) {
                let ty = self.declared_types.get(&(s, raw))?.as_ref()?;
                let kinds: Vec<_> = self
                    .resolve(*ty, true)
                    .iter()
                    .filter_map(|n| super::class_info(n))
                    .filter(|k| {
                        matches!(k, super::Kind::CertificateChain | super::Kind::XmlSignature)
                    })
                    .collect();
                return (kinds.len() == 1).then(|| kinds[0]);
            }
            scope = self.ctx.scope_parent(s);
        }
        self.ctx.limit("csharp_scope_depth", 64, 64);
        None
    }
    pub fn static_candidates(&self, node: Node<'_>) -> Vec<String> {
        let mut out = self.resolve(node, false);
        if out.is_empty() {
            return out;
        }
        if node.kind() == "identifier" {
            if let Some(raw) = self.path(node) {
                for i in &self.imports {
                    if i.static_
                        && i.at <= node.start_byte()
                        && self.visible(i.scope, self.ctx.scope(node))
                    {
                        out.push(format!("{}.{}", i.target, raw));
                    }
                }
            }
        }
        out.sort();
        out.dedup();
        out
    }
}
pub(super) fn in_error(mut node: Node<'_>, ctx: &Context<'_, '_>) -> bool {
    for _ in 0..64 {
        if node.kind() == "ERROR" {
            return true;
        }
        let Some(p) = node.parent() else {
            return false;
        };
        node = p;
    }
    ctx.limit("csharp_ancestor_depth", 64, 64);
    true
}
