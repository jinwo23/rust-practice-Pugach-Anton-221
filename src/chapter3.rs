/// https://practice.course.rs/variables.html
#[test] 
fn test31() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let _y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}

#[test]

// Fill the blanks in the code to make it compile
fn test32() {
    let mut x = 1;
    x += 2; 
    
    assert_eq!(x, 3);
    println!("Success!");
}

#[test]


// Fix the error below with least amount of modification
fn test33() {
    let x: i32 = 10;
     let y: i32 = 5;
    {
        
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y); 
}

#[test]


// Fix the error with the use of define_x
fn test34()
{let x = "hello";
    println!("{}, world", x); 
}

fn define_x() {
    
}

#[test]


// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
fn test35() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Prints "42".
}

#[test]


// Remove a line in the code to make it compile
fn test36() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    let mut x = x; 
    x += 3;


    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!"; 

    println!("Success!");
}

#[test]


fn test37() {
    let mut x = 1; 
       println!("The value of x is: {}", x);
}

#[test]


// Fix the error below with least amount of modification
fn test38() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}

#[test]


fn test39() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], [3,2]);

    println!("Success!");
} 
