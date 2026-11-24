//! Bounded declared version inputs, never Ruby execution or package resolution.
use super::{bound_spec, literal, method};
use pqc_scan_core::{RepositoryContext, ScannableFile};
use std::{collections::BTreeMap, path::Path};
use tree_sitter::Node;

pub(super) struct Versions<'a, 's> {
    definitions: BTreeMap<&'s str, Node<'a>>,
    text: &'s str,
    file: &'s ScannableFile,
    repo: Option<&'s RepositoryContext>,
    cache: BTreeMap<String, Option<String>>,
    pub limited: bool,
}
fn args(node: Node<'_>) -> Option<Vec<Node<'_>>> {
    let Some(args) = node.child_by_field_name("arguments") else {
        return Some(Vec::new());
    };
    if args.named_child_count() > 2 {
        return None;
    }
    let mut cursor = args.walk();
    Some(args.named_children(&mut cursor).collect())
}
fn supported_call(node: Node<'_>, text: &str) -> bool {
    let name = method(node, text).unwrap_or("");
    let Some(receiver) = node.child_by_field_name("receiver") else {
        return matches!(
            name,
            "gem" | "gemspec" | "group" | "platforms" | "platform" | "source" | "ruby"
        );
    };
    match &text[receiver.byte_range()] {
        "File" => return matches!(name, "read" | "expand_path"),
        "Gem::Specification" => return name == "new",
        _ => {}
    }
    if name == "strip" && method(receiver, text) == Some("read") {
        return receiver
            .child_by_field_name("receiver")
            .is_some_and(|r| &text[r.byte_range()] == "File");
    }
    if name == "concat" && metadata_array(receiver, text) {
        return literal_metadata_concat(node, text);
    }
    if receiver.kind() != "identifier" || !bound_spec(node, &text[receiver.byte_range()], text) {
        return false;
    }
    if matches!(
        name,
        "add_dependency" | "add_runtime_dependency" | "add_development_dependency"
    ) {
        return true;
    }
    let setter = node
        .parent()
        .is_some_and(|p| p.kind() == "assignment" && p.child_by_field_name("left") == Some(node));
    if name == "metadata"
        && node
            .parent()
            .is_some_and(|p| p.kind() == "element_reference")
    {
        return true;
    }
    if metadata_array(node, text) {
        if let Some(parent) = node.parent().filter(|p| p.kind() == "call") {
            return method(parent, text) == Some("concat") && literal_metadata_concat(parent, text);
        }
        // Only standalone append chains with literal text are modeled. The
        // metadata array cannot receive or expose a version-string reference.
        let mut current = node;
        let mut appended = false;
        for _ in 0..8 {
            let Some(parent) = current.parent().filter(|p| p.kind() == "binary") else {
                return appended;
            };
            if parent.child_by_field_name("left") != Some(current)
                || parent
                    .child_by_field_name("operator")
                    .is_none_or(|n| &text[n.byte_range()] != "<<")
                || parent
                    .child_by_field_name("right")
                    .and_then(|n| literal(n, text))
                    .is_none()
            {
                return false;
            }
            appended = true;
            current = parent;
        }
        return false;
    }
    setter
        && matches!(
            name,
            "name"
                | "version"
                | "summary"
                | "description"
                | "author"
                | "authors"
                | "email"
                | "homepage"
                | "license"
                | "licenses"
                | "platform"
                | "required_ruby_version"
                | "required_rubygems_version"
                | "files"
                | "test_files"
                | "require_path"
                | "require_paths"
                | "metadata"
                | "bindir"
                | "executables"
                | "extra_rdoc_files"
        )
}
fn metadata_array(node: Node<'_>, text: &str) -> bool {
    matches!(method(node, text), Some("requirements" | "rdoc_options"))
        && node.child_by_field_name("arguments").is_none()
        && node.child_by_field_name("receiver").is_some_and(|r| {
            r.kind() == "identifier" && bound_spec(node, &text[r.byte_range()], text)
        })
}
fn literal_metadata_concat(node: Node<'_>, text: &str) -> bool {
    let Some(args) = args(node).filter(|a| a.len() == 1) else {
        return false;
    };
    let list = args[0];
    if list.kind() != "array" || list.named_child_count() > 32 {
        return false;
    }
    let mut cursor = list.walk();
    let valid = list
        .named_children(&mut cursor)
        .all(|n| literal(n, text).is_some());
    valid
}
impl<'a, 's> Versions<'a, 's> {
    pub fn new(
        nodes: &[Node<'a>],
        text: &'s str,
        file: &'s ScannableFile,
        repo: Option<&'s RepositoryContext>,
    ) -> Self {
        let mut definitions = BTreeMap::new();
        let mut unsafe_names = std::collections::BTreeSet::new();
        let opaque = nodes.iter().any(|n| {
            // Reading a previously assigned spec.version can expose the mutable
            // string to an alias. Only the direct specification setter is modeled.
            let version_read = n.kind() == "call"
                && method(*n, text) == Some("version")
                && n.child_by_field_name("receiver").is_some()
                && n.parent().is_none_or(|p| {
                    p.kind() != "assignment" || p.child_by_field_name("left") != Some(*n)
                });
            let dir_shadow = n.kind() == "identifier"
                && &text[n.byte_range()] == "__dir__"
                && n.parent()
                    .is_none_or(|p| p.kind() != "argument_list" || p.named_child(1) != Some(*n));
            version_read
                || dir_shadow
                || n.kind() == "call" && !supported_call(*n, text)
                || matches!(
                    n.kind(),
                    "alias"
                        | "undef"
                        | "method"
                        | "singleton_method"
                        | "class"
                        | "module"
                        | "singleton_class"
                        | "subshell"
                )
                || n.kind() == "call"
                    && matches!(
                        method(*n, text),
                        Some(
                            "require"
                                | "require_relative"
                                | "load"
                                | "eval"
                                | "instance_eval"
                                | "class_eval"
                                | "module_eval"
                                | "send"
                                | "public_send"
                                | "define_method"
                                | "define_singleton_method"
                                | "const_set"
                        )
                    )
                || matches!(
                    n.kind(),
                    "assignment" | "operator_assignment" | "multiple_assignment"
                ) && n.child_by_field_name("left").is_some_and(|left| {
                    ["File", "Gem"]
                        .iter()
                        .any(|name| text[left.byte_range()].contains(name))
                })
        });
        if !opaque {
            for node in nodes {
                if node.kind() != "assignment"
                    || node.parent().is_none_or(|p| p.kind() != "program")
                {
                    continue;
                }
                if let (Some(left), Some(right)) = (
                    node.child_by_field_name("left"),
                    node.child_by_field_name("right"),
                ) {
                    if left.kind() == "identifier"
                        && definitions
                            .insert(&text[left.byte_range()], right)
                            .is_some()
                    {
                        unsafe_names.insert(&text[left.byte_range()]);
                    }
                }
            }
            for node in nodes.iter().filter(|n| n.kind() == "identifier") {
                let name = &text[node.byte_range()];
                if node
                    .parent()
                    .is_some_and(|p| p.child_by_field_name("method") == Some(*node))
                {
                    continue;
                }
                if !definitions.contains_key(name) {
                    continue;
                }
                let allowed = node.parent().is_some_and(|p| {
                    if p.kind() == "assignment" {
                        if p.child_by_field_name("left") == Some(*node)
                            && p.parent().is_some_and(|p| p.kind() == "program")
                        {
                            return true;
                        }
                        return p.child_by_field_name("right") == Some(*node)
                            && p.child_by_field_name("left").is_some_and(|left| {
                                method(left, text) == Some("version")
                                    && left
                                        .child_by_field_name("receiver")
                                        .is_some_and(|r| bound_spec(p, &text[r.byte_range()], text))
                            });
                    }
                    if p.kind() == "interpolation" {
                        return p.named_child_count() == 1;
                    }
                    p.kind() == "argument_list"
                        && p.parent().is_some_and(|call| {
                            let recognized = if method(call, text) == Some("gem") {
                                call.child_by_field_name("receiver").is_none()
                            } else {
                                matches!(
                                    method(call, text),
                                    Some(
                                        "add_dependency"
                                            | "add_runtime_dependency"
                                            | "add_development_dependency"
                                    )
                                ) && call
                                    .child_by_field_name("receiver")
                                    .is_some_and(|r| bound_spec(call, &text[r.byte_range()], text))
                            };
                            recognized && p.named_child(0) != Some(*node)
                        })
                });
                if !allowed {
                    unsafe_names.insert(name);
                }
            }
        }
        definitions.retain(|name, _| !unsafe_names.contains(name));
        Self {
            definitions,
            text,
            file,
            repo,
            cache: BTreeMap::new(),
            limited: false,
        }
    }
    pub fn value(&mut self, node: Node<'a>) -> Option<String> {
        if node.kind() != "identifier" {
            return None;
        }
        let value = *self.definitions.get(&self.text[node.byte_range()])?;
        if value.end_byte() >= node.start_byte() {
            return None;
        }
        if let Some(value) = literal(value, self.text) {
            return Some(value.to_owned());
        }
        // File.read(File.expand_path('relative', __dir__)).strip only. In
        // particular, File.read('relative') depends on an unknown working dir.
        if method(value, self.text) != Some("strip") || !args(value)?.is_empty() {
            return None;
        }
        let read = value.child_by_field_name("receiver")?;
        if method(read, self.text) != Some("read")
            || self.text[read.child_by_field_name("receiver")?.byte_range()] != *"File"
        {
            return None;
        }
        let read_args = args(read)?;
        if read_args.len() != 1 {
            return None;
        }
        let expand = read_args[0];
        if method(expand, self.text) != Some("expand_path")
            || self.text[expand.child_by_field_name("receiver")?.byte_range()] != *"File"
        {
            return None;
        }
        let path_args = args(expand)?;
        if path_args.len() != 2 || &self.text[path_args[1].byte_range()] != "__dir__" {
            return None;
        }
        let path = literal(path_args[0], self.text)?;
        if path.len() > 256 || path.contains(['\0', '\\']) || Path::new(path).is_absolute() {
            return None;
        }
        if let Some(cached) = self.cache.get(path) {
            return cached.clone();
        }
        if self.cache.len() == 8 {
            self.limited = true;
            return None;
        }
        let result = self
            .repo
            .and_then(|repo| repo.read_relative(&self.file.path, Path::new(path)))
            .and_then(|f| {
                if f.bytes.len() > 128 || !f.bytes.is_ascii() {
                    return None;
                }
                let value = f.text?.trim().to_string();
                (!value.is_empty() && value.is_ascii() && !value.contains(char::is_whitespace))
                    .then_some(value)
            });
        self.cache.insert(path.to_owned(), result.clone());
        result
    }
}
