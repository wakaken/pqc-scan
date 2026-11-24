//! Candidate references through bounded same-class Ruby data flow. No execution,
//! generated accessors, inheritance, external files or runtime dispatch inference.
use super::{
    context::{children, literal, Context, MAX_DEPTH},
    Facts,
};
use std::collections::HashMap;
use tree_sitter::Node;
const ID: &str = "API_RUBY_OPENSSL_CIPHER";
type Variables<'a> = HashMap<(usize, bool, usize, String), Vec<Node<'a>>>;
type Methods<'a> = HashMap<(usize, bool, String), Vec<Node<'a>>>;

fn owners(node: Node<'_>) -> (usize, usize, bool) {
    let mut parent = node.parent();
    let (mut method, mut singleton) = (0, false);
    for _ in 0..64 {
        let Some(n) = parent else { break };
        match n.kind() {
            "method" | "singleton_method" if method == 0 => {
                method = n.id();
                singleton = n.kind() == "singleton_method";
            }
            "singleton_class" => singleton = true,
            "class" | "module" => return (n.id(), method, singleton),
            _ => {}
        }
        parent = n.parent();
    }
    (0, method, singleton)
}
// Blocks can introduce locals or change self (for example instance_eval).
// Until receiver/block binding analysis is implemented, resolve only direct
// literal arguments there. Do not borrow an outer method's values or dispatch.
fn opaque_scope(node: Node<'_>) -> bool {
    let mut parent = node.parent();
    for _ in 0..64 {
        let Some(n) = parent else { break };
        match n.kind() {
            "block" | "do_block" | "lambda" | "for" => return true,
            "singleton_class"
                if n.child_by_field_name("value")
                    .is_none_or(|v| v.kind() != "self")
                    || owners(n).1 != 0 =>
            {
                return true
            }
            "class" | "module" => return false,
            _ => {}
        }
        parent = n.parent();
    }
    false
}
struct Resolver<'a, 'c, 's> {
    ctx: &'c Context<'a, 's>,
    variables: Variables<'a>,
    methods: Methods<'a>,
    constants: HashMap<(usize, String), Vec<Node<'a>>>,
    changed_constants: std::collections::HashSet<String>,
    dynamic_constants: bool,
    remaining: usize,
}
impl<'a> Resolver<'a, '_, '_> {
    fn values(&mut self, node: Node<'a>, depth: usize, out: &mut Vec<String>) {
        if depth > MAX_DEPTH || self.remaining == 0 || out.len() >= 16 {
            if depth > MAX_DEPTH {
                self.ctx.limit("source_alias_depth", MAX_DEPTH, MAX_DEPTH);
            } else if self.remaining == 0 {
                self.ctx.limit(
                    "ruby_value_candidates",
                    self.ctx.budget.max_ast_candidates,
                    self.ctx.budget.max_ast_candidates,
                );
            } else {
                self.ctx.limit("ruby_algorithm_values", 16, out.len());
            }
            return;
        }
        self.remaining -= 1;
        if let Some(value) = literal(self.ctx.text, node) {
            // Interpolated Ruby strings are not literal algorithm selections.
            if matches!(
                value,
                "aes-128-gcm"
                    | "aes-192-gcm"
                    | "aes-256-gcm"
                    | "aes-128-cbc"
                    | "aes-192-cbc"
                    | "aes-256-cbc"
                    | "AES-128-GCM"
                    | "AES-192-GCM"
                    | "AES-256-GCM"
                    | "AES-128-CBC"
                    | "AES-192-CBC"
                    | "AES-256-CBC"
            ) {
                out.push(value.to_owned());
            }
            return;
        }
        if opaque_scope(node) {
            return;
        }
        let (class, method, singleton) = owners(node);
        if node.kind() == "constant" {
            let name = self.ctx.source(node);
            if class != 0 && !self.dynamic_constants && !self.changed_constants.contains(name) {
                if let Some(values) = self.constants.get(&(class, name.to_owned())) {
                    if values.len() == 1 && literal(self.ctx.text, values[0]).is_some() {
                        self.values(values[0], depth + 1, out);
                    }
                }
            }
            return;
        }
        // Class/eigenclass bodies and their instance-variable storage have
        // different receivers. Only ordinary instance-method @ivar flow is
        // modeled; class methods can still return literal candidates.
        if method == 0 || node.kind() == "instance_variable" && singleton {
            return;
        }
        let class_receiver = singleton;
        match node.kind() {
            "identifier" | "instance_variable" => {
                let name = self.ctx.source(node).to_string();
                let local = if node.kind() == "instance_variable" {
                    0
                } else {
                    method
                };
                if let Some(assignments) =
                    self.variables
                        .get(&(class, class_receiver, local, name.clone()))
                {
                    // Reassignment is deliberately unresolved, never last-write guessing.
                    if assignments.len() == 1 {
                        let value = assignments[0];
                        if local == 0 || value.start_byte() < node.start_byte() {
                            self.values(value, depth + 1, out);
                        }
                    }
                } else if node.kind() == "identifier" {
                    self.method(class, class_receiver, &name, depth, out);
                }
            }
            "call" => {
                // Only a zero-argument same-class method reference is modeled.
                if node
                    .child_by_field_name("arguments")
                    .is_some_and(|n| n.named_child_count() != 0)
                {
                    return;
                }
                let Some(name) = node.child_by_field_name("method") else {
                    return;
                };
                let name = self.ctx.source(name).to_string();
                let receiver = node
                    .child_by_field_name("receiver")
                    .map(|n| self.ctx.source(n));
                let dispatch = match receiver {
                    Some("self.class") if !class_receiver => true,
                    Some("self") | None => class_receiver,
                    _ => return,
                };
                self.method(class, dispatch, &name, depth, out);
            }
            "binary"
                if node
                    .child_by_field_name("operator")
                    .is_some_and(|n| matches!(self.ctx.source(n), "||" | "or")) =>
            {
                for field in ["left", "right"] {
                    if let Some(n) = node.child_by_field_name(field) {
                        self.values(n, depth + 1, out);
                    }
                }
            }
            "if" | "elsif" | "unless" => {
                for field in ["consequence", "alternative"] {
                    if let Some(n) = node.child_by_field_name(field) {
                        self.values(n, depth + 1, out);
                    }
                }
            }
            "body_statement"
            | "then"
            | "else"
            | "parenthesized_statements"
            | "return"
            | "argument_list" => {
                if let Some(n) = children(node).last() {
                    self.values(*n, depth + 1, out);
                }
            }
            _ => {}
        }
    }
    fn method(
        &mut self,
        class: usize,
        singleton: bool,
        name: &str,
        depth: usize,
        out: &mut Vec<String>,
    ) {
        if let Some(methods) = self.methods.get(&(class, singleton, name.to_owned())) {
            if methods.len() == 1 {
                let method = methods[0];
                if opaque_scope(method) {
                    return;
                }
                if method
                    .child_by_field_name("parameters")
                    .is_none_or(|n| n.named_child_count() == 0)
                {
                    if let Some(body) = method.child_by_field_name("body") {
                        self.values(body, depth + 1, out);
                    }
                }
            }
        }
    }
}
pub(super) fn inspect(ctx: &Context<'_, '_>, facts: &mut Facts) {
    let mut resolver = Resolver {
        ctx,
        variables: HashMap::new(),
        methods: HashMap::new(),
        constants: HashMap::new(),
        changed_constants: std::collections::HashSet::new(),
        dynamic_constants: false,
        remaining: ctx.budget.max_ast_candidates,
    };
    let mut shadowed = false;
    for &node in &ctx.nodes {
        let (class, method, singleton) = owners(node);
        match node.kind() {
            "binary"
                if node
                    .child_by_field_name("operator")
                    .is_some_and(|n| ctx.source(n) == "<<") =>
            {
                if let Some(left) = node.child_by_field_name("left") {
                    for name in ctx
                        .source(left)
                        .split(|c: char| !c.is_alphanumeric() && c != '_')
                    {
                        if name.starts_with(char::is_uppercase) {
                            resolver.changed_constants.insert(name.to_owned());
                        }
                    }
                }
            }
            "call" => {
                if let Some(receiver) = node.child_by_field_name("receiver") {
                    if matches!(receiver.kind(), "constant" | "scope_resolution") {
                        if let Some(name) = ctx.source(receiver).rsplit("::").next() {
                            resolver.changed_constants.insert(name.to_owned());
                        }
                    }
                }
                if node.child_by_field_name("method").is_some_and(|n| {
                    matches!(
                        ctx.source(n),
                        "const_set"
                            | "remove_const"
                            | "autoload"
                            | "class_eval"
                            | "module_eval"
                            | "instance_eval"
                            | "eval"
                            | "send"
                            | "public_send"
                            | "__send__"
                    )
                }) {
                    resolver.dynamic_constants = true;
                }
            }
            "operator_assignment" | "multiple_assignment" => {
                if let Some(left) = node.child_by_field_name("left") {
                    for name in ctx
                        .source(left)
                        .split(|c: char| !c.is_alphanumeric() && c != '_')
                    {
                        if name.starts_with(char::is_uppercase) {
                            resolver.changed_constants.insert(name.to_owned());
                        }
                    }
                }
            }
            "method_parameters" => {
                // A parameter shadows a same-named method, even when its value
                // or default is dynamic. Do not treat a bare parameter as a call.
                for child in children(node) {
                    let name = if child.kind() == "identifier" {
                        Some(child)
                    } else {
                        child.child_by_field_name("name")
                    };
                    if let Some(name) = name {
                        let values = resolver
                            .variables
                            .entry((
                                class,
                                singleton || method == 0,
                                method,
                                ctx.source(name).to_owned(),
                            ))
                            .or_default();
                        if child.kind() == "optional_parameter" {
                            if let Some(value) = child.child_by_field_name("value") {
                                // A default is a possible reference, not the
                                // actual argument supplied by any caller.
                                values.push(value);
                            }
                        }
                    }
                }
            }
            "assignment" => {
                if let (Some(left), Some(right)) = (
                    node.child_by_field_name("left"),
                    node.child_by_field_name("right"),
                ) {
                    let name = ctx.source(left);
                    if name == "OpenSSL" || name.starts_with("OpenSSL::") {
                        shadowed = true;
                    }
                    if left.kind() == "constant" && method == 0 && !singleton {
                        resolver
                            .constants
                            .entry((class, name.to_owned()))
                            .or_default()
                            .push(if opaque_scope(node) { left } else { right });
                    } else if left.kind() == "scope_resolution" {
                        if let Some(name) = name.rsplit("::").next() {
                            resolver.changed_constants.insert(name.to_owned());
                        }
                    } else if !matches!(left.kind(), "identifier" | "instance_variable") {
                        // Ruby multiple assignment is assignment plus a
                        // left_assignment_list. Indexing may mutate the value.
                        for name in ctx
                            .source(left)
                            .split(|c: char| !c.is_alphanumeric() && c != '_')
                        {
                            if name.starts_with(char::is_uppercase) {
                                resolver.changed_constants.insert(name.to_owned());
                                if name == "OpenSSL" {
                                    shadowed = true;
                                }
                            }
                        }
                    }
                    if matches!(left.kind(), "identifier" | "instance_variable") {
                        resolver
                            .variables
                            .entry((
                                class,
                                singleton || method == 0,
                                if left.kind() == "instance_variable" {
                                    0
                                } else {
                                    method
                                },
                                name.to_owned(),
                            ))
                            .or_default()
                            // Keep opaque writes as unresolved rather than
                            // borrowing a block-local or rebound receiver value.
                            .push(if opaque_scope(node) { left } else { right });
                    }
                }
            }
            "method" | "singleton_method" => {
                if let Some(name) = node.child_by_field_name("name") {
                    let own_singleton = node.kind() == "singleton_method" || singleton;
                    if node.kind() != "singleton_method"
                        || node
                            .child_by_field_name("object")
                            .is_some_and(|n| ctx.source(n) == "self")
                    {
                        resolver
                            .methods
                            .entry((class, own_singleton, ctx.source(name).to_owned()))
                            .or_default()
                            .push(node);
                    }
                }
            }
            "class" | "module" => {
                if node
                    .child_by_field_name("name")
                    .is_some_and(|n| matches!(ctx.source(n), "OpenSSL" | "OpenSSL::Cipher"))
                {
                    shadowed = true;
                }
            }
            _ => {}
        }
    }
    if shadowed {
        return;
    }
    // A Ruby literal String remains mutable. Do not follow its value after it
    // escapes to a local alias, container or arbitrary call. The one trusted
    // consumption modeled here is the exact OpenSSL cipher constructor.
    for &node in &ctx.nodes {
        if node.kind() != "constant" {
            continue;
        }
        let name = ctx.source(node);
        let (class, _, _) = owners(node);
        if !resolver.constants.contains_key(&(class, name.to_owned())) {
            continue;
        }
        let Some(parent) = node.parent() else {
            continue;
        };
        let definition = parent.kind() == "assignment"
            && parent
                .child_by_field_name("left")
                .is_some_and(|n| n.id() == node.id());
        let constructor = parent.kind() == "argument_list"
            && parent.named_child_count() == 1
            && parent.parent().is_some_and(|call| {
                call.kind() == "call"
                    && call.child_by_field_name("receiver").is_some_and(|n| {
                        matches!(ctx.source(n), "OpenSSL::Cipher" | "::OpenSSL::Cipher")
                    })
                    && call
                        .child_by_field_name("method")
                        .is_some_and(|n| ctx.source(n) == "new")
            });
        if !definition && !constructor {
            resolver.changed_constants.insert(name.to_owned());
        }
    }
    for &node in &ctx.nodes {
        if node.kind() != "call"
            || node
                .child_by_field_name("receiver")
                .is_none_or(|n| !matches!(ctx.source(n), "OpenSSL::Cipher" | "::OpenSSL::Cipher"))
            || node
                .child_by_field_name("method")
                .is_none_or(|n| ctx.source(n) != "new")
        {
            continue;
        }
        let Some(args) = node.child_by_field_name("arguments") else {
            continue;
        };
        let args = children(args);
        if args.len() != 1 {
            continue;
        }
        let mut values = vec![];
        resolver.values(args[0], 0, &mut values);
        values.sort();
        values.dedup();
        // The match/native facts retain only the callee and allowlisted names.
        // Legacy surrounding snippets still use the existing privacy sanitizer.
        let end = node.child_by_field_name("method").unwrap().end_byte();
        let before = facts.hits.get(ID).map_or(0, Vec::len);
        facts.push(
            ID,
            node.start_byte(),
            end,
            "ruby_openssl_cipher_candidate_references",
        );
        if let Some(hit) = facts.hits.get_mut(ID).and_then(|v| v.get_mut(before)) {
            hit.algorithm_references = values;
        }
    }
}
