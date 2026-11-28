use super::{Assignment, Scope};
use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Range,
};
use tree_sitter::Node;

#[derive(Default)]
pub(super) struct Index<'tree> {
    setters: Vec<(Scope, Node<'tree>, Node<'tree>)>,
    shadows: BTreeMap<usize, bool>,
    contexts: BTreeMap<usize, Context>,
    regions: BTreeMap<Scope, Region>,
    region_order: Vec<Scope>,
    namespaces: BTreeMap<usize, Namespace>,
    namespace_order: Vec<usize>,
    parameters: BTreeSet<(Scope, String)>,
    names: BTreeMap<Scope, BTreeSet<String>>,
    declarations: BTreeMap<(Scope, String), usize>,
    bindings: BTreeMap<(Scope, String), Scope>,
}

#[derive(Clone, Copy)]
struct Context {
    scope: Scope,
    namespace: usize,
    parameter_path: bool,
}

struct Region {
    parent: Option<Scope>,
    barrier: Scope,
    start: usize,
    end: usize,
}

struct Namespace {
    parent: Option<usize>,
    relative_shadow: bool,
    absolute_shadow: bool,
}

fn region(kind: &str) -> bool {
    matches!(
        kind,
        "method"
            | "singleton_method"
            | "class"
            | "module"
            | "singleton_class"
            | "program"
            | "block"
            | "do_block"
            | "lambda"
    )
}

fn source<'s>(node: Node<'_>, text: &'s str) -> &'s str {
    text.get(node.byte_range())
        .filter(|s| s.len() <= 512)
        .unwrap_or("")
}

pub(super) fn local<'s>(node: Node<'_>, text: &'s str) -> Option<&'s str> {
    (node.kind() == "identifier")
        .then(|| text.get(node.byte_range()))
        .flatten()
        .filter(|s| !s.is_empty())
}

fn unwrap(mut node: Node<'_>) -> Result<Node<'_>, Node<'_>> {
    for _ in 0..16 {
        if node.kind() != "parenthesized_statements" || node.named_child_count() != 1 {
            break;
        }
        let Some(child) = node.named_child(0) else {
            break;
        };
        node = child;
    }
    if node.kind() == "parenthesized_statements" {
        Err(node)
    } else {
        Ok(node)
    }
}

pub(super) fn literal(node: Node<'_>) -> bool {
    if matches!(node.kind(), "integer" | "float") {
        return true;
    }
    if !matches!(node.kind(), "string" | "heredoc_body") {
        return false;
    }
    let mut cursor = node.walk();
    let plain = node
        .named_children(&mut cursor)
        .all(|n| matches!(n.kind(), "string_content" | "heredoc_content"));
    plain
}

pub(super) fn protect_values(node: Node<'_>, out: &mut Vec<Range<usize>>) {
    let mut cursor = node.walk();
    for _ in 0..256 {
        let current = cursor.node();
        let data = matches!(
            current.kind(),
            "string" | "heredoc_body" | "integer" | "float"
        );
        if data {
            out.push(current.byte_range());
        }
        if !data && cursor.goto_first_child() {
            continue;
        }
        loop {
            if cursor.goto_next_sibling() {
                break;
            }
            if !cursor.goto_parent() {
                return;
            }
        }
    }
    out.push(node.byte_range());
}

