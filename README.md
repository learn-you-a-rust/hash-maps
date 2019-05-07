# hash-maps
https://doc.rust-lang.org/stable/book/ch08-03-hash-maps.html

Values implementing `Copy` (ex. `i32`) are copied into a hash map. Others are moved into the hash map and the hash map becomes 
the owner.
