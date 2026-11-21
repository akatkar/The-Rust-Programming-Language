use std::collections::HashMap;

fn main() {
    let field_name = String::from("Favorite Color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 25);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("Score of {} is {:?}", team_name, score);

    // Access all entries of a hashmap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Overwrite score of Blue
    scores.insert(String::from("Blue"), 15);
    println!("Score of {} is {:?}", team_name, scores.get(&team_name));

    // Insert only if absent
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Counting words 
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
