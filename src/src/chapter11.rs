Chapter11.1


fn test1() {
    let mut s: String = String::from("hello, ");
    s.push_str("world");
    s.push('!');

    move_ownership(&s); // Borrow `s` by passing a reference

    assert_eq!(s, "hello, world!");

    println!("Success!");
}

fn move_ownership(s: &String) { // Change parameter to borrow the String
    println!("ownership of \"{}\" is moved here!", s);
}



fn test2() {  
   let mut s = String::from("hello, world");

   let slice1: &str = &s; // In two ways: either &s or &s[..]
   assert_eq!(slice1, "hello, world");

   let slice2 = &s[..5]; // Slice the first 5 characters
   assert_eq!(slice2, "hello");

   let slice3: &mut String = &mut s; // We need a mutable reference to modify
   slice3.push('!'); // Modify the original String
   assert_eq!(slice3, "hello, world!");

   println!("Success!");
}



fn test3() {  
    // Create a String type based on `&str`
    // The type of string literals is `&str`
    let s: String = String::from("hello, world!");

    // Create a slice pointing to String `s`
    let slice: &str = &s;

    // Create a String type based on the recently created slice
    let s: String = slice.to_string();

    assert_eq!(s, "hello, world!");

    println!("Success!");
}



fn test4() {
    let s = String::from("hello, 世界");
    
    // `s[0..1]` correctly gets the first character "h"
    let slice1 = &s[0..1]; // `h` takes 1 byte in UTF-8 format
    assert_eq!(slice1, "h");

    // We need to work with characters directly, not byte indices.
    let slice2 = &s[7..10]; // This is a valid slice for the character "世"
    assert_eq!(slice2, "世");

    // Iterate through all chars in s using `char_indices`
    for (i, c) in s.char_indices() { // Use `char_indices` to get char positions
        if i == 7 {
            assert_eq!(c, '世');
        }
    }

    println!("Success!");
}



fn test5() {
    let mut s = String::new();
    s.push_str("hello"); // Fill in: Initialize `s` with the string "hello"

    // Some bytes, in a vector
    let v = vec![104, 101, 108, 108, 111]; // Represents the bytes for "hello"

    // Turn a byte's vector into a String
    let s1 = String::from_utf8(v).unwrap(); // Fill in: Convert the byte vector into a String
    
    assert_eq!(s, s1); // Assert that both strings are equal

    println!("Success!");
}



fn test6() {
    let mut s = String::with_capacity(25); // Set the capacity to 25 to avoid reallocation

    println!("{}", s.capacity()); // This will print the initial capacity: 25

    for _ in 0..2 {
        s.push_str("hello"); // Adds 5 characters each time, without triggering reallocation
        println!("{}", s.capacity()); // Capacity should remain 25 after each push
    }

    println!("Success!");
}



use std::mem;

fn test7() {
    let story = String::from("Rust By Practice");

    // Prevent automatically dropping of the String's data
    let mut story = mem::ManuallyDrop::new(story);

    let ptr = story.as_ptr(); // Get a pointer to the string's data
    let len = story.len();    // Get the length of the string
    let capacity = story.capacity(); // Get the capacity of the string

    assert_eq!(16, len);

    // We can rebuild a String out of ptr, len, and capacity. This is all
    // unsafe because we are responsible for making sure the components are
    // valid:
    let s = unsafe { String::from_raw_parts(ptr as *mut u8, len, capacity) };

    assert_eq!(*story, s);

    println!("Success!");
}


Chapter11.2


fn test1() {
    let arr: [u8; 3] = [1, 2, 3];
    
    // Convert array to vector
    let v = Vec::from(arr);
    is_vec(&v);

    let v = vec![1, 2, 3];
    is_vec(&v);

    // Use the vec! macro (same as vec![..])
    let v = vec!(1, 2, 3);
    is_vec(&v);
    
    // Use Vec::new() and a for loop to rewrite the code
    let mut v1 = Vec::new();
    for &x in &arr {
        v1.push(x);
    }
    is_vec(&v1);

    // Check that both vectors are equal
    assert_eq!(v, v1);

    println!("Success!");
}

// Function to check and print the vector contents
fn is_vec(v: &Vec<u8>) {
    // For simplicity, just print the contents of the vector
    println!("{:?}", v);
}




fn test2() {
    let mut v1 = Vec::from([1, 2, 4]);
    v1.pop(); // Removes the last element (4)
    v1.push(3); // Adds 3 to the end of the vector

    let mut v2 = Vec::new();
    v2.extend(&v1); // Extends v2 with the elements from v1

    assert_eq!(v1, v2); // Ensures v1 and v2 are equal

    println!("Success!");
}




