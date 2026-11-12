//! Local parent properties only: selected regular files, finite depth, no Maven execution.
use pqc_scan_core::{RepositoryContext, ScannableFile};
use std::collections::{BTreeMap, BTreeSet};
pub(super) type Properties = BTreeMap<String, String>;
pub(super) struct Context {
    pub properties: Properties,
    pub profiles: Vec<Properties>,
    pub limited: bool,
}
fn document(text: &str) -> Option<roxmltree::Document<'_>> {
    if text.contains("<!DOCTYPE") || text.contains("<!ENTITY") {
        return None;
    }
    let d = roxmltree::Document::parse_with_options(
        text,
        roxmltree::ParsingOptions {
            nodes_limit: 50000,
            ..Default::default()
        },
    )
    .ok()?;
    if d.descendants().any(|n| n.ancestors().take(66).count() > 65) {
        return None;
    }
    if d.descendants()
        .filter(|n| n.has_tag_name("properties"))
        .any(|n| n.children().filter(|n| n.is_element()).take(8193).count() > 8192)
    {
        return None;
    }
    if d.descendants().filter(|n| n.is_element()).any(|n| {
        n.tag_name()
            .namespace()
            .is_some_and(|ns| ns != "http://maven.apache.org/POM/4.0.0")
    }) {
        return None;
    }
    Some(d)
}
fn field<'a, 'b>(node: roxmltree::Node<'a, 'b>, key: &str) -> Option<&'a str> {
    node.children()
        .find(|n| n.has_tag_name(key))
        .and_then(|n| n.text())
        .map(str::trim)
}
pub(super) fn node_properties(node: roxmltree::Node<'_, '_>) -> Properties {
    let mut out = Properties::new();
    for n in node
        .children()
        .filter(|n| n.has_tag_name("properties"))
        .flat_map(|n| n.children().filter(|n| n.is_element()))
    {
        let key = n.tag_name().name();
        let value = n.text().unwrap_or("").trim();
        if out.get(key).is_some_and(|v| v != value) {
            out.insert(key.into(), format!("${{{key}}}"));
        } else {
            out.insert(key.into(), value.into());
        }
    }
    out
}

pub(super) fn load(file: &ScannableFile, repo: Option<&RepositoryContext>) -> Context {
    let mut seen = BTreeSet::new();
    let mut bytes = 0;
    visit(file, repo, &mut seen, &mut bytes, 0)
}
fn visit(
    file: &ScannableFile,
    repo: Option<&RepositoryContext>,
    seen: &mut BTreeSet<std::path::PathBuf>,
    bytes: &mut usize,
    depth: usize,
) -> Context {
    let mut out = Context {
        properties: Properties::new(),
        profiles: vec![],
        limited: false,
    };
    if depth >= 16 || !seen.insert(file.path.clone()) || *bytes > 8 * 1024 * 1024 {
        out.limited = true;
        return out;
    }
    *bytes += file.bytes.len();
    if *bytes > 8 * 1024 * 1024 {
        out.limited = true;
        return out;
    }
    let Some(doc) = file.text.as_deref().and_then(document) else {
        return out;
    };
    let root = doc.root_element();
    if !root.has_tag_name("project") {
        return out;
    }
    let parent = root.children().find(|n| n.has_tag_name("parent"));
    if let (Some(p), Some(repo)) = (parent, repo) {
        let relative = field(p, "relativePath").unwrap_or("../pom.xml");
        if !relative.is_empty() && !relative.contains(['$', '\0']) {
            let path = std::path::Path::new(relative);
            let parent_file = repo
                .read_relative(&file.path, path)
                .or_else(|| repo.read_relative(&file.path, &path.join("pom.xml")));
            if let Some(parent_file) = parent_file {
                if let Some(pd) = parent_file.text.as_deref().and_then(document) {
                    let pr = pd.root_element();
                    let pp = pr.children().find(|n| n.has_tag_name("parent"));
                    if ["groupId", "artifactId", "version"].iter().all(|key| {
                        field(p, key).is_some_and(|v| !v.contains('$'))
                            && field(p, key)
                                == field(pr, key).or_else(|| {
                                    (*key != "artifactId")
                                        .then(|| pp.and_then(|n| field(n, key)))
                                        .flatten()
                                })
                    }) {
                        out = visit(&parent_file, Some(repo), seen, bytes, depth + 1);
                    }
                }
            }
        }
    }
    let own = node_properties(root);
    for inherited_profile in &mut out.profiles {
        for key in own.keys() {
            inherited_profile.remove(key);
        }
    }
    out.properties.extend(own);
    for key in ["groupId", "artifactId", "version"] {
        if let Some(value) = field(root, key).or_else(|| parent.and_then(|n| field(n, key))) {
            out.properties
                .insert(format!("project.{key}"), value.into());
            out.properties.insert(format!("pom.{key}"), value.into());
        }
        if let Some(value) = parent.and_then(|n| field(n, key)) {
            for prefix in ["parent.", "project.parent.", "pom.parent."] {
                out.properties
                    .insert(format!("{prefix}{key}"), value.into());
            }
        }
    }
    for p in root
        .children()
        .filter(|n| n.has_tag_name("profiles"))
        .flat_map(|n| n.children().filter(|n| n.has_tag_name("profile")))
    {
        if out.profiles.len() >= 128 {
            out.limited = true;
            break;
        }
        let props = node_properties(p);
        if !props.is_empty() {
            out.profiles.push(props);
        }
    }
    out
}
