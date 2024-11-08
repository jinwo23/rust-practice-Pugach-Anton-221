Chapter 8.1 

enum Direction {
    East,
    West,
    North,
    South,
}

fn test1() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::South | Direction::North => { // Matching South or North here
            println!("South or North");
        },
        _ => println!("Other direction"), // Catch-all case
    };
}



fn test2() {
    let boolean = true;

    // Match the boolean value and assign the corresponding binary value
    let binary = match boolean {
        true => 1,  // If boolean is true, set binary to 1
        false => 0, // If boolean is false, set binary to 0
    };

    assert_eq!(binary, 1);

    println!("Success!");
}



enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn test3() {
    let msgs = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];

    for msg in msgs {
        show_message(msg);
    }

    println!("Success!");
}

fn show_message(msg: Message) {
    match msg {
        Message::Move { x: a, y: b } => { // Match Message::Move and destructure x and y
            assert_eq!(a, 1);
            assert_eq!(b, 3);
        },
        Message::ChangeColor(_, g, b) => { // Match Message::ChangeColor and destructure the colors
            assert_eq!(g, 255);
            assert_eq!(b, 0);
        },
        _ => println!("no data in these variants") // For the Quit and Write variants
    } // <-- Make sure this closing brace is here to close the match block.
} // <-- Make sure this closing brace is here to close the show_message function.




fn test4() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

    // Check if each character is an uppercase alphabet letter
    for ab in alphabets {
        // Skip non-uppercase letters
        if !matches!(ab, 'A'..='Z') {
            continue;  // Skip non-uppercase letters
        }

        assert!(matches!(ab, 'A'..='Z'));  // Matches if `ab` is between 'A' and 'Z'
    }

    println!("Success!");
}




#[derive(PartialEq)] // Derive the PartialEq trait to enable comparison
enum MyEnum {
    Foo,
    Bar,
}

fn test5() {
    let mut count = 0;

    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    for e in v {
        if e == MyEnum::Foo { // Now it works because PartialEq is implemented
            count += 1;
        }
    }

    assert_eq!(count, 2);

    println!("Success!");
}




fn test6() {
    let o = Some(7);

    // Using `if let` instead of `match`
    if let Some(i) = o {
        println!("This is a really long string and `{:?}`", i);
        println!("Success!");
    }
}



enum Foo {
    Bar(u8),
}

fn test7() {
    let a = Foo::Bar(1);

    if let Foo::Bar(i) = a {  // Use if let to match Foo::Bar and extract the value `i`
        println!("foobar holds the value: {}", i);
        println!("Success!");
    }
}




enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

fn test8() {
    let a = Foo::Qux(10);

    // Using `match` to match all variants of `Foo`
    match a {
        Foo::Bar => println!("match foo::bar"),
        Foo::Baz => println!("match foo::baz"),
        _ => println!("match others"),  // This will handle Foo::Qux or any other cases
    }
}




fn test9() {
    let age = Some(30);
    
    // Shadow the variable `age` within the if let scope
    if let Some(age) = age { 
        assert_eq!(age, 30); // This works because the `age` here is the value inside `Some(30)`
    } // The `age` in the `if let` block goes out of scope here
    
    // Now we use the outer `age`, which is still in scope
    match age {
        Some(age) => println!("age is a new variable, it's value is {}", age),
        _ => (),
    }
}



Chapter 8.2



fn test1() {
    match_number(3);  // Example call with 3
    match_number(7);  // Example call with 7
    match_number(15); // Example call with 15
}

fn match_number(n: i32) {
    match n {
        // Match a single value
        1 => println!("One!"),
        // Match several values using |
        2 | 3 | 4 | 5 => println!("match 2 -> 5"),
        // Match an inclusive range using ..=
        6..=10 => {
            println!("match 6 -> 10")
        },
        _ => {
            println!("match -infinite -> 0 or 11 -> +infinite")
        }
    }
}




struct Point {
    x: i32,
    y: i32,
}

fn test2() {
    // Fill in the blank to let p match the second arm
    let p = Point { x: 3, y: 20 }; // This will match the second arm with `y@ (10 | 20 | 30)`

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        // Second arm: match y values 10, 20, or 30 and capture the value of `y` into `y`
        Point { x: 0..=5, y: y@ (10 | 20 | 30) } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}



enum Message {
    Hello { id: i32 },
}

fn test3() {
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id:  id@3..=7,
        } => println!("Found an id in range [3, 7]: {}", id),
        Message::Hello { id: newid@(10 | 11 | 12) } => {
            println!("Found an id in another range [10, 12]: {}", newid)
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}



fn test4() {
    let num = Some(4);
    let split = 5;
    match num {
        Some(x) if x < split => assert!(x < split), // Match guard added here
        Some(x) => assert!(x >= split),
        None => (),
    }

    println!("Success!");
}




fn test5() {
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

    match numbers {
        (first, _, _, _, _, _, _, _, _, _, last) => {
            assert_eq!(first, 2);
            assert_eq!(last, 2048);
        }
    }

    println!("Success!");
}


fn test6() {
    let mut v = String::from("hello,");
    let r = &mut v;

    match r {
       value => value.push_str(" world!") // no &mut here, just use the mutable reference directly
    }

    println!("{}", v); // This will print "hello, world!"
}
