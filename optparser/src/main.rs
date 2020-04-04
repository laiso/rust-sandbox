use std::str::FromStr;
use std::collections::HashMap;

pub struct Mhead {
    formal: HashMap<String, String>
}

impl Mhead {
    fn parse() -> Self {
        return Mhead {
            formal: HashMap::new()
        }
    }
    fn string(name: &str, desc: &str) {

    }
    fn get_string(&self, name: &str) -> String {
        return String::from("result message")
    }
}


fn main() {
    Mhead::string("help", "this is help message aaa bbb.");

    let m = Mhead::parse();

    let mes = m.get_string("help");

    println!("{}", mes)
}
