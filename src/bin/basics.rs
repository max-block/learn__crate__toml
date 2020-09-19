use std::fs;

use serde::Deserialize;
use toml::value::Datetime;

#[derive(Deserialize, Debug)]
struct Data {
    name: String,
    tags: Vec<String>,
    link: String,
    description: String,
    created_at: Datetime,
    archived_at: Option<Datetime>
}

#[derive(Deserialize, Debug)]
struct DataRoot {
    data: Vec<Data>
}


fn main() {
    let data = fs::read_to_string("data/i-use-it.toml").unwrap();


    let res: DataRoot = toml::from_str(data.as_str()).unwrap();
    println!("{:#?}", res);
}