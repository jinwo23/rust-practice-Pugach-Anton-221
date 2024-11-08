Chapter 5.1
  
fn test1() {
    let x = String::from("Hello world");
    let y = x.clone();  // Clone `x` to create a separate copy for `y`
    
    println!("{}, {}", x, y); // Now both x and y can be used
}



fn test2() {
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);

    println!("{}", s2); // `s2` holds the ownership of the String
}

fn take_ownership(s: String) -> String {  // Return ownership of the string
    println!("{}", s);
    s  // Return the ownership back to `main`
}



fn test3() {
    let s = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("Hello world");
    let _s = s.clone().into_bytes(); // Use `clone` to avoid consuming `s`
    s // Return the original `String`
}




fn test4() {
    let s = String::from("Hello World");

    print_str(&s);  // Borrow `s` by reference

    println!("{}", s);  // `s` is still valid here
}

fn print_str(s: &String) {  // Accept a reference to the String
    println!("{}", s);  // We can print it without taking ownership
}



fn test5() {
    let x = (1, 2, (), 'h');  // We replace String with a char, which implements Copy
    let y = x;  // Now we can directly copy the tuple because all its elements implement Copy
    println!("{:?}, {:?}", x, y);
}



fn test6() {
    let mut s = String::from("Hello ");  // Make `s` mutable
    
    let mut s1 = s;  // Also make `s1` mutable to allow mutation

    s1.push_str("World!");  // Now we can mutate `s1`

    println!("{}", s1);  // Print the modified string

    println!("Success!");
}



fn test7() {
    let x = Box::new(5);
    
    let mut y = x; // Make `y` mutable, so we can modify the value it points to
    
    *y = 4; // Dereference and assign a new value
    
    assert_eq!(*y, 4); // The value in `y` is now 4

    println!("Success!");
}



fn test0() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` is moved out of person, but `age` is referenced
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error! borrow of partially moved value: `person` partial move occurs
    //println!("The person struct is {:?}", person); // This line will give an error!

    // `person` cannot be used but `person.age` can be used as it is not moved
    println!("The person's age from person struct is {}", person.age);
}



fn test8() {
    let t = (String::from("hello"), String::from("world"));

    let _s = &t.0; // Borrow the value instead of moving it

    // Now you can print the entire tuple `t` without any issues
    println!("{:?}", t);
}




fn test9() {
    let t = (String::from("hello"), String::from("world"));

    // fill the blanks
    let (s1, s2) = t.clone();

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}



Chapter 5.2



fn test1() {
   let x = 5;
   // Create a reference to `x`
   let p = &x;  // This creates a reference to `x`

   println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
}


fn test2() {
    let x = 5;
    let y = &x;

    // Dereference `y` to get the value it points to and compare it with 5
    assert_eq!(5, *y);

    println!("Success!");
}



fn test3() {
    let mut s = String::from("hello, ");

    // Pass a reference to `s` to `borrow_object`
    borrow_object(&s);

    println!("Success!");
}

fn borrow_object(s: &String) {
    // Function takes a reference to `s`, no ownership transfer
}



fn test4() {
    let mut s = String::from("hello, ");

    // Pass a mutable reference to `s`
    push_str(&mut s);

    println!("Success! {}", s);
}

fn push_str(s: &mut String) {
    s.push_str("world"); // Modify the String by appending "world"
}



fn test5() {
    let mut s = String::from("hello, ");

    // Create a mutable reference to `s` so that it can be modified
    let p = &mut s;
    
    p.push_str("world");

    println!("Success! {}", s);
}



fn test6() {
    let s = String::from("hello");

    // Pass by reference using ref in the function signature
    print_ref(s);
}

fn print_ref(ref s: String) {
    // This is a reference to the value passed
    println!("The value is: {}", s);
}



fn test7() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    // Removed r2 to ensure we only have one mutable reference at a time.
    // let r2 = &mut s;

    println!("{}", r1);  // Only print r1

    println!("Success!");
}



fn test8() {
    // Modify this line to make `s` mutable
    let mut s = String::from("hello, ");

    borrow_object(&mut s);  // Pass a mutable reference

    println!("Success!");
}

fn borrow_object(s: &mut String) {
    // The function can now modify `s` if necessary
}



fn test9() {
    let mut s = String::from("hello, ");

    // Borrow `s` as a mutable reference to allow modification inside the function
    borrow_object(&mut s);
    
    // `s` has been modified inside the function
    println!("Modified string: {}", s); // This will print "Modified string: hello, world"
}

// Function that borrows the string as a mutable reference and modifies it
fn borrow_object(s: &mut String) {
    s.push_str("world");
}




fn test10() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");

    // Comment this line to avoid having two mutable references at the same time
    // let r2 = &mut s;

    // This works because `r1` is no longer in use after `r1.push_str()`
    println!("{}", r1); // This will print "hello, world"
}




fn test11() {
    let mut s = String::from("hello, ");

    {
        let r1 = &mut s; // First mutable borrow
        r1.push_str("world");
    } // r1 is dropped here

    let r2 = &mut s; // Now we can create r2 after r1 goes out of scope
    r2.push_str("!");

    println!("{}", r2); // Now r2 is used here
}




