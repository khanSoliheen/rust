use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
fn main() {
    let a = [6, 5, 4];
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    let mut v2 = vec![6, 7, 8]; // same as normal array
    let third_element = &v2[2];
    println!("Third element of vector normal array is: {}", third_element);
    match v.get(2) {
        Some(third) => println!(
            "This is the third index element using get method: {}",
            third
        ),
        None => println!("There is no third element"),
    }
    for item in &mut v2 {
        *item += 50;
        println!("The item after addition: {}", item);
    }
    let row = vec![
        // we can store different type of data in vector
        SpreadsheetCell::Int(32),
        SpreadsheetCell::Float(11.2),
        SpreadsheetCell::Text(String::from("Hello")),
    ];
    match &row[1] {
        SpreadsheetCell::Int(i) => println!("{}", i),
        _ => println!("Not an integer"),
    };
    // creating strings;
    let empty_string = String::new();
    let string_slice = "initail content";
    let mut s1 = string_slice.to_string();
    let mut s2 = String::from("another string");
    // appending to a string
    s2.push_str("appending to");
    s2.push('!');
    println!("{}", s2);
    let concat_string = s1 + &s2;
    println!("{}", concat_string);
    let format_appening = format!("{}, {}", string_slice, s2);
    println!("fortmat appending: {}", format_appening);
    for item in format_appening.as_bytes() {
        println!("bytes: {}", item);
    }
    for item in format_appening.chars() {
        println!("chars: {}", item);
    }
    for item in format_appening.graphemes(true) {
        println!("graphemes: {}", item);
    }
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");
    let mut hash_map = HashMap::new();
    hash_map.insert(blue, 10);
    hash_map.insert(yellow, 15);
    let team_name = String::from("Blue");
    let score = hash_map.get(&team_name);
    println!("score: {:#?}", score);
    for (key, value) in &hash_map {
        println!("key: {}, value: {}", key, value);
    }
    hash_map.entry(String::from("Yellow")).or_insert(30); // if doesnot exist add it or do nothing
    let text = "Hello world! from rust world!";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("map: {:#?}", map);
}