impl<'tree> Index<'tree> {
    pub(super) fn observe(
        &mut self,
        node: Node<'tree>,
        text: &str,
        assignments: &mut Vec<Assignment<'tree>>,
    ) {
        let parent = node.parent();
        let inherited = parent.and_then(|parent| self.contexts.get(&parent.id()).copied());
        let owner = if region(node.kind()) {
            let owner = Scope::Ruby(node.id());
            let parent = matches!(node.kind(), "block" | "do_block" | "lambda")
                .then_some(inherited.map(|context| context.scope))
                .flatten();
            let barrier = parent
                .and_then(|parent| self.regions.get(&parent).map(|region| region.barrier))
                .unwrap_or(owner);
            self.regions.insert(
                owner,
                Region {
                    parent,
                    barrier,
                    start: node.start_byte(),
                    end: node.end_byte(),
                },
            );
            self.region_order.push(owner);
            owner
        } else if let Some(context) = inherited {
            context.scope
        } else {
            return;
        };
        let namespace = if matches!(node.kind(), "class" | "module" | "program") {
            self.namespaces.insert(
                node.id(),
                Namespace {
                    parent: inherited.map(|context| context.namespace),
                    relative_shadow: false,
                    absolute_shadow: false,
                },
            );
            self.namespace_order.push(node.id());
            node.id()
        } else if let Some(context) = inherited {
            context.namespace
        } else {
            return;
        };
        let parameter_path = parent.is_some_and(|parent| {
            matches!(parent.kind(), "block_parameters" | "lambda_parameters")
                || inherited.is_some_and(|context| context.parameter_path)
                    && (parent.kind() == "destructured_parameter"
                        || parent
                            .child_by_field_name("name")
                            .is_some_and(|name| name.id() == node.id()))
        });
        self.contexts.insert(
            node.id(),
            Context {
                scope: owner,
                namespace,
                parameter_path,
            },
        );
        if let Some(name) = local(node, text) {
            self.names.entry(owner).or_default().insert(name.to_owned());
            if parameter_path {
                self.parameters.insert((owner, name.to_owned()));
            }
        }
        if node.kind() == "assignment" {
            if let (Some(left), Some(right)) = (
                node.child_by_field_name("left"),
                node.child_by_field_name("right"),
            ) {
                if let Some(name) = local(left, text) {
                    self.names.entry(owner).or_default().insert(name.to_owned());
                    assignments.push((owner, name.to_owned(), right));
                } else if matches!(left.kind(), "constant" | "scope_resolution") {
                    self.shadow(left, node, text);
                } else if left.kind() == "call"
                    && left
                        .child_by_field_name("method")
                        .is_some_and(|m| source(m, text) == "private_key")
                {
                    if let Some(receiver) = left.child_by_field_name("receiver") {
                        self.setters.push((owner, receiver, right));
                    }
                }
            }
        }
        if node.kind() == "call"
            && node
                .child_by_field_name("method")
                .is_some_and(|m| source(m, text) == "private_key")
            && node.child_by_field_name("arguments").is_none()
        {
            if let (Some(receiver), Some(error)) = (
                node.child_by_field_name("receiver"),
                node.next_named_sibling().filter(|n| n.is_error()),
            ) {
                self.setters.push((owner, receiver, error));
            }
        }
        if matches!(node.kind(), "class" | "module") {
            if let Some(name) = node.child_by_field_name("name") {
                self.shadow(name, node.parent().unwrap_or(node), text);
            }
        }
        if node.kind() == "call"
            && node.child_by_field_name("method").is_some_and(|m| {
                matches!(
                    source(m, text),
                    "const_set"
                        | "remove_const"
                        | "autoload"
                        | "class_eval"
                        | "module_eval"
                        | "eval"
                        | "send"
                        | "public_send"
                        | "__send__"
                )
            })
        {
            self.shadows.insert(namespace, true);
        }
        if node.kind() == "operator_assignment" || node.kind() == "multiple_assignment" {
            if let Some(left) = node.child_by_field_name("left") {
                let mut cursor = left.walk();
                let mut stack = vec![left];
                for _ in 0..256 {
                    let Some(part) = stack.pop() else { break };
                    if let Some(name) = local(part, text) {
                        self.names.entry(owner).or_default().insert(name.to_owned());
                        assignments.push((owner, name.to_owned(), node));
                    }
                    stack.extend(part.named_children(&mut cursor));
                }
            }
        }
    }

    fn scope(&self, node: Node<'_>) -> Option<Scope> {
        self.contexts.get(&node.id()).map(|context| context.scope)
    }

