extern crate tera;

use tera::Tera;
use tera::Context;
use std::fs;
use std::path::Path;
use regex::{Regex};

pub mod subject;

fn main() {
    let path = Path::new("src/subject.rs");

    let re = Regex::new(r"pub fn (?P<fname>.*)\(").unwrap();
    let source = fs::read_to_string(path).expect("read error");
    for line in source.lines() {
        for caps in re.captures_iter(&line) {
            let fname = &caps["fname"];
            let code = render(fname);
            let path = "tests/subject.rs";
            let _ = fs::write(path, code.as_str());
            println!("{}", code);
        }
    }
}

fn render(fname: &str) -> String {
    let tera = match Tera::new("*.tmpl") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let mut ctx = Context::new();
    ctx.insert("test_name", fname);
    let result = match tera.render("test.tmpl", &ctx) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    return result;
}