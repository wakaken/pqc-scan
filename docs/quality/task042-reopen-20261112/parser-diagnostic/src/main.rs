use tree_sitter::{Node,Parser};
fn dump(node:Node,text:&str,depth:usize) {
 assert!(depth<1000);
 println!("node {} {}..{} {:?}",node.kind(),node.start_byte(),node.end_byte(),node.utf8_text(text.as_bytes()).unwrap());
 let mut c=node.walk();for child in node.children(&mut c){dump(child,text,depth+1);}
}
fn main(){for name in std::env::args().skip(1){
 let text=std::fs::read_to_string(&name).unwrap();assert!(text.len()<=4096);
 let lang=if name.ends_with(".java"){tree_sitter_java::LANGUAGE}else{tree_sitter_python::LANGUAGE};
 let mut parser=Parser::new();parser.set_language(&lang.into()).unwrap();
 let tree=parser.parse(&text,None).unwrap();println!("input {name} error={} {}",tree.root_node().has_error(),tree.root_node().to_sexp());dump(tree.root_node(),&text,0);
}}
