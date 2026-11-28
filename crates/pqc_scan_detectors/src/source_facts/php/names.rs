//! PHP namespace segments and case-aware import identities; never loads a file.
use super::super::context::{children, Context};
use super::super::{PartialCause, PartialCauses};
use std::{
    cell::Cell,
    collections::{HashMap, HashSet},
};
use tree_sitter::Node;
pub(super) const CLASS: u8 = 0;
pub(super) const FUNCTION: u8 = 1;
pub(super) const CONSTANT: u8 = 2;
struct Segment {
    start: usize,
    end: usize,
    name: String,
}
pub(super) struct Names<'a, 's, 'c> {
    pub ctx: &'c Context<'a, 's>,
    segments: Vec<Segment>,
    imports: HashMap<(usize, u8, String), (Option<String>, usize)>,
    pub import_references: Vec<(Node<'a>, String)>,
    declared: HashSet<(u8, String)>,
    remaining: Cell<usize>,
    dynamic_classes: bool,
    partial_causes: Cell<PartialCauses>,
}
impl<'a, 's, 'c> Names<'a, 's, 'c> {
    pub fn mark_partial(&self, cause: PartialCause) {
        let mut causes = self.partial_causes.get();
        causes.insert(cause);
        self.partial_causes.set(causes);
    }
    pub fn restore_partial_causes(&self, causes: PartialCauses) {
        self.partial_causes.set(causes);
    }
    pub fn partial_causes(&self) -> PartialCauses {
        self.partial_causes.get()
    }
    pub fn new(ctx: &'c Context<'a, 's>) -> Self {
        let mut result = Self {
            ctx,
            segments: vec![Segment {
                start: 0,
                end: ctx.text.len(),
                name: String::new(),
            }],
            imports: HashMap::new(),
            declared: HashSet::new(),
            import_references: Vec::new(),
            remaining: Cell::new(ctx.budget.max_ast_candidates),
            dynamic_classes: ctx
                .nodes
                .iter()
                .filter(|n| n.kind() == "function_call_expression")
                .filter_map(|n| n.child_by_field_name("function"))
                .any(|n| {
                    matches!(
                        ctx.source(n)
                            .trim_start_matches('\\')
                            .to_ascii_lowercase()
                            .as_str(),
                        "class_alias" | "eval"
                    )
                }),
            partial_causes: Cell::new(PartialCauses::default()),
        };
        if result.dynamic_classes {
            result.mark_partial(PartialCause::OtherSourceUncertainty);
        }
        let nodes: Vec<_> = ctx
            .nodes
            .iter()
            .copied()
            .filter(|n| {
                n.kind() == "namespace_definition"
                    && n.parent().is_some_and(|p| p.kind() == "program")
            })
            .collect();
        for (i, n) in nodes.iter().enumerate() {
            if !result.charge() {
                return result;
            }
            let name = n
                .child_by_field_name("name")
                .map(|n| ctx.source(n))
                .unwrap_or("");
            if name.len() > 512 {
                ctx.limit("php_namespace_bytes", 512, 512);
                return result;
            }
            let (start, end) = n
                .child_by_field_name("body")
                .map(|b| (b.start_byte(), b.end_byte()))
                .unwrap_or((
                    n.end_byte(),
                    nodes
                        .get(i + 1)
                        .map_or(ctx.text.len(), |next| next.start_byte()),
                ));
            result.segments.push(Segment {
                start,
                end,
                name: name.trim_matches('\\').to_ascii_lowercase(),
            });
        }
        result.segments.sort_by_key(|s| s.start);
        for n in &ctx.nodes {
            let kind = match n.kind() {
                "function_definition" => FUNCTION,
                "class_declaration"
                | "interface_declaration"
                | "trait_declaration"
                | "enum_declaration" => CLASS,
                "const_element" => CONSTANT,
                _ => continue,
            };
            if kind == CONSTANT && ancestor(ctx, *n, "class_declaration") {
                continue;
            }
            let name = n
                .child_by_field_name("name")
                .or_else(|| children(*n).into_iter().find(|n| n.kind() == "name"));
            if let Some(name) = name {
                if !result.charge() {
                    return result;
                }
                if let Some(raw) = result.token(name, kind) {
                    let full = result.qualify(result.segment(n.start_byte()), &raw);
                    result.declared.insert((kind, full));
                }
            }
        }
        for declaration in ctx
            .nodes
            .iter()
            .filter(|n| n.kind() == "namespace_use_declaration")
        {
            let segment = result.segment(declaration.start_byte());
            let default_kind = import_kind(*declaration, CLASS);
            let group = declaration.child_by_field_name("body");
            let prefix = if group.is_some() {
                children(*declaration)
                    .into_iter()
                    .find(|n| n.kind() == "namespace_name")
                    .map(|n| ctx.source(n))
                    .unwrap_or("")
            } else {
                ""
            };
            let clauses = children(group.unwrap_or(*declaration));
            for clause in clauses
                .into_iter()
                .filter(|n| n.kind() == "namespace_use_clause")
            {
                if !result.charge() {
                    return result;
                }
                let kind = import_kind(clause, default_kind);
                let alias = clause.child_by_field_name("alias");
                let target = children(clause).into_iter().find(|n| {
                    alias.is_none_or(|a| a.id() != n.id())
                        && matches!(n.kind(), "name" | "qualified_name")
                });
                let Some(target) = target else {
                    result.mark_partial(PartialCause::OtherSourceUncertainty);
                    continue;
                };
                let Some(raw) = result.token(target, kind) else {
                    continue;
                };
                let full = normalize(
                    &if prefix.is_empty() {
                        raw
                    } else {
                        format!(
                            "{}\\{}",
                            prefix.trim_matches('\\'),
                            raw.trim_start_matches('\\')
                        )
                    },
                    kind,
                );
                let alias = alias
                    .and_then(|a| result.token(a, kind))
                    .unwrap_or_else(|| full.rsplit('\\').next().unwrap_or("").into());
                if full.len() > 512 || alias.len() > 256 {
                    ctx.limit("php_import_bytes", 512, 512);
                    return result;
                }
                if kind == CLASS
                    && !result.dynamic_classes
                    && !result.declared.contains(&(CLASS, full.clone()))
                {
                    result.import_references.push((target, full.clone()));
                }
                let key = (segment, kind, alias);
                match result.imports.entry(key) {
                    std::collections::hash_map::Entry::Vacant(e) => {
                        e.insert((Some(full), declaration.end_byte()));
                    }
                    std::collections::hash_map::Entry::Occupied(mut e) => {
                        let at = e.get().1.min(declaration.end_byte());
                        e.insert((None, at));
                        result.mark_partial(PartialCause::OtherSourceUncertainty);
                    }
                }
            }
        }
        if !result.dynamic_classes {
            for n in ctx
                .nodes
                .iter()
                .filter(|n| n.kind() == "function_call_expression")
            {
                if n.child_by_field_name("function")
                    .and_then(|f| result.resolve(f, FUNCTION))
                    .is_some_and(|f| matches!(f.as_str(), "class_alias" | "eval"))
                {
                    result.dynamic_classes = true;
                    result.mark_partial(PartialCause::OtherSourceUncertainty);
                    result.import_references.clear();
                    break;
                }
            }
        }
        result
    }
    pub fn charge(&self) -> bool {
        let n = self.remaining.get();
        if n == 0 {
            self.ctx.limit(
                "php_source_candidates",
                self.ctx.budget.max_ast_candidates,
                self.ctx.budget.max_ast_candidates,
            );
            false
        } else {
            self.remaining.set(n - 1);
            true
        }
    }
    pub fn token(&self, node: Node<'_>, kind: u8) -> Option<String> {
        if !matches!(
            node.kind(),
            "name" | "qualified_name" | "relative_name" | "namespace_name"
        ) {
            return None;
        }
        let raw = self.ctx.source(node);
        if raw.len() > 512 {
            self.ctx.limit("php_identifier_bytes", 512, 512);
            return None;
        }
        if raw
            .chars()
            .any(|c| c.is_whitespace() || matches!(c, '$' | '\0' | '\'' | '"'))
        {
            return None;
        }
        Some(normalize(raw, kind))
    }
    fn segment(&self, offset: usize) -> usize {
        let i = self
            .segments
            .partition_point(|s| s.start <= offset)
            .saturating_sub(1);
        if self.segments[i].start <= offset && offset < self.segments[i].end {
            i
        } else {
            0
        }
    }
    fn qualify(&self, segment: usize, raw: &str) -> String {
        let ns = &self.segments[segment].name;
        if ns.is_empty() {
            raw.trim_start_matches('\\').into()
        } else {
            format!("{ns}\\{}", raw.trim_start_matches('\\'))
        }
    }
    pub(super) fn function_import_alias(&self, node: Node<'_>) -> bool {
        let raw = self.ctx.source(node);
        if raw.len() > 512 {
            return true;
        }
        let raw = raw.to_ascii_lowercase();
        let first = raw.split('\\').next().unwrap_or("");
        self.imports
            .contains_key(&(self.segment(node.start_byte()), FUNCTION, first.into()))
    }
    pub fn resolve(&self, node: Node<'_>, kind: u8) -> Option<String> {
        if kind == CLASS && self.dynamic_classes {
            return None;
        }
        if !self.charge() {
            return None;
        }
        let raw = self.token(node, kind)?;
        let segment = self.segment(node.start_byte());
        let full = if raw.starts_with('\\') {
            raw.trim_start_matches('\\').to_owned()
        } else if let Some(rest) = raw.strip_prefix("namespace\\") {
            self.qualify(segment, rest)
        } else {
            let (first, rest) = raw
                .split_once('\\')
                .map_or((raw.as_str(), ""), |(a, b)| (a, b));
            let alias_kind = if rest.is_empty() { kind } else { CLASS };
            if let Some((target, at)) = self.imports.get(&(segment, alias_kind, first.to_owned())) {
                if node.start_byte() < *at {
                    self.mark_partial(PartialCause::OtherSourceUncertainty);
                    return None;
                }
                let Some(target) = target else {
                    self.mark_partial(PartialCause::OtherSourceUncertainty);
                    return None;
                };
                if rest.is_empty() {
                    target.clone()
                } else {
                    format!("{target}\\{rest}")
                }
            } else {
                let local = self.qualify(segment, &raw);
                if self.declared.contains(&(kind, local.clone())) {
                    return None;
                }
                if (kind == FUNCTION || kind == CONSTANT) && rest.is_empty() {
                    raw
                } else {
                    local
                }
            }
        };
        (!self.declared.contains(&(kind, full.clone()))).then_some(full)
    }
}
fn normalize(raw: &str, kind: u8) -> String {
    if kind == CONSTANT {
        raw.rsplit_once('\\').map_or_else(
            || raw.into(),
            |(a, b)| format!("{}\\{b}", a.to_ascii_lowercase()),
        )
    } else {
        raw.to_ascii_lowercase()
    }
}
fn import_kind(node: Node<'_>, default: u8) -> u8 {
    match node.child_by_field_name("type").map(|n| n.kind()) {
        Some("function") => FUNCTION,
        Some("const") => CONSTANT,
        _ => default,
    }
}
pub(super) fn ancestor(ctx: &Context<'_, '_>, mut node: Node<'_>, kind: &str) -> bool {
    for _ in 0..64 {
        let Some(p) = node.parent() else { return false };
        if p.kind() == kind {
            return true;
        }
        node = p;
    }
    ctx.limit("php_ancestor_depth", 64, 64);
    true
}
