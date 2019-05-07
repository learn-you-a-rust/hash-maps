// hash maps are not automatically included like strings and vectors are
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    // you can also combine data into a hash map
    let team = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // this makes the hash map a table of refs, i think
    let mut scores: HashMap<_, _> = team.iter().zip(initial_scores.iter()).collect();

    // ownership of hash map data
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new(); // must be mutable to get updated
    map.insert(field_name, field_value); // you can't use these vars anymore now

    //println!("Field value was {}", field_value); // nope
    //println!("Field value was {}", &field_value); // this doesn't work either
    
    let other_field_name = String::from("Favorite animal");
    let other_field_value = String::from("Cat");

    // you can also add references to a hash map, and that works, but you
    // have to consider lifetimes
    let mut other_map = HashMap::new();
    other_map.insert(&other_field_name, &other_field_value);

    // accessing values in a hash map
    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // score = Some(&10);

    // iterate over key/value pairs (in arbitrary order);
    // using a ref to scores means you can use scores later
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // you can do this, but scores has to be mutable (see above);
    // this expects key, values as refs, probably because of the zip()
    scores.insert(&String::from("Red"), &56);

    // this works and doesn't take refs as key, values
    map.insert(String::from("Favorite food"), String::from("Pizza"));

    // this works
    let mut scores_nonzip = HashMap::new();
    scores_nonzip.insert(String::from("Blue"), 10);
    scores_nonzip.insert(String::from("Yellow"), 50);


    // adding values to a hash map

    // overwrite old value
    scores_nonzip.insert(String::from("Blue"), 25);
    println!("{:?}", scores_nonzip);

    // add value only if no value exists for key
    let mut scores_entry = HashMap::new();
    scores_entry.insert(String::from("Blue"), 10);
    
    scores_entry.entry(String::from("Yellow")).or_insert(500);
    scores_entry.entry(String::from("Blue")).or_insert(500);
    println!("{:?}", scores_entry);

    // update value based on old value: word count
    let text = "hello world wonderful world";
    let mut wordmap = HashMap::new();

    for word in text.split_whitespace() {
        let count = wordmap.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("{:?}", wordmap);
}