fn test3() {
    // Array -> Vec
    // impl From<[T; N]> for Vec
    let arr = [1, 2, 3];
    let v1 = Vec::from(arr); // Converts array to Vec
    let v2: Vec<i32> = arr.to_vec(); // Converts array to Vec using to_vec method
 
    assert_eq!(v1, v2);
    
    // String -> Vec
    // impl From<String> for Vec
    let s = "hello".to_string();
    let v1: Vec<u8> = s.into_bytes(); // Converts String to Vec<u8> (byte representation)

    let s = "hello".to_string();
    let v2 = s.into_bytes(); // Another way to convert String to Vec<u8>
    assert_eq!(v1, v2);

    // impl<'_> From<&'_ str> for Vec
    let s = "hello";
    let v3 = Vec::from(s.as_bytes()); // Converts &str to Vec<u8>
    assert_eq!(v2, v3);

    // Iterators can be collected into vectors
    let v4: Vec<i32> = [0; 10].into_iter().collect(); // Collects elements of the iterator into a Vec
    assert_eq!(v4, vec![0; 10]);

    println!("Success!");
}



fn test4() {
    let mut v = Vec::from([1, 2, 3]);

    // First loop, just print the existing values
    for i in 0..v.len() {
        println!("{:?}", v[i]);  // This will print the current values in the vector
    }

    // Second loop, add 1 to each element of the vector
    for i in 0..v.len() {
        v[i] += 1;  // Increment each element by 1
    }
    
    // Assert the final state of the vector
    assert_eq!(v, vec![2, 3, 4]);

    println!("Success!");
}



fn test5() {
    let mut v = vec![1, 2, 3];

    let slice1 = &v[..]; // Get the whole vector as a slice
    let slice2 = &v[0..v.len()]; // Use v.len() to safely slice the vector (no out of bounds error)
    
    assert_eq!(slice1, slice2);

    // Slices are read-only
    let vec_ref: &mut Vec<i32> = &mut v;
    (*vec_ref).push(4); // Push 4 into the vector
    
    // Create a mutable slice (up to index 3)
    let slice3 = &mut v[0..3];
    slice3[0] = 10; // Modify the first element of the slice

    assert_eq!(slice3, &[10, 2, 3]); // Now the slice reflects the updated value

    println!("Success!");
}



fn test6() {
    let mut vec = Vec::with_capacity(10);

    // The vector contains no items, even though it has capacity for more
    assert_eq!(vec.len(), 0);  // Length is 0 because we haven't added anything yet
    assert_eq!(vec.capacity(), 10);  // Capacity is 10 as specified

    // These are all done without reallocating...
    for i in 0..10 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10);  // Length is now 10 after pushing 10 elements
    assert_eq!(vec.capacity(), 10);  // Capacity remains 10, no reallocation

    // ...but this may make the vector reallocate
    vec.push(11);
    assert_eq!(vec.len(), 11);  // Length is now 11
    assert!(vec.capacity() >= 11);  // Capacity is now greater than or equal to 11 (likely doubled)

    // Fill in an appropriate value to make the `for` done without reallocating
    let mut vec = Vec::with_capacity(100);  // Initial capacity of 100 ensures no reallocation
    for i in 0..100 {
        vec.push(i);
    }

    assert_eq!(vec.len(), 100);  // Length should be 100 after pushing 100 elements
    assert_eq!(vec.capacity(), 100);  // Capacity should remain 100 as no reallocation occurred
    
    println!("Success!");
}



#[derive(Debug, PartialEq)] // Derive PartialEq for comparison
enum IpAddr {
    V4(String),
    V6(String),
}

fn test7() {
    // Initialize a vector containing distinct types of IpAddr enum
    let v: Vec<IpAddr> = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string()),
    ];

    // Comparing two enums, which now derive PartialEq
    assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
    assert_eq!(v[1], IpAddr::V6("::1".to_string()));

    println!("Success!");
}



trait IpAddr {
    fn display(&self);
}

struct V4(String);
impl IpAddr for V4 {
    fn display(&self) {
        println!("ipv4: {:?}", self.0);
    }
}

struct V6(String);
impl IpAddr for V6 {
    fn display(&self) {
        println!("ipv6: {:?}", self.0);
    }
}

fn test8() {
    // The vector holds trait objects of type Box<dyn IpAddr>
    let v: Vec<Box<dyn IpAddr>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display();  // Call the `display` method on each trait object
    }
}


