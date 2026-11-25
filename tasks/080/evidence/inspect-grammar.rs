use std::{env, fs};
fn main() {
 let a: Vec<_> = env::args().collect();
 let text = fs::read_to_string(&a[2]).unwrap();
 let language = if a[1] == "cpp" { tree_sitter_cpp::LANGUAGE.into() } else { tree_sitter_c::LANGUAGE.into() };
 let mut parser=tree_sitter::Parser::new();parser.set_language(&language).unwrap();
 let tree=parser.parse(&text,None).unwrap();println!("{}",tree.root_node().to_sexp());
}
