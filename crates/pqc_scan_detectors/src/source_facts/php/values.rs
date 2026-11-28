//! Bounded, source-ordered PHP local values and generation-tuple roles.
use super::super::context::children;
use super::super::PartialCause;
use super::names::{Names, CLASS, CONSTANT, FUNCTION};
use super::{class_info, ClassInfo, Key, Role};
use std::collections::HashMap;
use tree_sitter::Node;
#[derive(Clone, Copy)]
pub(super) enum Value {
    Key(Key),
    Pair(Key, Key),
}
#[derive(Clone, Copy)]
struct Binding<'a> {
    end: usize,
    value: Option<Node<'a>>,
    projection: Option<usize>,
    guard: Option<(usize, usize)>,
    hint: Option<Key>,
}
pub(super) struct Values<'a, 's, 'n, 'c> {
    pub names: &'n Names<'a, 's, 'c>,
    bindings: HashMap<(usize, &'s str), Vec<Binding<'a>>>,
    overflows: HashMap<(usize, &'s str), usize>,
    pub(super) binding_limited: bool,
}
impl<'a, 's, 'n, 'c> Values<'a, 's, 'n, 'c> {
    pub fn new(names: &'n Names<'a, 's, 'c>) -> Self {
        let mut result = Self {
            names,
            bindings: HashMap::new(),
            overflows: HashMap::new(),
            binding_limited: false,
        };
        for n in &names.ctx.nodes {
            match n.kind() {
                "assignment_expression"
                | "reference_assignment_expression"
                | "augmented_assignment_expression" => {
                    let (Some(left), Some(right)) = (
                        n.child_by_field_name("left"),
                        n.child_by_field_name("right"),
                    ) else {
                        continue;
                    };
                    let plain = n.kind() == "assignment_expression";
                    if left.kind() == "variable_name" {
                        result.add(left, plain.then_some(right), None, n.end_byte());
                    } else if left.kind() == "subscript_expression"
                        || left.kind() == "member_access_expression"
                    {
                        result.invalidate_target(left, n.end_byte(), 0);
                    } else if left.kind() == "list_literal" {
                        let mut slot = 0;
                        let mut c = left.walk();
                        for child in left.children(&mut c) {
                            if child.kind() == "," {
                                slot += 1;
                            } else if child.kind() == "variable_name" {
                                result.add(child, plain.then_some(right), Some(slot), n.end_byte());
                            }
                        }
                    }
                    if n.kind() == "reference_assignment_expression"
                        && right.kind() == "variable_name"
                    {
                        result.add(right, None, None, n.end_byte());
                        names.mark_partial(PartialCause::OtherSourceUncertainty);
                    }
                }
                "simple_parameter" | "variadic_parameter" | "property_promotion_parameter" => {
                    if let Some(name) = n.child_by_field_name("name") {
                        result.add(name, None, None, n.end_byte());
                    }
                }
                "foreach_statement" => {
                    let body = n.child_by_field_name("body");
                    let mut after_as = false;
                    let mut cursor = n.walk();
                    for child in n.children(&mut cursor) {
                        if child.kind() == "as" {
                            after_as = true;
                            continue;
                        }
                        if after_as && child.is_named() && body.is_none_or(|b| b.id() != child.id())
                        {
                            result.invalidate_target(child, child.end_byte(), 0);
                        }
                    }
                }
                "catch_clause" => {
                    if let Some(name) = n.child_by_field_name("name") {
                        result.add(name, None, None, name.end_byte());
                    }
                }
                "unset_statement" | "update_expression" | "global_declaration" => {
                    for name in children(*n).into_iter() {
                        result.invalidate_target(name, n.end_byte(), 0);
                    }
                    if n.kind() == "global_declaration" {
                        names.mark_partial(PartialCause::OtherSourceUncertainty);
                    }
                }
                _ => {}
            }
        }
        // Calls outside the frozen value semantics may write through references.
        // Explicit OpenSSL output parameters also invalidate an older local value.
        for n in &names.ctx.nodes {
            if n.kind() == "function_call_expression" {
                let function = n
                    .child_by_field_name("function")
                    .and_then(|f| names.resolve(f, FUNCTION));
                if function
                    .as_deref()
                    .is_none_or(|name| !super::known_function(name))
                {
                    for argument in args(*n) {
                        result.invalidate_target(argument, n.end_byte(), 0);
                    }
                } else if matches!(
                    function.as_deref(),
                    Some(
                        "openssl_sign"
                            | "openssl_public_encrypt"
                            | "openssl_private_encrypt"
                            | "openssl_private_decrypt"
                            | "openssl_public_decrypt"
                    )
                ) {
                    let label = if function.as_deref() == Some("openssl_sign") {
                        "signature"
                    } else {
                        "encrypted_data"
                    };
                    // Positional slot remains authoritative for ordinary calls;
                    // decrypted_data is the named output on decrypt variants.
                    let label = if function.as_deref().is_some_and(|s| s.ends_with("decrypt")) {
                        "decrypted_data"
                    } else {
                        label
                    };
                    if let Some(output) = argument(*n, 1, label, names.ctx.text) {
                        result.invalidate_target(output, n.end_byte(), 0);
                    }
                }
            } else if matches!(
                n.kind(),
                "member_call_expression"
                    | "nullsafe_member_call_expression"
                    | "scoped_call_expression"
            ) {
                for argument in args(*n) {
                    result.invalidate_target(argument, n.end_byte(), 0);
                }
                let safe = n
                    .child_by_field_name("name")
                    .and_then(|n| names.token(n, FUNCTION))
                    .is_some_and(|s| {
                        matches!(
                            s.as_str(),
                            "sign" | "verify" | "encapsulate" | "decapsulate"
                        )
                    });
                let opaque_xml = n
                    .child_by_field_name("object")
                    .is_some_and(|object| result.is_opaque_xml_receiver(object));
                if !safe && !opaque_xml {
                    if let Some(object) = n.child_by_field_name("object") {
                        result.invalidate_target(object, n.end_byte(), 0);
                    }
                }
            }
        }
        for bindings in result.bindings.values_mut() {
            bindings.sort_by_key(|b| b.end);
        }
        result
    }
    fn invalidate_target(&mut self, node: Node<'a>, end: usize, depth: usize) {
        if depth >= 64 {
            self.names.ctx.limit("php_write_depth", 64, 64);
            return;
        }
        if !self.names.charge() {
            return;
        }
        match node.kind() {
            "variable_name" => self.add(node, None, None, end),
            "subscript_expression" | "parenthesized_expression" | "by_ref" => {
                if let Some(base) = node.named_child(0) {
                    self.invalidate_target(base, end, depth + 1);
                }
            }
            "member_access_expression" => {
                if let Some(base) = node.child_by_field_name("object") {
                    self.invalidate_target(base, end, depth + 1);
                }
            }
            "list_literal" | "pair" => {
                for child in children(node) {
                    self.invalidate_target(child, end, depth + 1);
                }
            }
            _ => {}
        }
    }
    fn add(
        &mut self,
        name: Node<'a>,
        value: Option<Node<'a>>,
        projection: Option<usize>,
        end: usize,
    ) {
        if !self.names.charge() {
            return;
        }
        let raw = self.names.ctx.source(name);
        if raw.len() > 256 {
            self.names.ctx.limit("php_variable_bytes", 256, 256);
            return;
        }
        let function = self.names.ctx.function_scope(self.names.ctx.scope(name));
        let key = (function, raw);
        let hint = self.hint(name);
        let list = self.bindings.entry(key).or_default();
        if list.len() >= 64 {
            self.overflows
                .entry(key)
                .and_modify(|at| *at = (*at).min(end))
                .or_insert(end);
            // The cutoff invalidates this variable's later values. Other
            // variables/functions still have independent, bounded facts.
            self.binding_limited = true;
            return;
        }
        // The mutation pass also resolves receivers before construction ends.
        // Keep the lookup invariant even when it inserts an earlier call write.
        let at = list.partition_point(|binding| binding.end <= end);
        list.insert(
            at,
            Binding {
                end,
                value,
                projection,
                guard: guard(self.names, name),
                hint,
            },
        );
    }
    fn binding(&self, node: Node<'a>) -> Option<&Binding<'a>> {
        let key = (
            self.names.ctx.function_scope(self.names.ctx.scope(node)),
            self.names.ctx.source(node),
        );
        if self
            .overflows
            .get(&key)
            .is_some_and(|at| *at <= node.start_byte())
        {
            return None;
        }
        let values = self.bindings.get(&key)?;
        let b = values.get(
            values
                .partition_point(|b| b.end <= node.start_byte())
                .checked_sub(1)?,
        )?;
        if b.guard
            .is_some_and(|(start, end)| node.start_byte() < start || node.end_byte() > end)
        {
            self.names
                .mark_partial(PartialCause::OtherSourceUncertainty);
            return None;
        }
        Some(b)
    }
    fn hint(&self, name: Node<'a>) -> Option<Key> {
        let parent = name.parent()?;
        if !matches!(
            parent.kind(),
            "simple_parameter" | "property_promotion_parameter"
        ) {
            return None;
        }
        let mut node = parent.child_by_field_name("type")?;
        for _ in 0..8 {
            if matches!(node.kind(), "name" | "qualified_name" | "relative_name") {
                return match class_info(&self.names.resolve(node, CLASS)?)? {
                    ClassInfo::Key(key) => Some(key),
                    _ => None,
                };
            }
            let next = children(node);
            if next.len() != 1 {
                return None;
            }
            node = next[0];
        }
        self.names.ctx.limit("php_type_hint_depth", 8, 8);
        None
    }
    pub fn expression(&self, node: Node<'a>, depth: usize) -> Option<Node<'a>> {
        if !self.step(depth) {
            return None;
        }
        if node.kind() == "parenthesized_expression" {
            return self.expression(node.named_child(0)?, depth + 1);
        }
        if node.kind() == "variable_name" {
            let b = self.binding(node)?;
            if b.projection.is_some() {
                return None;
            }
            let rhs = b.value?;
            return self.expression(rhs, depth + usize::from(rhs.kind() == "variable_name"));
        }
        Some(node)
    }
    fn step(&self, depth: usize) -> bool {
        if depth > 8 {
            self.names.ctx.limit("source_alias_depth", 8, 8);
            false
        } else {
            self.names.charge()
        }
    }
    pub fn value(&self, node: Node<'a>, depth: usize) -> Option<Value> {
        if !self.step(depth) {
            return None;
        }
        match node.kind() {
            "parenthesized_expression" => self.value(node.named_child(0)?, depth + 1),
            "variable_name" => {
                let b = self.binding(node)?;
                let Some(rhs) = b.value else {
                    return b.hint.map(Value::Key);
                };
                let value = self.value(rhs, depth + usize::from(rhs.kind() == "variable_name"))?;
                match (b.projection, value) {
                    (None, value) => Some(value),
                    (Some(0), Value::Pair(a, _)) => Some(Value::Key(a)),
                    (Some(1), Value::Pair(_, b)) => Some(Value::Key(b)),
                    _ => None,
                }
            }
            "subscript_expression" => {
                let parts = children(node);
                if parts.len() != 2 {
                    return None;
                }
                let pair = self.value(parts[0], depth + 1)?;
                let index = self.names.ctx.source(parts[1]);
                match (pair, index) {
                    (Value::Pair(a, _), "0") => Some(Value::Key(a)),
                    (Value::Pair(_, b), "1") => Some(Value::Key(b)),
                    _ => None,
                }
            }
            "function_call_expression" => {
                let name = self
                    .names
                    .resolve(node.child_by_field_name("function")?, FUNCTION)?;
                if name == "openssl_pkey_new" {
                    let key =
                        self.openssl_type(argument(node, 0, "options", self.names.ctx.text)?, 0)?;
                    Some(Value::Key(Key {
                        algorithm: key,
                        role: Role::OpenSsl,
                    }))
                } else {
                    None
                }
            }
            "scoped_call_expression" => {
                let class = self
                    .names
                    .resolve(node.child_by_field_name("scope")?, CLASS)?;
                let method = self
                    .names
                    .token(node.child_by_field_name("name")?, FUNCTION)?;
                match class_info(&class)? {
                    ClassInfo::Factory(algorithm) if method == "generatekeypair" => {
                        Some(pair(algorithm))
                    }
                    ClassInfo::Compat => {
                        let (algorithm, operation) = super::compat_method(&method)?;
                        (operation == "keygen").then(|| pair(algorithm))
                    }
                    ClassInfo::Key(key) if method == "frombytes" => Some(Value::Key(key)),
                    _ => None,
                }
            }
            "object_creation_expression" => {
                let class = children(node)
                    .into_iter()
                    .find(|n| matches!(n.kind(), "name" | "qualified_name" | "relative_name"))?;
                match class_info(&self.names.resolve(class, CLASS)?)? {
                    ClassInfo::XmlSignature => Some(Value::Key(Key {
                        algorithm: "",
                        role: Role::XmlSignature,
                    })),
                    ClassInfo::Reference(Some("SLH-DSA")) => Some(Value::Key(Key {
                        algorithm: "SLH-DSA",
                        role: Role::Reference,
                    })),
                    _ => None,
                }
            }
            _ => None,
        }
    }
    pub fn key(&self, node: Node<'a>) -> Option<Key> {
        let value = self.value(node, 0);
        if value.is_none() && node.kind() == "variable_name" {
            let key = (
                self.names.ctx.function_scope(self.names.ctx.scope(node)),
                self.names.ctx.source(node),
            );
            if self
                .bindings
                .get(&key)
                .is_some_and(|v| v.iter().filter(|b| b.end <= node.start_byte()).count() > 1)
            {
                self.names
                    .mark_partial(PartialCause::OtherSourceUncertainty);
            }
        }
        match value? {
            Value::Key(k) => Some(k),
            _ => None,
        }
    }
    fn is_opaque_xml_receiver(&self, node: Node<'a>) -> bool {
        // This is a speculative mutation-policy probe, including for unrelated
        // methods. Only actual crypto lookups may report unresolved bindings.
        // Resolution remains fail-closed; work budgets and context limits are
        // still charged and are never restored by this diagnostic suppression.
        let partial = self.names.partial_causes();
        let value = self.value(node, 0);
        self.names.restore_partial_causes(partial);
        matches!(value, Some(Value::Key(key)) if key.role == Role::XmlSignature)
    }
    /// A nearby canonical key-type assertion is a source reference, not proof
    /// that every control-flow path supplies that type at the later operation.
    pub fn guarded_signer_reference(&self, object: Node<'a>, at: usize) -> bool {
        if object.kind() != "variable_name" {
            return false;
        }
        let scope = self.names.ctx.function_scope(self.names.ctx.scope(object));
        let raw = self.names.ctx.source(object);
        let mut checked = None;
        for node in &self.names.ctx.nodes {
            if !self.names.charge() {
                return false;
            }
            if node.end_byte() > at
                || self.names.ctx.function_scope(self.names.ctx.scope(*node)) != scope
            {
                continue;
            }
            if node.kind() == "binary_expression"
                && node
                    .child_by_field_name("operator")
                    .is_some_and(|n| self.names.ctx.source(n) == "instanceof")
                && node
                    .child_by_field_name("left")
                    .is_some_and(|n| self.names.ctx.source(n) == raw)
                && node
                    .child_by_field_name("right")
                    .and_then(|n| self.names.resolve(n, CLASS))
                    .is_some_and(|s| {
                        matches!(
                            s.as_str(),
                            "phpseclib3\\crypt\\rsa\\privatekey"
                                | "phpseclib3\\crypt\\ec\\privatekey"
                        )
                    })
            {
                checked = Some(node.end_byte());
            }
        }
        let Some(checked) = checked else {
            return false;
        };
        for node in &self.names.ctx.nodes {
            if !self.names.charge() {
                return false;
            }
            if node.start_byte() < checked
                || node.end_byte() > at
                || self.names.ctx.function_scope(self.names.ctx.scope(*node)) != scope
            {
                continue;
            }
            if matches!(
                node.kind(),
                "assignment_expression"
                    | "reference_assignment_expression"
                    | "augmented_assignment_expression"
            ) && node
                .child_by_field_name("left")
                .is_some_and(|n| self.names.ctx.source(n) == raw)
            {
                return false;
            }
            if matches!(
                node.kind(),
                "function_call_expression" | "member_call_expression" | "scoped_call_expression"
            ) && args(*node).iter().any(|n| self.names.ctx.source(*n) == raw)
            {
                return false;
            }
        }
        true
    }
    fn openssl_type(&self, node: Node<'a>, depth: usize) -> Option<&'static str> {
        let array = self.expression(node, depth)?;
        if array.kind() != "array_creation_expression" {
            return None;
        }
        let mut result = None;
        let mut found = false;
        for element in children(array) {
            if !self.names.charge() {
                return None;
            }
            let values = children(element);
            if values.len() != 2 {
                return None;
            }
            // Spread/computed keys can overwrite a known selector. Do not skip
            // them and reuse an earlier private_key_type entry.
            let key = literal(self.names.ctx.source(values[0]))?;
            if key == "private_key_type" {
                if found {
                    return None;
                }
                found = true;
                let value = self.expression(values[1], 0)?;
                let name = self.names.resolve(value, CONSTANT)?;
                result = match name.as_str() {
                    "OPENSSL_KEYTYPE_RSA" => Some("RSA"),
                    "OPENSSL_KEYTYPE_EC" => Some("EC"),
                    "OPENSSL_KEYTYPE_DH" => Some("DH"),
                    _ => None,
                };
            }
        }
        result
    }
}
pub(super) fn pair(algorithm: &'static str) -> Value {
    let (a, b) = if algorithm.starts_with("ML-DSA") {
        (Role::Sign, Role::Verify)
    } else {
        (Role::Decap, Role::Encap)
    };
    Value::Pair(Key { algorithm, role: a }, Key { algorithm, role: b })
}
pub(super) fn args(node: Node<'_>) -> Vec<Node<'_>> {
    let arguments = node
        .child_by_field_name("arguments")
        .or_else(|| children(node).into_iter().find(|n| n.kind() == "arguments"));
    arguments.map_or_else(Vec::new, |arguments| {
        children(arguments)
            .into_iter()
            .filter_map(|a| {
                let name = a.child_by_field_name("name");
                children(a).into_iter().find(|n| {
                    name.is_none_or(|label| n.id() != label.id())
                        && n.kind() != "reference_modifier"
                })
            })
            .collect()
    })
}
pub(super) fn literal(raw: &str) -> Option<&str> {
    let q = *raw.as_bytes().first()?;
    if raw.len() < 2 || !matches!(q, b'\'' | b'"') || raw.as_bytes().last() != Some(&q) {
        return None;
    }
    let v = &raw[1..raw.len() - 1];
    (v.len() <= 64 && !v.contains(['\\', '$', '\0', '\n', '\r'])).then_some(v)
}
fn guard(names: &Names<'_, '_, '_>, mut node: Node<'_>) -> Option<(usize, usize)> {
    for _ in 0..64 {
        let p = node.parent()?;
        if matches!(
            p.kind(),
            "function_definition"
                | "method_declaration"
                | "anonymous_function"
                | "arrow_function"
                | "program"
        ) {
            return None;
        }
        if matches!(
            p.kind(),
            "if_statement"
                | "else_clause"
                | "else_if_clause"
                | "while_statement"
                | "for_statement"
                | "foreach_statement"
                | "switch_statement"
                | "try_statement"
                | "catch_clause"
                | "conditional_expression"
        ) {
            return Some((node.start_byte(), node.end_byte()));
        }
        node = p;
    }
    names.ctx.limit("php_guard_depth", 64, 64);
    None
}

pub(super) fn argument<'a>(
    node: Node<'a>,
    index: usize,
    label: &str,
    text: &str,
) -> Option<Node<'a>> {
    let arguments = node.child_by_field_name("arguments")?;
    let mut positional = Vec::new();
    let mut named = None;
    let mut saw_named = false;
    for a in children(arguments) {
        let name = a.child_by_field_name("name");
        let value = children(a).into_iter().find(|n| {
            name.is_none_or(|key| n.id() != key.id()) && n.kind() != "reference_modifier"
        })?;
        if value.kind() == "variadic_unpacking" {
            return None;
        }
        if let Some(name) = name {
            saw_named = true;
            if &text[name.byte_range()] == label {
                if named.is_some() {
                    return None;
                }
                named = Some(value);
            }
        } else if !saw_named {
            positional.push(value);
        }
    }
    named.or_else(|| positional.get(index).copied())
}
