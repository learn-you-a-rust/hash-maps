# hash-maps
https://doc.rust-lang.org/stable/book/ch08-03-hash-maps.html

Values implementing `Copy` (ex. `i32`) are copied into a hash map. Others are moved into the hash map and the hash map becomes 
the owner.

You can also move refs into a hash map, but those have to stay valid for at least as long as the hash map is valid. (See 'lifetimes'.)

The `or_insert` method returns a mutable reference `(&mut V)` to the value for a specified key.