Chapter11.3


use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Score {
    Integer(i32),
    Float(f64),
    String(String),
}

fn test1() {
    let mut scores = HashMap::new();
    scores.insert("Sunface", Score::Integer(98));
    scores.insert("Daniel", Score::Integer(95));
    scores.insert("Ashley", Score::Float(69.0));  // Use Score::Float to unify types
    scores.insert("Katie", Score::String("58".to_string()));  // Use Score::String to unify types

    // Get returns an Option<&V>
    let score = scores.get("Sunface");
    assert_eq!(score, Some(&Score::Integer(98)));  // Compare with Score::Integer

    if scores.contains_key("Daniel") {
        // Indexing returns a value V
        let score = &scores["Daniel"];
        assert_eq!(score, &Score::Integer(95));  // Compare with Score::Integer
        scores.remove("Daniel");
    }

    assert_eq!(scores.len(), 3);

    for (name, score) in &scores {
        println!("The score of {} is {:?}", name, score);
    }
}



use std::collections::HashMap;

fn test2() {
    let teams = [
        ("Chinese Team", 100),
        ("American Team", 10),
        ("France Team", 50),
    ];

    let mut teams_map1 = HashMap::new();
    for team in &teams {
        teams_map1.insert(team.0, team.1);
    }

    // IMPLEMENT team_map2 in two ways
    // 1. Using `collect()` to transform the array of tuples into a HashMap
    let teams_map2: HashMap<_, _> = teams.iter().cloned().collect();

    // 2. Using `collect()` again, but starting from a different approach
    let teams_map3: HashMap<_, _> = teams.iter().map(|&(k, v)| (k, v)).collect();

    assert_eq!(teams_map1, teams_map2);
    assert_eq!(teams_map1, teams_map3);

    println!("Success!");
}




use std::collections::HashMap;

fn test3() {
    // Type inference lets us omit an explicit type signature (which
    // would be `HashMap<&str, u8>` in this example).
    let mut player_stats = HashMap::new();

    // Insert a key only if it doesn't already exist
    player_stats.entry("health").or_insert(100);

    assert_eq!(player_stats["health"], 100);  // `health` should have the value 100 initially

    // Insert a key using a function that provides a new value only if it
    // doesn't already exist
    player_stats.entry("health").or_insert_with(random_stat_buff);
    assert_eq!(player_stats["health"], 100);  // Value doesn't change because "health" already exists

    // Ensures a value is in the entry by inserting the default if empty, and returns
    // a mutable reference to the value in the entry.
    let health = player_stats.entry("health").or_insert(50);
    assert_eq!(*health, 100);  // Value should still be 100
    *health -= 50;
    assert_eq!(*health, 50);  // After decreasing by 50, the value should be 50

    println!("Success!");
}

fn random_stat_buff() -> u8 {
    // Could actually return some random value here - let's just return
    // some fixed value for now
    42
}




use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Eq, PartialEq, Hash)]  // Deriving the required traits
struct Viking {
    name: String,
    country: String,
}

impl Viking {
    /// Creates a new Viking.
    fn new(name: &str, country: &str) -> Viking {
        Viking {
            name: name.to_string(),
            country: country.to_string(),
        }
    }
}

fn test4() {
    // Use a HashMap to store the vikings' health points.
    let vikings = HashMap::from([
        (Viking::new("Einar", "Norway"), 25),
        (Viking::new("Olaf", "Denmark"), 24),
        (Viking::new("Harald", "Iceland"), 12),
    ]);

    // Use derived implementation to print the status of the vikings.
    for (viking, health) in &vikings {
        println!("{:?} has {} hp", viking, health);
    }
}




use std::collections::HashMap;

fn test0() {
    // Create a HashMap with a starting capacity of 100
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);
    
    // Assert that the capacity of the map is at least 100
    assert!(map.capacity() >= 100);

    // Shrinks the capacity of the map to fit its current size
    map.shrink_to_fit();
    assert!(map.capacity() >= 2); // The capacity will not shrink below the number of elements

    println!("Success!");
}



use std::collections::HashMap;

fn test5() {
    let v1 = 10;
    let mut m1 = HashMap::new();
    m1.insert(v1, v1);
    println!("v1 is still usable after inserting to hashmap : {}", v1);

    let v2 = "hello".to_string();
    let mut m2 = HashMap::new();
    // Ownership moved here, so we clone v2
    m2.insert(v2.clone(), v1);

    // v2 is still usable after cloning
    assert_eq!(v2, "hello");

    println!("Success!");
}



