use std::{collections::HashMap, fs};

fn main() {
    let data = fs::read_to_string("data/map.toml").unwrap();

    let res: HashMap<String, String> = toml::from_str(data.as_str()).unwrap();
    println!("{:#?}", res);
}