    pub(super) fn prepare(&mut self, assignments: &mut [Assignment<'tree>]) {
        for (owner, name, value) in assignments.iter() {
            self.declarations
                .entry((*owner, name.clone()))
                .and_modify(|start| *start = (*start).min(value.start_byte()))
                .or_insert(value.start_byte());
        }
        self.prepare_bindings();
        for (owner, name, _) in assignments {
            if let Some(binding) = self.bindings.get(&(*owner, name.clone())) {
                *owner = *binding;
            }
        }
        for id in &self.namespace_order {
            let namespace = &self.namespaces[id];
            let parent = namespace.parent.and_then(|id| self.namespaces.get(&id));
            let relative = parent.is_some_and(|parent| parent.relative_shadow)
                || self.shadows.contains_key(id);
            let absolute = parent.is_some_and(|parent| parent.absolute_shadow)
                || self.shadows.get(id).copied().unwrap_or(false);
            if let Some(namespace) = self.namespaces.get_mut(id) {
                namespace.relative_shadow = relative;
                namespace.absolute_shadow = absolute;
            }
        }
    }

    fn prepare_bindings(&mut self) {
        enum Event<'name> {
            Exit(Scope),
            Enter(Scope),
            Declare(Scope, &'name str),
        }
        let mut events = Vec::new();
        for (order, owner) in self.region_order.iter().enumerate() {
            let region = &self.regions[owner];
            events.push((region.start, 1, order, Event::Enter(*owner)));
            events.push((region.end, 0, usize::MAX - order, Event::Exit(*owner)));
        }
        for ((owner, name), position) in &self.declarations {
            events.push((*position, 2, 0, Event::Declare(*owner, name)));
        }
        events.sort_by_key(|(position, rank, order, _)| (*position, *rank, *order));
        let mut environment = BTreeMap::new();
        let mut undo: BTreeMap<_, Vec<(_, Option<Scope>)>> = BTreeMap::new();
        for (_, _, _, event) in events {
            match event {
                Event::Enter(owner) => {
                    let barrier = self.regions[&owner].barrier;
                    if let Some(names) = self.names.get(&owner) {
                        for name in names {
                            let key = (barrier, name.clone());
                            let parameter = self.parameters.contains(&(owner, name.clone()));
                            let binding = if parameter {
                                owner
                            } else {
                                environment.get(&key).copied().unwrap_or(owner)
                            };
                            self.bindings.insert((owner, name.clone()), binding);
                            if parameter {
                                let previous = environment.insert(key.clone(), owner);
                                undo.entry(owner).or_default().push((key, previous));
                            }
                        }
                    }
                }
                Event::Declare(owner, name) => {
                    let key = (self.regions[&owner].barrier, name.to_owned());
                    let binding = self.bindings[&(owner, name.to_owned())];
                    let previous = environment.insert(key.clone(), binding);
                    undo.entry(owner).or_default().push((key, previous));
                }
                Event::Exit(owner) => {
                    if let Some(changes) = undo.remove(&owner) {
                        for (key, previous) in changes.into_iter().rev() {
                            if let Some(previous) = previous {
                                environment.insert(key, previous);
                            } else {
                                environment.remove(&key);
                            }
                        }
                    }
                }
            }
        }
    }

    pub(super) fn binding(&self, node: Node<'_>, name: &str) -> Option<Scope> {
        self.scope(node)
            .and_then(|owner| self.bindings.get(&(owner, name.to_owned())).copied())
    }

    pub(super) fn extend_captures(&self, scopes: &mut BTreeSet<Scope>) {
        let mut pending: Vec<_> = scopes.iter().copied().collect();
        while let Some(owner) = pending.pop() {
            if let Some(parent) = self.regions.get(&owner).and_then(|region| region.parent) {
                if scopes.insert(parent) {
                    pending.push(parent);
                }
            }
        }
    }

    fn potential_captures(&self, mut potential: BTreeSet<Scope>) -> BTreeSet<Scope> {
        for owner in &self.region_order {
            if self.regions[owner]
                .parent
                .is_some_and(|parent| potential.contains(&parent))
            {
                potential.insert(*owner);
            }
        }
        potential
    }

    fn shadow(&mut self, name: Node<'_>, owner: Node<'_>, text: &str) {
        let raw = source(name, text).trim_start_matches("::");
        if matches!(raw, "OpenSSL" | "PKey" | "EC" | "BN")
            || raw.starts_with("OpenSSL::")
                && matches!(raw.rsplit("::").next(), Some("PKey" | "EC" | "BN"))
        {
            let global = raw == "OpenSSL" || raw.starts_with("OpenSSL::");
            let Some(namespace) = self
                .contexts
                .get(&owner.id())
                .map(|context| context.namespace)
            else {
                return;
            };
            self.shadows
                .entry(namespace)
                .and_modify(|value| *value |= global)
                .or_insert(global);
        }
    }

    fn trusted(&self, node: Node<'_>, absolute: bool) -> Option<bool> {
        let context = self.contexts.get(&node.id())?;
        let namespace = self.namespaces.get(&context.namespace)?;
        Some(if absolute {
            !namespace.absolute_shadow
        } else {
            !namespace.relative_shadow
        })
    }

    fn constructor(&self, node: Node<'_>, text: &str, class: &str, methods: &[&str]) -> EcOrigin {
        if node.kind() != "call"
            || !node
                .child_by_field_name("method")
                .is_some_and(|method| methods.contains(&source(method, text)))
        {
            return EcOrigin::Absent;
        }
        let Some(receiver) = node.child_by_field_name("receiver") else {
            return EcOrigin::Absent;
        };
        let name = source(receiver, text);
        if name.trim_start_matches("::") != class {
            return EcOrigin::Absent;
        }
        match self.trusted(node, name.starts_with("::")) {
            Some(true) => EcOrigin::Exact,
            Some(false) => EcOrigin::Absent,
            None => EcOrigin::Possible,
        }
    }

    pub(super) fn value(&self, node: Node<'tree>, text: &str) -> Node<'tree> {
        let mut node = unwrap(node).unwrap_or_else(|node| node);
        for _ in 0..16 {
            if !matches!(
                self.constructor(node, text, "OpenSSL::BN", &["new"]),
                EcOrigin::Exact
            ) {
                break;
            }
            let Some(arg) = node.child_by_field_name("arguments").and_then(|args| {
                let mut cursor = args.walk();
                let first = args
                    .named_children(&mut cursor)
                    .find(|arg| arg.kind() != "comment");
                first
            }) else {
                break;
            };
            node = unwrap(arg).unwrap_or_else(|node| node);
        }
        node
    }

    pub(super) fn inputs(
        &self,
        text: &str,
        assignments: &[Assignment<'tree>],
        remaining: usize,
    ) -> Vec<(Scope, Node<'tree>, bool)> {
        let potential = assignments
            .iter()
            .filter_map(|(owner, _, value)| {
                (match unwrap(*value) {
                    Ok(node) => !matches!(
                        self.constructor(node, text, "OpenSSL::PKey::EC", &["new", "generate"]),
                        EcOrigin::Absent
                    ),
                    Err(_) => true,
                })
                .then_some(*owner)
            })
            .collect();
        let mut origins = Origins {
            syntax: self,
            text,
            assignments,
            potential: self.potential_captures(potential),
            remaining,
        };
        self.setters
            .iter()
            .filter_map(
                |(owner, receiver, value)| match origins.resolve(*receiver, *owner, 0) {
                    EcOrigin::Exact => Some((*owner, *value, false)),
                    EcOrigin::Possible => Some((*owner, *value, true)),
                    EcOrigin::Absent => None,
                },
            )
            .collect()
    }
}

#[derive(Clone, Copy)]
enum EcOrigin {
    Exact,
    Possible,
    Absent,
}

struct Origins<'index, 'tree, 'source> {
    syntax: &'index Index<'tree>,
    text: &'source str,
    assignments: &'index [Assignment<'tree>],
    potential: BTreeSet<Scope>,
    remaining: usize,
}

impl<'tree> Origins<'_, 'tree, '_> {
    fn resolve(&mut self, node: Node<'tree>, owner: Scope, depth: usize) -> EcOrigin {
        let node = match unwrap(node) {
            Ok(node) => node,
            Err(_) => return EcOrigin::Possible,
        };
        match self
            .syntax
            .constructor(node, self.text, "OpenSSL::PKey::EC", &["new", "generate"])
        {
            EcOrigin::Exact => return EcOrigin::Exact,
            EcOrigin::Possible => return EcOrigin::Possible,
            EcOrigin::Absent => {}
        }
        if self.remaining == 0 || depth == 8 {
            return if self.potential.contains(&owner) {
                EcOrigin::Possible
            } else {
                EcOrigin::Absent
            };
        }
        self.remaining -= 1;
        let Some(name) = local(node, self.text) else {
            return EcOrigin::Absent;
        };
        let Some(owner) = self.syntax.binding(node, name) else {
            return EcOrigin::Possible;
        };
        let mut found = false;
        let mut unknown = false;
        let mut writes = 0;
        for (scope, alias, value) in self.assignments {
            if self.remaining == 0 {
                return if found || self.potential.contains(&owner) {
                    EcOrigin::Possible
                } else {
                    EcOrigin::Absent
                };
            }
            self.remaining -= 1;
            if *scope == owner && alias == name {
                writes += 1;
                match self.resolve(*value, owner, depth + 1) {
                    EcOrigin::Exact => found = true,
                    EcOrigin::Possible => {
                        found = true;
                        unknown = true;
                    }
                    EcOrigin::Absent => unknown = true,
                }
            }
        }
        if !found {
            EcOrigin::Absent
        } else if unknown || writes > 1 {
            EcOrigin::Possible
        } else {
            EcOrigin::Exact
        }
    }
}
