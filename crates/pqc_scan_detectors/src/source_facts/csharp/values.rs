//! Local lexical values, immutable parameter symbols and conservative write barriers.
use super::super::context::children;
use super::names::Names;
use super::{class_info, compatible, factory_method, parameter, unique_kind, Key, Kind};
use std::collections::HashMap;
use tree_sitter::Node;
#[derive(Clone)]
struct Binding<'a> {
    end: usize,
    value: Option<Node<'a>>,
    hint: Option<Kind>,
    guard: Option<(usize, usize)>,
    known: Option<Key>,
}
pub(super) struct Values<'a, 's, 'n, 'c> {
    names: &'n Names<'a, 's, 'c>,
    bindings: HashMap<(usize, String), Vec<Binding<'a>>>,
    overflows: HashMap<(usize, String), usize>,
    pub(super) binding_limited: bool,
    barriers: HashMap<usize, Vec<usize>>,
    has_error: bool,
}
impl<'a, 's, 'n, 'c> Values<'a, 's, 'n, 'c> {
    pub fn new(names: &'n Names<'a, 's, 'c>) -> Self {
        let mut out = Self {
            names,
            bindings: HashMap::new(),
            overflows: HashMap::new(),
            binding_limited: false,
            barriers: HashMap::new(),
            has_error: names.ctx.nodes.iter().any(|n| n.kind() == "ERROR"),
        };
        for n in &names.ctx.nodes {
            if !names.charge() {
                return out;
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
                    out.add(names.ctx.scope(*n), name, None, None, n.end_byte(), None);
                }
            }
            match n.kind() {
                "implicit_parameter" => {
                    out.add(names.ctx.scope(*n), *n, None, None, n.end_byte(), None)
                }
                "variable_declarator" => {
                    if let Some(name) = n.child_by_field_name("name") {
                        let value = children(*n).into_iter().find(|c| c.id() != name.id());
                        let field = n
                            .parent()
                            .and_then(|p| p.parent())
                            .is_some_and(|p| p.kind() == "field_declaration");
                        let hint = if !field {
                            n.parent()
                                .and_then(|p| p.child_by_field_name("type"))
                                .and_then(|t| out.hint(t))
                        } else {
                            None
                        };
                        out.add(
                            names.ctx.scope(*n),
                            name,
                            if field { None } else { value },
                            hint,
                            if field { 0 } else { n.end_byte() },
                            None,
                        );
                    }
                }
                "parameter" | "catch_declaration" | "declaration_expression" => {
                    if let Some(name) = n.child_by_field_name("name") {
                        let hint = if n.kind() == "parameter" {
                            n.child_by_field_name("type").and_then(|t| out.hint(t))
                        } else {
                            None
                        };
                        out.add(names.ctx.scope(*n), name, None, hint, n.end_byte(), None);
                    }
                }
                "foreach_statement" => {
                    if let Some(name) = n.child_by_field_name("left") {
                        out.invalidate(name, name.end_byte(), 0);
                    }
                }
                _ => {}
            }
        }
        // Collect source writes after declarations so a nested block writes the
        // nearest existing local rather than inventing a new inner binding.
        for n in &names.ctx.nodes {
            if !names.charge() {
                return out;
            }
            match n.kind() {
                "assignment_expression" => {
                    if let (Some(left), Some(right)) = (
                        n.child_by_field_name("left"),
                        n.child_by_field_name("right"),
                    ) {
                        let plain = n
                            .child_by_field_name("operator")
                            .is_some_and(|n| names.ctx.source(n) == "=");
                        if left.kind() == "identifier" {
                            let scope = out.declaration_scope(left);
                            out.add(
                                scope,
                                left,
                                plain.then_some(right),
                                None,
                                n.end_byte(),
                                None,
                            );
                        } else {
                            out.invalidate(left, n.end_byte(), 0);
                        }
                    }
                }
                "postfix_unary_expression" | "prefix_unary_expression" => {
                    let raw = names.ctx.source(*n);
                    if raw.contains("++") || raw.contains("--") {
                        if let Some(child) = n.named_child(0) {
                            out.invalidate(child, n.end_byte(), 0);
                        }
                    }
                }
                _ => {}
            }
        }
        for n in &names.ctx.nodes {
            if !names.charge() {
                return out;
            }
            if n.kind() != "invocation_expression" {
                continue;
            }
            let Some(function) = n.child_by_field_name("function") else {
                continue;
            };
            let mut initialized = None;
            let mut known = false;
            let mut mutating = false;
            if function.kind() == "member_access_expression" {
                if let (Some(object), Some(method)) = (
                    function.child_by_field_name("expression"),
                    function.child_by_field_name("name"),
                ) {
                    let method = names.path(method).unwrap_or_default();
                    if let Some(mut key) = out.key(object) {
                        known = super::operation(key.kind, &method);
                        if method == "Init"
                            && matches!(
                                key.kind,
                                Kind::Generator(_)
                                    | Kind::Signer(_)
                                    | Kind::Engine(_)
                                    | Kind::Encap
                                    | Kind::Decap
                            )
                        {
                            if matches!(key.kind, Kind::Generator(_)) {
                                let selected = argument(*n, 0, "parameters", names.ctx.text)
                                    .and_then(|n| out.selector_in(n, 0))
                                    .filter(|s| compatible(key.kind.family().unwrap_or(""), s));
                                key.algorithm = selected.unwrap_or_else(|| {
                                    names.partial.set(true);
                                    key.kind.family().unwrap_or("").into()
                                });
                            }
                            // Signer/KEM constructor parameters are immutable;
                            // Init selects a key/mode, not a different suite.
                            initialized = Some((object, key));
                            known = true;
                            mutating = true;
                        }
                        if !known {
                            out.invalidate(object, n.end_byte(), 0);
                        }
                    }
                }
            }
            known |= out.factory(*n, 0).is_some();
            if !known || mutating {
                let scope = names.ctx.function_scope(names.ctx.scope(*n));
                let list = out.barriers.entry(scope).or_default();
                let at = list.partition_point(|v| *v <= n.end_byte());
                list.insert(at, n.end_byte());
            }
            for argument in arguments(*n) {
                let raw = argument.parent().map(|p| names.ctx.source(p)).unwrap_or("");
                if raw.split_whitespace().any(|s| matches!(s, "ref" | "out")) || !known {
                    out.invalidate(argument, n.end_byte(), 0);
                }
            }
            if let Some((object, key)) = initialized {
                if object.kind() == "identifier" {
                    let scope = out.declaration_scope(object);
                    out.add(scope, object, None, None, n.end_byte(), Some(key));
                }
            }
        }
        out
    }
    fn hint(&self, mut node: Node<'a>) -> Option<Kind> {
        for _ in 0..8 {
            if let Some(kind) = unique_kind(self.names, node, true) {
                return Some(kind);
            }
            if node.kind() != "nullable_type" {
                return None;
            }
            node = node.named_child(0)?;
        }
        self.names.ctx.limit("csharp_type_hint_depth", 8, 8);
        None
    }
    fn add(
        &mut self,
        scope: usize,
        name: Node<'a>,
        value: Option<Node<'a>>,
        hint: Option<Kind>,
        end: usize,
        known: Option<Key>,
    ) {
        if !self.names.charge() {
            return;
        }
        let Some(raw) = self.names.path(name) else {
            return;
        };
        if raw.len() > 256 {
            self.names.ctx.limit("csharp_variable_bytes", 256, 256);
            return;
        }
        let key = (scope, raw);
        let list = self.bindings.entry(key.clone()).or_default();
        if list.len() >= 64 {
            self.overflows
                .entry(key)
                .and_modify(|n| *n = (*n).min(end))
                .or_insert(end);
            // Saturation invalidates this variable after its cutoff. Other
            // functions and variables still have independent bounded facts.
            self.binding_limited = true;
            return;
        }
        let position = list.partition_point(|b| b.end <= end);
        list.insert(
            position,
            Binding {
                end,
                value,
                hint,
                guard: guard(self.names, name),
                known,
            },
        );
    }
    fn declaration_scope(&self, name: Node<'a>) -> usize {
        let Some(raw) = self.names.path(name) else {
            return self.names.ctx.scope(name);
        };
        let mut current = Some(self.names.ctx.scope(name));
        let boundary = self.names.ctx.function_scope(self.names.ctx.scope(name));
        for _ in 0..64 {
            let Some(s) = current else {
                break;
            };
            if self.bindings.contains_key(&(s, raw.clone())) {
                return s;
            }
            if s == boundary {
                return self.names.ctx.scope(name);
            }
            current = self.names.ctx.scope_parent(s);
        }
        if current.is_some() {
            self.names.ctx.limit("csharp_scope_depth", 64, 64);
        }
        self.names.ctx.scope(name)
    }
    fn invalidate(&mut self, node: Node<'a>, end: usize, depth: usize) {
        if depth >= 64 {
            self.names.ctx.limit("csharp_write_depth", 64, 64);
            return;
        }
        if !self.names.charge() {
            return;
        }
        match node.kind() {
            "identifier" => {
                let scope = self.declaration_scope(node);
                self.add(scope, node, None, None, end, None);
            }
            "member_access_expression" => {
                if let Some(base) = node.child_by_field_name("expression") {
                    self.invalidate(base, end, depth + 1);
                }
            }
            "element_access_expression" | "parenthesized_expression" => {
                if let Some(base) = node.named_child(0) {
                    self.invalidate(base, end, depth + 1);
                }
            }
            "declaration_expression" => {
                if let Some(name) = node.child_by_field_name("name") {
                    self.invalidate(name, end, depth + 1);
                }
            }
            "argument" => {
                if let Some(base) = node.named_child(0) {
                    self.invalidate(base, end, depth + 1);
                }
            }
            "tuple_expression" | "tuple_pattern" => {
                for child in children(node) {
                    self.invalidate(child, end, depth + 1);
                }
            }
            _ => {}
        }
    }
    fn binding(&self, node: Node<'a>) -> Option<&Binding<'a>> {
        if self.has_error {
            return None;
        }
        let raw = self.names.path(node)?;
        let mut current = Some(self.names.ctx.scope(node));
        let boundary = self.names.ctx.function_scope(self.names.ctx.scope(node));
        for _ in 0..64 {
            let s = current?;
            let key = (s, raw.clone());
            if self
                .overflows
                .get(&key)
                .is_some_and(|at| *at <= node.start_byte())
            {
                return None;
            }
            if let Some(values) = self.bindings.get(&key) {
                let binding = values.get(
                    values
                        .partition_point(|v| v.end <= node.start_byte())
                        .checked_sub(1)?,
                )?;
                if binding
                    .guard
                    .is_some_and(|(a, b)| node.start_byte() < a || node.end_byte() > b)
                {
                    self.names.partial.set(true);
                    return None;
                }
                let function = self.names.ctx.function_scope(self.names.ctx.scope(node));
                if self.barriers.get(&function).is_some_and(|v| {
                    let last = v.partition_point(|at| *at <= node.start_byte());
                    last > 0 && binding.end < v[last - 1]
                }) {
                    self.names.partial.set(true);
                    return None;
                }
                return Some(binding);
            }
            if s == boundary {
                return None;
            }
            current = self.names.ctx.scope_parent(s);
        }
        self.names.ctx.limit("csharp_scope_depth", 64, 64);
        None
    }
    fn step(&self, depth: usize) -> bool {
        if depth > 8 {
            self.names.ctx.limit("source_alias_depth", 8, 8);
            false
        } else {
            self.names.charge()
        }
    }
    pub fn key(&self, node: Node<'a>) -> Option<Key> {
        self.value(node, 0)
            .or_else(|| self.names.opaque_receiver(node).and_then(Kind::key))
    }
    fn value(&self, node: Node<'a>, depth: usize) -> Option<Key> {
        if !self.step(depth) {
            return None;
        }
        match node.kind() {
            "identifier" => {
                let b = self.binding(node)?;
                if let Some(key) = &b.known {
                    return Some(key.clone());
                }
                if let Some(rhs) = b.value {
                    let value = self.value(rhs, depth + usize::from(rhs.kind() == "identifier"));
                    if let (Some(hint), Some(value)) = (b.hint, value.as_ref()) {
                        if hint.family() != value.kind.family() {
                            self.names.partial.set(true);
                            return None;
                        }
                    }
                    return value.or_else(|| b.hint.and_then(Kind::key));
                }
                b.hint?.key()
            }
            "parenthesized_expression" => self.value(node.named_child(0)?, depth + 1),
            "object_creation_expression" => {
                let kind = unique_kind(self.names, node.child_by_field_name("type")?, true)?;
                let mut key = kind.key()?;
                if matches!(
                    kind,
                    Kind::Encap | Kind::Decap | Kind::Signer("ML-DSA" | "SLH-DSA")
                ) {
                    if let Some(selected) = argument(node, 0, "parameters", self.names.ctx.text)
                        .and_then(|n| self.selector(n, 0))
                        .filter(|s| compatible(kind.family().unwrap_or(""), s))
                    {
                        key.algorithm = selected;
                    } else {
                        self.names.partial.set(true);
                    }
                }
                Some(key)
            }
            "invocation_expression" => self.factory(node, depth),
            _ => None,
        }
    }
    pub fn factory(&self, node: Node<'a>, depth: usize) -> Option<Key> {
        if !self.step(depth) {
            return None;
        }
        let function = node.child_by_field_name("function")?;
        let mut found = vec![];
        for full in self.names.static_candidates(function) {
            let Some((class, method)) = full.rsplit_once('.') else {
                continue;
            };
            let Some(kind) = class_info(class) else {
                continue;
            };
            if !factory_method(kind, method) {
                continue;
            }
            let mut key = kind.key()?;
            if matches!(kind, Kind::Pqc(_)) {
                let selected = argument(node, 0, "algorithm", self.names.ctx.text)
                    .and_then(|n| self.selector(n, 0))
                    .filter(|s| compatible(kind.family().unwrap_or(""), s));
                if let Some(selected) = selected {
                    key.algorithm = selected;
                } else if method == "GenerateKey" {
                    self.names.partial.set(true);
                }
            }
            found.push(key);
        }
        if found.len() == 1 {
            found.pop()
        } else {
            if found.len() > 1 {
                self.names.partial.set(true);
            }
            None
        }
    }
    pub fn selector(&self, node: Node<'a>, depth: usize) -> Option<String> {
        if !self.step(depth) {
            return None;
        }
        if node.kind() == "identifier" {
            if let Some(binding) = self.binding(node) {
                if let Some(rhs) = binding.value {
                    return self.selector(rhs, depth + usize::from(rhs.kind() == "identifier"));
                }
                return None;
            }
        }
        if node.kind() == "parenthesized_expression" {
            return self.selector(node.named_child(0)?, depth + 1);
        }
        let mut found: Vec<_> = self
            .names
            .resolve(node, false)
            .iter()
            .filter_map(|s| parameter(s))
            .collect();
        found.sort();
        found.dedup();
        if found.len() == 1 {
            found.pop()
        } else {
            None
        }
    }
    fn selector_in(&self, node: Node<'a>, depth: usize) -> Option<String> {
        if !self.step(depth) {
            return None;
        }
        if node.kind() == "identifier" {
            let binding = self.binding(node)?;
            let rhs = binding.value?;
            return self.selector_in(rhs, depth + usize::from(rhs.kind() == "identifier"));
        }
        if node.kind() != "object_creation_expression" {
            return None;
        }
        let typ = node.child_by_field_name("type")?;
        let families: Vec<_> = self
            .names
            .resolve(typ, true)
            .iter()
            .filter_map(
                |s| match s.strip_prefix("Org.BouncyCastle.Crypto.Parameters.")? {
                    "MLKemKeyGenerationParameters" => Some("ML-KEM"),
                    "MLDsaKeyGenerationParameters" => Some("ML-DSA"),
                    "SlhDsaKeyGenerationParameters" => Some("SLH-DSA"),
                    _ => None,
                },
            )
            .collect();
        if families.len() != 1 {
            return None;
        }
        let selected = self.selector(argument(node, 1, "parameters", self.names.ctx.text)?, 0)?;
        compatible(families[0], &selected).then_some(selected)
    }
}
fn argument<'a>(node: Node<'a>, index: usize, label: &str, text: &str) -> Option<Node<'a>> {
    let args = node.child_by_field_name("arguments")?;
    let mut positional = 0;
    let mut found = None;
    for arg in children(args)
        .into_iter()
        .filter(|n| n.kind() == "argument")
    {
        let name = arg.child_by_field_name("name");
        let value = children(arg)
            .into_iter()
            .find(|n| name.is_none_or(|a| n.id() != a.id()))?;
        let selects = if let Some(name) = name {
            &text[name.byte_range()] == label
        } else {
            let select = positional == index;
            positional += 1;
            select
        };
        if selects {
            if found.is_some() {
                return None;
            }
            found = Some(value);
        }
    }
    found
}
pub(super) fn arguments(node: Node<'_>) -> Vec<Node<'_>> {
    node.child_by_field_name("arguments")
        .map_or_else(Vec::new, |args| {
            children(args)
                .into_iter()
                .filter(|n| n.kind() == "argument")
                .filter_map(|n| {
                    let name = n.child_by_field_name("name");
                    children(n)
                        .into_iter()
                        .find(|c| name.is_none_or(|name| c.id() != name.id()))
                })
                .collect()
        })
}
fn guard(names: &Names<'_, '_, '_>, mut node: Node<'_>) -> Option<(usize, usize)> {
    for _ in 0..64 {
        let p = node.parent()?;
        if matches!(
            p.kind(),
            "method_declaration"
                | "constructor_declaration"
                | "local_function_statement"
                | "lambda_expression"
                | "anonymous_method_expression"
                | "compilation_unit"
        ) {
            return None;
        }
        if matches!(
            p.kind(),
            "if_statement"
                | "for_statement"
                | "foreach_statement"
                | "while_statement"
                | "do_statement"
                | "switch_section"
                | "catch_clause"
                | "preproc_if"
                | "preproc_else"
        ) {
            return Some((p.start_byte(), p.end_byte()));
        }
        node = p;
    }
    names.ctx.limit("csharp_ancestor_depth", 64, 64);
    None
}
