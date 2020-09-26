use std::{collections::HashMap, fs};

use serde::Deserialize;
use toml::value::Datetime;

#[derive(Deserialize, Debug)]
struct Data {
    name: String,
    group: String,
    link: Option<String>,
    description: String,
    created_at: Datetime,
}

#[derive(Deserialize, Debug)]
struct DataRoot {
    data: Vec<Data>,
    group_links: HashMap<String, String>
}


fn main() {
    let data = fs::read_to_string("data/i-use-it.toml").unwrap();


    let res: DataRoot = toml::from_str(data.as_str()).unwrap();
    println!("{:#?}", res);
}