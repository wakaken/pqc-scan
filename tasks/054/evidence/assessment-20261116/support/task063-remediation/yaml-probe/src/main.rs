use serde::Deserialize;
use serde_json::{json, Value};
use std::{env, fs};
fn check(label: &str, text: &str) -> Value {
    let old: Result<Vec<old::Value>, _> = old::Deserializer::from_str(text).map(old::Value::deserialize).collect();
    let new: Result<Vec<new::Value>, _> = new::Deserializer::from_str(text).map(new::Value::deserialize).collect();
    let a = old.map(|v| format!("{:?}",v)).map_err(|e|e.to_string());
    let b = new.map(|v| format!("{:?}",v)).map_err(|e|e.to_string());
    json!({"label":label,"same_acceptance":a.is_ok()==b.is_ok(),"same_value_or_error":a==b,"old":a,"new":b})
}
fn main(){
 let mut results=Vec::new();
 for path in env::args().skip(1){let text=fs::read_to_string(&path).unwrap();results.push(check(&path,&text));}
 let cases=[("duplicate","a: 1\na: 2\n"),("alias","a: &x [1, 2]\nb: *x\n"),("tag","a: !custom 1\n"),("float","a: .nan\nb: .inf\nc: -0.0\n"),("number","a: 01\nb: 0x10\nc: 1e3\n"),("empty",""),("multi","---\na: 1\n---\na: 2\n"),("bad","a: [1\n"),("boolean","a: yes\nb: on\nc: true\n"),("utf8","a: 日本語\n"),("overflow","a: 18446744073709551616\n"),("merge","a: &x {k: 1}\nb: {<<: *x}\n")];
 for (name,text) in cases {results.push(check(name,text));}
 println!("{}",serde_json::to_string_pretty(&results).unwrap());
}
