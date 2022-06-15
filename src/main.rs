use rand::seq::SliceRandom;
use serde_json;
use serde_json::Value;
use std::fs;
use textwrap;

fn main() {
    let file = fs::read_to_string("quotes.json").expect("Provide a valid file");
    let json_data: Value = serde_json::from_str(&file).unwrap();
    let arr = match json_data {
        Value::Array(vec_data) => vec_data,
        _ => unreachable!(),
    };
    let num_lines = 60;
    let rand_obj = arr.choose(&mut rand::thread_rng());
    let text = &rand_obj.unwrap()["text"].to_owned().to_string();
    let text = textwrap::wrap(text, num_lines);

    for i in &text {
        println!("{}", i);
    }
    println!("\n");
    println!("- {}", &rand_obj.unwrap()["author"]);
}
