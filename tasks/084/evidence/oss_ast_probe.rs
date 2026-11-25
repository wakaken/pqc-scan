//! Synthetic parser observations only; no target source or code execution.
fn main() {
    for (id,language,source) in [
        ("cpp_nested_direct_initialization",tree_sitter_cpp::LANGUAGE.into(),"void f(){ UniquePtr<EVP_PKEY_CTX> ctx(EVP_PKEY_CTX_new_id(EVP_PKEY_RSA, nullptr)); }"),
        ("cpp_assignment_control",tree_sitter_cpp::LANGUAGE.into(),"void f(){ auto ctx = EVP_PKEY_CTX_new_id(EVP_PKEY_RSA, nullptr); }"),
        ("csharp_same_named_property",tree_sitter_c_sharp::LANGUAGE.into(),"using System.Security.Cryptography; class C { private RSA RSA {get;set;} void M(){var rsa=RSA.Create();} }"),
    ] {
        let mut parser=tree_sitter::Parser::new();parser.set_language(&language).unwrap();let tree=parser.parse(source,None).unwrap();println!("{id}: {}",tree.root_node().to_sexp());
    }
}
