use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    
    // All keys must be of the same type;
    // the same goes for all values
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // You can also create a Hash Map from
    // a vector of tuples
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec! [10, 50];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // Both field_name and field_value are
    // invalid after this point; they
    // have been moved, since String does
    // not implement the copy trait

    // Accessing values in a hash map
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    // Iterating over key/value pairs in
    // a hash map
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
