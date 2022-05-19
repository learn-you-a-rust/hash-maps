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
}
