use super::{
    args, conditional, context_constructor, function_name, known, low_level, raw_key_constructor,
    selected_literal, SelectorKind,
};
use crate::source_facts::context::{children, Context};
use std::{
    cell::Cell,
    collections::{BTreeSet, HashMap, HashSet},
};
use tree_sitter::Node;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Role {
    Context,
    Key,
    LowLevelKey,
    OqsKem,
    OqsSig,
    CppKem,
    CppSig,
}
#[derive(Clone, Debug)]
pub(super) struct Key {
    pub role: Role,
    pub algorithm: String,
}
#[derive(Clone, Copy)]
enum Value<'a> {
    Node(Node<'a>),
    Generated(Node<'a>),
    ContextFromKey(Node<'a>),
    Cpp(Node<'a>, Role),
    Unknown,
}
#[derive(Clone, Copy)]
struct Binding<'a> {
    at: usize,
    value: Value<'a>,
}
pub(super) struct Values<'a, 's, 'c> {
    ctx: &'c Context<'a, 's>,
    bindings: HashMap<(usize, String), Vec<Binding<'a>>>,
    overflows: HashMap<(usize, String), usize>,
    globals: HashSet<String>,
    symbols: HashMap<usize, HashMap<String, usize>>,
    macros: HashSet<String>,
    oqs_shadow: bool,
    botan_shadow: bool,
    barriers: HashMap<usize, BTreeSet<usize>>,
    scalar_barriers: HashMap<usize, BTreeSet<usize>>,
    work: Cell<usize>,
    names_incomplete: Cell<bool>,
}
impl<'a, 's, 'c> Values<'a, 's, 'c> {
    pub fn new(ctx: &'c Context<'a, 's>) -> Self {
        let mut out = Self {
            ctx,
            bindings: HashMap::new(),
            overflows: HashMap::new(),
            globals: HashSet::new(),
            symbols: HashMap::new(),
            macros: HashSet::new(),
            oqs_shadow: false,
            botan_shadow: false,
            barriers: HashMap::new(),
            scalar_barriers: HashMap::new(),
            work: Cell::new(0),
            names_incomplete: Cell::new(false),
        };
        for node in &ctx.nodes {
            if !out.charge() {
                break;
            }
            if matches!(
                node.kind(),
                "namespace_definition"
                    | "namespace_alias_definition"
                    | "class_specifier"
                    | "struct_specifier"
            ) && node
                .child_by_field_name("name")
                .is_some_and(|n| ctx.bounded_source(n) == "Botan")
            {
                out.botan_shadow = true;
            }
            if matches!(
                node.kind(),
                "preproc_def" | "preproc_function_def" | "preproc_undef"
            ) {
                if let Some(n) = node.child_by_field_name("name").or_else(|| {
                    children(*node)
                        .into_iter()
                        .find(|n| n.kind() == "identifier")
                }) {
                    out.macros.insert(ctx.bounded_source(n).into());
                }
            }
            if matches!(
                node.kind(),
                "namespace_definition"
                    | "namespace_alias_definition"
                    | "class_specifier"
                    | "struct_specifier"
            ) && node
                .child_by_field_name("name")
                .is_some_and(|n| ctx.bounded_source(n) == "oqs")
            {
                out.oqs_shadow = true;
            }
            if matches!(
                node.kind(),
                "type_parameter_declaration"
                    | "optional_type_parameter_declaration"
                    | "variadic_type_parameter_declaration"
            ) {
                if let Some(name) = node.child_by_field_name("name").or_else(|| {
                    children(*node)
                        .into_iter()
                        .find(|n| n.kind() == "type_identifier")
                }) {
                    out.add_symbol(ctx.scope(*node), name, node.end_byte());
                }
            }
            if node.kind().ends_with("parameter_declaration")
                && node
                    .parent()
                    .is_some_and(|p| p.kind() == "template_parameter_list")
            {
                if let Some(name) = node
                    .child_by_field_name("declarator")
                    .and_then(|n| out.declarator(n))
                {
                    out.add_symbol(ctx.scope(*node), name, node.end_byte());
                }
            }
            if node.kind() == "using_declaration" {
                if let Some(qualified) = children(*node)
                    .into_iter()
                    .find(|n| n.kind() == "qualified_identifier")
                {
                    if let Some(name) = qualified.child_by_field_name("name") {
                        out.add_symbol(ctx.scope(*node), name, node.end_byte());
                    }
                }
            }
            if node.kind() == "function_definition" {
                if let Some(name) = node
                    .child_by_field_name("declarator")
                    .and_then(|n| out.declarator(n))
                {
                    out.globals.insert(ctx.bounded_source(name).into());
                }
            }
            if node.kind() == "enumerator" {
                if let Some(name) = node.child_by_field_name("name") {
                    out.add(ctx.scope(*node), name, node.end_byte(), Value::Unknown);
                    if ctx.function_scope(ctx.scope(*node)) == 0 {
                        out.globals.insert(ctx.bounded_source(name).into());
                    }
                }
            }
            if matches!(
                node.kind(),
                "type_definition"
                    | "alias_declaration"
                    | "class_specifier"
                    | "struct_specifier"
                    | "enum_specifier"
            ) {
                if let Some(name) = node.child_by_field_name("name").or_else(|| {
                    node.child_by_field_name("declarator")
                        .and_then(|n| out.declarator(n))
                }) {
                    out.add(ctx.scope(*node), name, node.end_byte(), Value::Unknown);
                    if ctx.function_scope(ctx.scope(*node)) == 0 {
                        out.globals.insert(ctx.bounded_source(name).into());
                    }
                }
            }
            if matches!(
                node.kind(),
                "declaration"
                    | "parameter_declaration"
                    | "optional_parameter_declaration"
                    | "field_declaration"
            ) {
                let mut cursor = node.walk();
                for decl in node.children_by_field_name("declarator", &mut cursor) {
                    let Some(name) = out.declarator(decl) else {
                        continue;
                    };
                    let mut value = decl
                        .child_by_field_name("value")
                        .map(Value::Node)
                        .unwrap_or(Value::Unknown);
                    if let Some(ty) = node.child_by_field_name("type") {
                        let role = match ctx.bounded_source(ty).replace(' ', "").as_str() {
                            "oqs::KeyEncapsulation" => Some(Role::CppKem),
                            "oqs::Signature" => Some(Role::CppSig),
                            _ => None,
                        };
                        if let (Some(role), Some(arguments)) = (
                            role,
                            decl.child_by_field_name("value").filter(|n| {
                                matches!(n.kind(), "argument_list" | "initializer_list")
                            }),
                        ) {
                            value = Value::Cpp(arguments, role);
                        }
                    }
                    if conditional(*node) || ctx.function_scope(ctx.scope(*node)) == 0 {
                        value = Value::Unknown;
                    }
                    out.add(ctx.scope(*node), name, node.end_byte(), value);
                    if node.kind() == "declaration" && ctx.function_scope(ctx.scope(*node)) == 0 {
                        out.globals.insert(ctx.bounded_source(name).into());
                    }
                }
            }
        }
        for node in &ctx.nodes {
            if !out.charge() {
                break;
            }
            if node.kind() == "assignment_expression" {
                if let (Some(left), Some(right)) = (
                    node.child_by_field_name("left"),
                    node.child_by_field_name("right"),
                ) {
                    if left.kind() == "identifier" {
                        let scope = out
                            .binding_scope(left, node.start_byte())
                            .unwrap_or(ctx.scope(*node));
                        let plain = node
                            .child_by_field_name("operator")
                            .is_some_and(|n| ctx.source(n) == "=");
                        out.add(
                            scope,
                            left,
                            node.end_byte(),
                            if plain && !conditional(*node) {
                                Value::Node(right)
                            } else {
                                Value::Unknown
                            },
                        );
                    } else {
                        out.invalidate_all(*node);
                    }
                }
            }
            if node.kind() == "update_expression" {
                if let Some(n) = node
                    .child_by_field_name("argument")
                    .filter(|n| n.kind() == "identifier")
                {
                    let scope = out
                        .binding_scope(n, node.start_byte())
                        .unwrap_or(ctx.scope(*node));
                    out.add(scope, n, node.end_byte(), Value::Unknown);
                }
            }
            if node.kind() == "call_expression" {
                let Some(function) = node.child_by_field_name("function") else {
                    continue;
                };
                let name = function_name(ctx, function);
                let name = name.as_deref().unwrap_or("");
                let name = name.strip_prefix("::").unwrap_or(name);
                let arguments = args(*node);
                let output_indices: &[usize] = match name {
                    "OQS_KEM_keypair" | "OQS_KEM_encaps" | "OQS_SIG_keypair" | "OQS_SIG_sign" => {
                        &[1, 2]
                    }
                    "OQS_KEM_decaps" => &[1],
                    "EVP_PKEY_encapsulate" => &[1, 2, 3, 4],
                    "EVP_PKEY_decapsulate"
                    | "EVP_PKEY_encrypt"
                    | "EVP_PKEY_decrypt"
                    | "EVP_PKEY_sign"
                    | "EVP_PKEY_derive" => &[1, 2],
                    "RSA_public_encrypt"
                    | "RSA_private_encrypt"
                    | "RSA_public_decrypt"
                    | "RSA_private_decrypt" => &[2],
                    "RSA_sign" | "DSA_sign" | "ECDSA_sign" => &[3, 4],
                    "DH_compute_key" | "ECDH_compute_key" => &[0],
                    _ => &[],
                };
                if !output_indices.is_empty()
                    && out.api_available(name, function)
                    && output_indices.iter().any(|i| {
                        arguments
                            .get(*i)
                            .is_some_and(|n| !matches!(ctx.source(*n), "NULL" | "nullptr" | "0"))
                    })
                {
                    out.scalar_barriers
                        .entry(ctx.function_scope(ctx.scope(*node)))
                        .or_default()
                        .insert(node.end_byte());
                }
                if matches!(
                    name,
                    "EVP_PKEY_keygen" | "EVP_PKEY_generate" | "EVP_PKEY_paramgen"
                ) && out.api_available(name, function)
                {
                    if let Some(output) = arguments.get(1).and_then(|n| out.addressed(*n)) {
                        let scope = out
                            .binding_scope(output, node.start_byte())
                            .unwrap_or(ctx.scope(*node));
                        let value = if conditional(*node) {
                            Value::Unknown
                        } else {
                            arguments
                                .first()
                                .copied()
                                .map(Value::Generated)
                                .unwrap_or(Value::Unknown)
                        };
                        out.add(scope, output, node.end_byte(), value);
                    } else {
                        out.invalidate_all(*node);
                    }
                } else if super::digest_operation(name) && out.api_available(name, function) {
                    if let Some(output) = arguments.get(1).and_then(|n| out.addressed(*n)) {
                        let scope = out
                            .binding_scope(output, node.start_byte())
                            .unwrap_or(ctx.scope(*node));
                        let key = arguments
                            .get(if name.ends_with("_ex") { 5 } else { 4 })
                            .copied();
                        let value = if conditional(*node) {
                            Value::Unknown
                        } else {
                            key.map(Value::ContextFromKey).unwrap_or(Value::Unknown)
                        };
                        out.add(scope, output, node.end_byte(), value);
                    } else if !arguments
                        .get(1)
                        .is_some_and(|n| matches!(ctx.source(*n), "NULL" | "nullptr" | "0"))
                    {
                        out.invalidate_all(*node);
                    }
                } else {
                    let safe_c =
                        name != "wc_MakeRsaKey" && known(name) && out.api_available(name, function);
                    let safe_cpp = out
                        .receiver(function)
                        .and_then(|(receiver, method)| {
                            out.key(receiver, node.start_byte(), 0)
                                .map(|key| (key.role, method))
                        })
                        .is_some_and(|(role, method)| super::cpp_operation_allowed(role, method));
                    if !safe_c && !safe_cpp {
                        out.invalidate_all(*node);
                    }
                }
            }
        }
        out
    }
    fn invalidate_all(&mut self, node: Node<'_>) {
        let barriers = self
            .barriers
            .entry(self.ctx.function_scope(self.ctx.scope(node)))
            .or_default();
        barriers.insert(node.end_byte());
    }

    pub fn charge(&self) -> bool {
        let n = self.work.get();
        let limit = self.ctx.budget.max_ast_nodes.saturating_mul(8);
        if n >= limit {
            self.names_incomplete.set(true);
            self.ctx.limit("c_family_resolution_work", limit, n);
            return false;
        }
        self.work.set(n + 1);
        true
    }
    fn identifier(&self, node: Node<'_>) -> Option<String> {
        if !matches!(
            node.kind(),
            "identifier" | "type_identifier" | "namespace_identifier"
        ) {
            return None;
        }
        let value = self.ctx.bounded_source(node);
        if value.len() > 256 {
            self.ctx.limit("c_family_identifier_bytes", 256, 256);
            return None;
        }
        (!value.is_empty()).then(|| value.into())
    }
    fn declarator(&self, mut node: Node<'a>) -> Option<Node<'a>> {
        for _ in 0..8 {
            if matches!(node.kind(), "identifier" | "type_identifier") {
                return Some(node);
            }
            node = if node.kind() == "parenthesized_declarator" {
                *children(node).first()?
            } else {
                node.child_by_field_name("declarator")?
            };
        }
        self.ctx.limit("c_family_declarator_depth", 8, 8);
        self.names_incomplete.set(true);
        None
    }
    fn add(&mut self, scope: usize, node: Node<'a>, at: usize, value: Value<'a>) {
        let Some(name) = self.identifier(node) else {
            return;
        };
        let key = (scope, name);
        let bindings = self.bindings.entry(key.clone()).or_default();
        if bindings.len() >= 64 {
            self.ctx.limit("c_family_same_name_bindings", 64, 64);
            self.overflows
                .entry(key)
                .and_modify(|old| *old = (*old).min(at))
                .or_insert(at);
            return;
        }
        let index = bindings.partition_point(|b| b.at <= at);
        bindings.insert(index, Binding { at, value });
    }

    fn binding_scope(&self, node: Node<'_>, at: usize) -> Option<usize> {
        let name = self.identifier(node)?;
        let function = self.ctx.function_scope(self.ctx.scope(node));
        let mut scope = Some(self.ctx.scope(node));
        for _ in 0..64 {
            if !self.charge() {
                return None;
            }
            let s = scope?;
            if self.ctx.function_scope(s) != function {
                return None;
            }
            if self
                .bindings
                .get(&(s, name.clone()))
                .is_some_and(|b| b.first().is_some_and(|v| v.at <= at))
            {
                return Some(s);
            }
            scope = self.ctx.scope_parent(s);
        }
        self.ctx.limit("c_family_scope_depth", 64, 64);
        self.names_incomplete.set(true);
        None
    }
    fn binding(&self, node: Node<'_>, at: usize) -> Option<Binding<'a>> {
        if !self.charge() {
            return None;
        }
        let name = self.identifier(node)?;
        let scope = self.binding_scope(node, at)?;
        let key = (scope, name);
        let entries = self.bindings.get(&key)?;
        let index = entries.partition_point(|b| b.at <= at).checked_sub(1)?;
        let binding = entries[index];
        let function = self.ctx.function_scope(self.ctx.scope(node));
        let mutated = self.barriers.get(&function).is_some_and(|bs| {
            bs.range(..=at)
                .next_back()
                .is_some_and(|last| *last > binding.at)
        });
        if mutated || self.overflows.get(&key).is_some_and(|first| at >= *first) {
            return Some(Binding {
                at: binding.at,
                value: Value::Unknown,
            });
        }
        Some(binding)
    }
    fn scalar_binding(&self, node: Node<'_>, at: usize) -> Option<Binding<'a>> {
        let binding = self.binding(node, at)?;
        let function = self.ctx.function_scope(self.ctx.scope(node));
        if self.scalar_barriers.get(&function).is_some_and(|barriers| {
            barriers
                .range(..=at)
                .next_back()
                .is_some_and(|last| *last > binding.at)
        }) {
            Some(Binding {
                at: binding.at,
                value: Value::Unknown,
            })
        } else {
            Some(binding)
        }
    }
    fn add_symbol(&mut self, scope: usize, node: Node<'_>, at: usize) {
        if let Some(name) = self.identifier(node) {
            self.symbols
                .entry(scope)
                .or_default()
                .entry(name)
                .and_modify(|old| *old = (*old).min(at))
                .or_insert(at);
        }
    }
    fn symbol_shadowed(&self, name: &str, node: Node<'_>, at: usize) -> bool {
        if self.symbols.is_empty() {
            return false;
        }
        if self.ctx.source(node).trim_start().starts_with("::") {
            return self
                .symbols
                .get(&0)
                .and_then(|s| s.get(name))
                .is_some_and(|start| *start <= at);
        }
        let mut scope = Some(self.ctx.scope(node));
        for _ in 0..64 {
            if !self.charge() {
                return true;
            }
            let Some(s) = scope else { return false };
            if self
                .symbols
                .get(&s)
                .and_then(|m| m.get(name))
                .is_some_and(|start| *start <= at)
            {
                return true;
            }
            scope = self.ctx.scope_parent(s);
        }
        self.ctx.limit("c_family_symbol_scope_depth", 64, 64);
        self.names_incomplete.set(true);
        true
    }
    pub fn api_available(&self, name: &str, node: Node<'_>) -> bool {
        if name.len() > 512 {
            self.ctx.limit("c_family_api_identifier_bytes", 512, 512);
            return false;
        }
        let shadowed = self.binding(node, node.start_byte()).is_some()
            || self.symbol_shadowed(name, node, node.start_byte())
            || (name.starts_with("oqs::")
                && (self.symbol_shadowed("oqs", node, node.start_byte())
                    || (!self.ctx.source(node).trim_start().starts_with("::")
                        && node.child_by_field_name("scope").is_some_and(|scope| {
                            self.binding(scope, node.start_byte()).is_some()
                        }))));
        !self.names_incomplete.get()
            && !self.macros.contains(name)
            && !self.globals.contains(name)
            && (!name.starts_with("oqs::") || !self.oqs_shadowed())
            && (!name.starts_with("Botan::") || !self.botan_shadow)
            && !shadowed
    }
    pub fn implementation_available(&self, name: &str) -> bool {
        !self.names_incomplete.get() && !self.macros.contains(name)
    }
    pub fn oqs_shadowed(&self) -> bool {
        self.names_incomplete.get()
            || self.oqs_shadow
            || self.macros.contains("oqs")
            || self.globals.contains("oqs")
    }
    pub fn receiver(&self, node: Node<'a>) -> Option<(Node<'a>, &'s str)> {
        if node.kind() != "field_expression" {
            return None;
        }
        let receiver = node.child_by_field_name("argument")?;
        if receiver.kind() != "identifier" {
            return None;
        }
        let field = node.child_by_field_name("field")?;
        Some((receiver, self.ctx.bounded_source(field)))
    }
    fn addressed(&self, node: Node<'a>) -> Option<Node<'a>> {
        if node.kind() == "pointer_expression"
            && node
                .child_by_field_name("operator")
                .is_some_and(|n| self.ctx.source(n) == "&")
        {
            node.child_by_field_name("argument")
                .filter(|n| n.kind() == "identifier")
        } else {
            None
        }
    }
    pub fn literal(&self, node: Node<'a>, at: usize, depth: usize) -> Option<String> {
        if depth > 8 {
            self.ctx.limit("c_family_alias_depth", 8, 8);
            return None;
        }
        if !self.charge() {
            return None;
        }
        if node.kind() == "string_literal" {
            let raw = self.ctx.bounded_source(node);
            return raw
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .filter(|s| s.len() <= 512 && !s.contains(['\\', '"']))
                .map(str::to_owned);
        }
        if let Some(binding) = self.scalar_binding(node, at) {
            if let Value::Node(value) = binding.value {
                return self.literal(
                    value,
                    value.start_byte(),
                    depth + usize::from(value.kind() != "string_literal"),
                );
            }
        }
        None
    }
    fn selector(
        &self,
        node: Option<Node<'a>>,
        at: usize,
        depth: usize,
        kind: SelectorKind,
    ) -> Option<String> {
        let n = node?;
        if depth > 8 {
            self.ctx.limit("c_family_alias_depth", 8, 8);
            return None;
        }
        if !self.charge() {
            return None;
        }
        if n.kind() == "string_literal" {
            let raw = self.ctx.bounded_source(n);
            return raw
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .filter(|s| !s.contains(['\\', '"']))
                .and_then(|value| selected_literal(value, kind));
        }
        if n.kind() == "identifier"
            || (n.kind() == "type_identifier" && {
                let mut ancestor = n.parent();
                let mut recovered = false;
                for _ in 0..8 {
                    let Some(parent) = ancestor else {
                        break;
                    };
                    if parent.kind() == "declaration" {
                        recovered = super::recovered_cpp_call(self.ctx, parent).is_some();
                        break;
                    }
                    ancestor = parent.parent();
                }
                recovered
            })
        {
            if let Some(binding) = self.scalar_binding(n, at) {
                return match binding.value {
                    Value::Node(value) => self.selector(
                        Some(value),
                        value.start_byte(),
                        depth + usize::from(value.kind() != "string_literal"),
                        kind,
                    ),
                    _ => None,
                };
            }
            let name = self.ctx.bounded_source(n);
            if self.macros.contains(name)
                || self.globals.contains(name)
                || self.symbol_shadowed(name, n, at)
            {
                return None;
            }
            if kind == SelectorKind::EvpId {
                return name
                    .strip_prefix("EVP_PKEY_")
                    .filter(|s| matches!(*s, "RSA" | "DSA" | "DH" | "EC" | "ED25519" | "X25519"))
                    .map(|s| if s == "ED25519" { "Ed25519" } else { s }.to_owned());
            }
            if let Some(s) = name.strip_prefix("OQS_KEM_alg_ml_kem_") {
                return selected_literal(&format!("ML-KEM-{s}"), kind);
            }
            if let Some(s) = name.strip_prefix("OQS_SIG_alg_ml_dsa_") {
                return selected_literal(&format!("ML-DSA-{s}"), kind);
            }
            if let Some(s) = name.strip_prefix("OQS_SIG_alg_slh_dsa_pure_") {
                let (hash, set) = s.split_once('_')?;
                if !matches!(hash, "sha2" | "shake")
                    || !matches!(set, "128s" | "128f" | "192s" | "192f" | "256s" | "256f")
                {
                    return None;
                }
                return selected_literal(
                    &format!(
                        "SLH_DSA_PURE_{}_{}",
                        hash.to_ascii_uppercase(),
                        set.to_ascii_uppercase()
                    ),
                    kind,
                );
            }
        }
        None
    }
    pub fn constructor(&self, node: Node<'a>, depth: usize) -> Option<Key> {
        let function = node
            .child_by_field_name("function")
            .or_else(|| node.child_by_field_name("type"))?;
        let name = function_name(self.ctx, function)?;
        let name = name.strip_prefix("::").unwrap_or(&name);
        if !self.api_available(name, function) {
            return None;
        }
        let arguments = args(node);
        let at = node.start_byte();
        let (role, selector) = match name {
            "EVP_PKEY_CTX_new_id" => (Role::Context, arguments.first().copied()),
            "EVP_PKEY_CTX_new_from_name" => (Role::Context, arguments.get(1).copied()),
            name if raw_key_constructor(name) => (
                Role::Key,
                arguments.get(usize::from(name.ends_with("_ex"))).copied(),
            ),
            "OQS_KEM_new" => (Role::OqsKem, arguments.first().copied()),
            "OQS_SIG_new" => (Role::OqsSig, arguments.first().copied()),
            "oqs::KeyEncapsulation" => (Role::CppKem, arguments.first().copied()),
            "oqs::Signature" => (Role::CppSig, arguments.first().copied()),
            _ => {
                if context_constructor(name) {
                    let offset = usize::from(name == "EVP_PKEY_CTX_new_from_pkey");
                    let mut key = self.key(*arguments.get(offset)?, at, depth + 1)?;
                    let required = if name == "EVP_PKEY_CTX_dup" {
                        Role::Context
                    } else {
                        Role::Key
                    };
                    if key.role != required {
                        return None;
                    }
                    key.role = Role::Context;
                    return Some(key);
                }
                let algorithm = low_level(name)?;
                if !matches!(
                    name,
                    "RSA_new" | "DSA_new" | "DH_new" | "EC_KEY_new" | "EC_KEY_new_by_curve_name"
                ) {
                    return None;
                }
                return Some(Key {
                    role: Role::LowLevelKey,
                    algorithm: algorithm.into(),
                });
            }
        };
        let kind = if name == "EVP_PKEY_CTX_new_id"
            || (raw_key_constructor(name) && !name.ends_with("_ex"))
        {
            SelectorKind::EvpId
        } else {
            match role {
                Role::OqsKem | Role::CppKem => SelectorKind::OqsKem,
                Role::OqsSig | Role::CppSig => SelectorKind::OqsSig,
                _ => SelectorKind::EvpName,
            }
        };
        let algorithm = self.selector(selector, at, 0, kind)?;
        if raw_key_constructor(name)
            && !matches!(algorithm.as_str(), "Ed25519" | "X25519")
            && !algorithm.starts_with("ML-KEM-")
            && !algorithm.starts_with("ML-DSA-")
        {
            return None;
        }
        Self::valid_role(role, &algorithm).then_some(Key { role, algorithm })
    }
    fn valid_role(role: Role, algorithm: &str) -> bool {
        match role {
            Role::OqsKem | Role::CppKem => {
                algorithm.starts_with("ML-KEM") || algorithm.starts_with("Kyber")
            }
            Role::OqsSig | Role::CppSig => {
                algorithm.starts_with("ML-DSA")
                    || algorithm.starts_with("SLH-")
                    || algorithm.starts_with("Dilithium")
            }
            _ => true,
        }
    }
    pub fn cpp_constructor(&self, arguments: Node<'a>, role: Role) -> Option<Key> {
        if self.oqs_shadowed() {
            return None;
        }
        if let Some(ty) = arguments
            .parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.child_by_field_name("type"))
        {
            let name = if role == Role::CppKem {
                "oqs::KeyEncapsulation"
            } else {
                "oqs::Signature"
            };
            if !self.api_available(name, ty) {
                return None;
            }
        }
        let kind = if role == Role::CppKem {
            SelectorKind::OqsKem
        } else {
            SelectorKind::OqsSig
        };
        let algorithm = self.selector(
            args(arguments).first().copied(),
            arguments.start_byte(),
            0,
            kind,
        )?;
        Self::valid_role(role, &algorithm).then_some(Key { role, algorithm })
    }

    pub fn key(&self, node: Node<'a>, at: usize, depth: usize) -> Option<Key> {
        if depth > 8 {
            self.ctx.limit("c_family_alias_depth", 8, 8);
            return None;
        }
        if !self.charge() {
            return None;
        }
        if node.kind() == "call_expression" {
            return self.constructor(node, depth);
        }
        if node.kind() == "parenthesized_expression" {
            return self.key(*children(node).first()?, at, depth + 1);
        }
        let binding = self.binding(node, at)?;
        match binding.value {
            Value::Node(value) => {
                if conditional(value) {
                    return None;
                }
                if value.kind() == "call_expression" {
                    self.constructor(value, depth)
                } else {
                    self.key(value, value.start_byte(), depth + 1)
                }
            }
            Value::Generated(context) => {
                let mut key = self.key(context, context.start_byte(), depth + 1)?;
                if key.role != Role::Context {
                    return None;
                }
                key.role = Role::Key;
                Some(key)
            }
            Value::ContextFromKey(key_node) => {
                let mut key = self.key(key_node, key_node.start_byte(), depth + 1)?;
                if key.role != Role::Key {
                    return None;
                }
                key.role = Role::Context;
                Some(key)
            }
            Value::Cpp(arguments, role) => self.cpp_constructor(arguments, role),
            Value::Unknown => None,
        }
    }
}
