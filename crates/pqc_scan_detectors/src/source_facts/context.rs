//! Bounded lexical scopes and literal provenance. No evaluation or module loading.
use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use tree_sitter::Node;

pub(super) const MAX_DEPTH: usize = 8;
#[derive(Clone)]
pub(super) struct Binding<'a> {
    pub id: usize,
    declared_at: usize,
    pub value: Option<Node<'a>>,
    pub projection: Option<String>,
    pub imported: Option<String>,
}
struct Scope<'a> {
    parent: Option<usize>,
    function: bool,
    class: bool,
    bindings: HashMap<String, Vec<Binding<'a>>>,
}
pub(super) struct Context<'a, 's> {
    pub nodes: Vec<Node<'a>>,
    pub text: &'s str,
    pub truncated: Cell<bool>,
    pub budget: crate::DetectorBudget,
    language: &'s str,
    scopes: Vec<Scope<'a>>,
    node_scopes: HashMap<usize, usize>,
    node_positions: HashMap<usize, usize>,
    writes: HashSet<usize>,
    protected: Vec<(usize, usize)>,
    dangling_strings: Vec<(usize, usize)>,
}
impl<'a, 's> Context<'a, 's> {
    pub fn new(
        root: Node<'a>,
        text: &'s str,
        language: &'s str,
        budget: crate::DetectorBudget,
    ) -> Self {
        let max_nodes = budget.max_ast_nodes;
        let mut result = Self {
            budget,
            nodes: Vec::new(),
            text,
            language,
            truncated: Cell::new(false),
            scopes: vec![Scope {
                parent: None,
                function: true,
                class: false,
                bindings: HashMap::new(),
            }],
            node_scopes: HashMap::new(),
            node_positions: HashMap::new(),
            writes: HashSet::new(),
            protected: Vec::new(),
            dangling_strings: Vec::new(),
        };
        if max_nodes == 0 {
            result.truncated.set(true);
            return result;
        }
        let mut stack = vec![(root, 0, 0)];
        let mut scheduled = 1;
        while let Some((node, parent_scope, depth)) = stack.pop() {
            if depth > 64 || node.named_child_count() > max_nodes {
                result.truncated.set(true);
                continue;
            }
            let kind = node.kind();
            let function = matches!(
                kind,
                "function_declaration"
                    | "function_expression"
                    | "generator_function_declaration"
                    | "generator_function"
                    | "arrow_function"
                    | "function_item"
                    | "closure_expression"
                    | "function_definition"
                    | "lambda"
                    | "method_definition"
                    | "method_signature"
            );
            let block = matches!(
                kind,
                "statement_block"
                    | "for_in_statement"
                    | "for_statement"
                    | "catch_clause"
                    | "class_definition"
                    | "class_body"
                    | "declaration_list"
                    | "impl_item"
                    | "trait_item"
            ) && !(language == "python" && kind == "for_statement")
                || (kind == "block" && language != "python");
            let scope = if function || block {
                let id = result.scopes.len();
                result.scopes.push(Scope {
                    parent: Some(parent_scope),
                    function,
                    class: matches!(kind, "class_definition" | "class_body"),
                    bindings: HashMap::new(),
                });
                id
            } else {
                parent_scope
            };
            result.node_scopes.insert(node.id(), scope);
            result.node_positions.insert(node.id(), node.start_byte());
            result.nodes.push(node);
            if kind.contains("comment")
                || matches!(
                    kind,
                    "string" | "string_literal" | "raw_string_literal" | "char_literal"
                )
            {
                result.protected.push((node.start_byte(), node.end_byte()));
            }
            if kind == "string_start" {
                let mut parent = node.parent();
                let mut end = node.end_byte();
                let mut complete = false;
                for _ in 0..64 {
                    let Some(ancestor) = parent else {
                        break;
                    };
                    if ancestor.kind() == "string" {
                        let mut cursor = ancestor.walk();
                        complete = ancestor
                            .named_children(&mut cursor)
                            .any(|n| n.kind() == "string_end" && !n.is_missing());
                        if complete {
                            break;
                        }
                    }
                    if ancestor.kind() == "ERROR" {
                        end = end.max(ancestor.end_byte());
                    }
                    parent = ancestor.parent();
                }
                if !complete {
                    result.protected.push((node.start_byte(), end));
                    result.dangling_strings.push((node.start_byte(), end));
                }
            }
            let mut cursor = node.walk();
            for child in node.named_children(&mut cursor) {
                if scheduled >= max_nodes {
                    result.truncated.set(true);
                    break;
                }
                stack.push((child, scope, depth + 1));
                scheduled += 1;
            }
        }
        result.nodes.sort_by_key(|n| (n.start_byte(), n.end_byte()));
        result.protected.sort_unstable();
        let mut merged: Vec<(usize, usize)> = Vec::new();
        for (start, end) in result.protected.drain(..) {
            if let Some(last) = merged.last_mut().filter(|last| start <= last.1) {
                last.1 = last.1.max(end);
            } else {
                merged.push((start, end));
            }
        }
        result.protected = merged;
        result
    }
    pub fn bounded_source(&self, node: Node<'_>) -> &'s str {
        let raw = self.source(node);
        let mut boundary = raw.len().min(self.budget.max_ast_match_bytes);
        while !raw.is_char_boundary(boundary) {
            boundary -= 1;
        }
        self.truncated
            .set(self.truncated.get() || boundary < raw.len());
        &raw[..boundary]
    }
    pub fn source(&self, node: Node<'_>) -> &'s str {
        &self.text[node.start_byte()..node.end_byte()]
    }
    pub fn scope(&self, node: Node<'_>) -> usize {
        self.node_scopes.get(&node.id()).copied().unwrap_or(0)
    }
    pub fn parent_scope(&self, node: Node<'_>) -> usize {
        self.scopes[self.scope(node)].parent.unwrap_or(0)
    }
    pub fn add(
        &mut self,
        scope: usize,
        name: &str,
        value: Option<Node<'a>>,
        projection: Option<String>,
        imported: Option<String>,
        id: usize,
    ) {
        let bindings = self.scopes[scope]
            .bindings
            .entry(name.to_string())
            .or_default();
        if bindings.len() == 64 {
            self.truncated.set(true);
            // An omitted later declaration must not expose an older crypto
            // binding. Keep an unresolved sentinel from the point of overflow.
            if let Some(last) = bindings.last_mut() {
                last.value = None;
                last.imported = None;
                last.projection = None;
            }
            return;
        }
        bindings.push(Binding {
            id,
            declared_at: self.node_positions.get(&id).copied().unwrap_or(0),
            value,
            projection,
            imported,
        });
    }
    pub fn function_scope(&self, mut scope: usize) -> usize {
        for _ in 0..128 {
            if self.scopes[scope].function {
                return scope;
            }
            scope = self.scopes[scope].parent.unwrap_or(0);
        }
        self.truncated.set(true);
        0
    }
    pub fn add_hoisted(&mut self, scope: usize, name: &str, imported: Option<String>, id: usize) {
        self.add(scope, name, None, None, imported, id);
        if let Some(binding) = self.scopes[scope]
            .bindings
            .get_mut(name)
            .and_then(|v| v.last_mut())
        {
            binding.declared_at = 0;
        }
    }
    pub fn lookup(&self, node: Node<'_>, name: &str) -> Option<&Binding<'a>> {
        let mut scope = self.scope(node);
        let mut function_seen = false;
        for _ in 0..128 {
            if let Some(bindings) = self.scopes[scope].bindings.get(name).filter(|_| {
                !(self.language == "python" && function_seen && self.scopes[scope].class)
            }) {
                let binding = if self.language == "rust" {
                    bindings
                        .iter()
                        .filter(|b| b.declared_at <= node.start_byte())
                        .max_by_key(|b| (b.declared_at, b.imported.is_none()))
                } else {
                    bindings.last()
                };
                if let Some(binding) = binding {
                    return Some(binding);
                }
            }
            function_seen |= self.scopes[scope].function;
            scope = self.scopes[scope].parent?;
        }
        self.truncated.set(true);
        None
    }
    pub fn unchanged(&self, binding: &Binding<'_>) -> bool {
        !self.writes.contains(&binding.id)
    }
    pub fn mark_write(&mut self, node: Node<'_>, name: &str) {
        if let Some(id) = self.lookup(node, name).map(|b| b.id) {
            self.writes.insert(id);
        }
    }
    pub fn code_at(&self, start: usize) -> bool {
        let i = self.protected.partition_point(|&(left, _)| left <= start);
        i == 0 || self.protected[i - 1].1 <= start
    }
    pub fn unambiguous_string_context(&self, start: usize) -> bool {
        !self
            .dangling_strings
            .iter()
            .any(|&(left, right)| left <= start && start < right)
    }
}
pub(super) fn children(node: Node<'_>) -> Vec<Node<'_>> {
    let mut cursor = node.walk();
    node.named_children(&mut cursor)
        .filter(|n| !n.kind().contains("comment"))
        .collect()
}
pub(super) fn literal<'s>(text: &'s str, node: Node<'_>) -> Option<&'s str> {
    if !matches!(node.kind(), "string" | "string_literal") {
        return None;
    }
    let raw = &text[node.start_byte()..node.end_byte()];
    let quote = *raw.as_bytes().first()?;
    if !matches!(quote, b'\'' | b'"') || raw.as_bytes().last() != Some(&quote) || raw.len() < 2 {
        return None;
    }
    let value = &raw[1..raw.len() - 1];
    (!value.contains(['\\', '\n', '\r', '\0'])).then_some(value)
}
pub(super) fn unwrap_expression(mut node: Node<'_>) -> Node<'_> {
    for _ in 0..MAX_DEPTH {
        if matches!(node.kind(), "await_expression" | "parenthesized_expression") {
            if let Some(child) = node.named_child(0) {
                node = child;
                continue;
            }
        }
        break;
    }
    node
}
pub(super) fn pattern_names<'a>(
    node: Node<'a>,
    text: &str,
    depth: usize,
) -> Vec<(String, Option<String>)> {
    if depth > 64 {
        return vec![];
    }
    match node.kind() {
        "identifier" | "shorthand_property_identifier_pattern" => {
            vec![(text[node.start_byte()..node.end_byte()].into(), None)]
        }
        "pair_pattern" => {
            let Some(key) = node.child_by_field_name("key") else {
                return vec![];
            };
            let Some(value) = node.child_by_field_name("value") else {
                return vec![];
            };
            pattern_names(value, text, depth + 1)
                .into_iter()
                .map(|(name, _)| (name, Some(text[key.start_byte()..key.end_byte()].into())))
                .collect()
        }
        "array_pattern" | "tuple_pattern" => {
            let mut result = Vec::new();
            let mut slot = 0;
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "," {
                    slot += 1;
                }
                if child.is_named() && !child.kind().contains("comment") {
                    result.extend(
                        pattern_names(child, text, depth + 1)
                            .into_iter()
                            .map(|(name, _)| (name, Some(slot.to_string()))),
                    );
                }
            }
            result
        }
        "object_pattern" | "formal_parameters" | "parameters" | "closure_parameters"
        | "tuple_parameter" | "lambda_parameters" | "as_pattern_target" => children(node)
            .into_iter()
            .flat_map(|n| pattern_names(n, text, depth + 1))
            .collect(),
        "required_parameter" | "optional_parameter" => node
            .child_by_field_name("pattern")
            .map_or_else(Vec::new, |n| pattern_names(n, text, depth + 1)),
        "as_pattern" => node
            .child_by_field_name("alias")
            .map_or_else(Vec::new, |n| pattern_names(n, text, depth + 1)),
        "assignment_pattern" => node
            .child_by_field_name("left")
            .map_or_else(Vec::new, |n| pattern_names(n, text, depth + 1)),
        "default_parameter" | "typed_default_parameter" => node
            .child_by_field_name("name")
            .map_or_else(Vec::new, |n| pattern_names(n, text, depth + 1)),
        "rest_pattern" | "typed_parameter" | "list_splat_pattern" | "dictionary_splat_pattern" => {
            children(node)
                .into_iter()
                .take(1)
                .flat_map(|n| pattern_names(n, text, depth + 1))
                .collect()
        }
        "pattern_list" | "list_pattern" => children(node)
            .into_iter()
            .flat_map(|n| pattern_names(n, text, depth + 1))
            .collect(),
        _ => vec![],
    }
}
