// hash maps are not automatically included like strings and vectors are
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // you can also combine data into a hash map
    let team = vec![String::from("Blue"), String::from("Yellow")];
    let intial_scores = vec![10, 50];

    let scores: HashMap<_, _> = team.iter().zip(initial_scores.iter()).collect();
}
