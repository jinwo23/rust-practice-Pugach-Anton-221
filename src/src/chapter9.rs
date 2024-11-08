// Define the `Point` struct
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // Associated function to create a Point at the origin (0.0, 0.0)
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Associated function to create a new Point with given x and y coordinates
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

// Define the `Rectangle` struct that holds two `Point` structs
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // Method to calculate the area of the rectangle
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    // Method to calculate the perimeter of the rectangle
    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // Method to translate the rectangle by x and y
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// Define the `Pair` struct which holds two integers inside `Box<i32>`
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // Method to "consume" the `Pair` and print its contents
    fn destroy(self) {
        let Pair(first, second) = self;
        println!("Destroying Pair({}, {})", first, second);
    }
}

fn test0() {
    let mut rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Mutable object can call mutable methods
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();

    // Uncommenting this line would cause an error, as the pair has been moved
    // pair.destroy();  // Error: pair has been moved
}



struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Complete the area method to return the area of the Rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn test1() {
    let rect1 = Rectangle { width: 30, height: 50 };

    assert_eq!(rect1.area(), 1500); // Assert that the area is correct

    println!("Success!");
}



#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    pub fn show_state(&self)  {  // Borrow the instance with &self
        println!("the current state is {}", self.color);
    }
}

fn test2() {
    let light = TrafficLight {
        color: "red".to_owned(),
    };
    
    // Don't take the ownership of `light` here.
    light.show_state();  // This will work because we're borrowing `light`
    
    // ... Otherwise, there will be an error below
    println!("{:?}", light);  // This will print `light` after borrowing it
}





struct TrafficLight {
    color: String,
}

impl TrafficLight {
    // Using `Self` to fill in the blank.
    pub fn show_state(self: &Self) {  // `&Self` is the full form of `&self`
        println!("the current state is {}", self.color);
    }

    // Fill in the blank, DON'T use any variants of `Self`.
    pub fn change_state(&mut self) {  // We need a mutable reference here since we modify `self.color`
        self.color = "green".to_string();
    }
}

fn test3() {
    let mut light = TrafficLight {
        color: "red".to_string(),
    };

    light.show_state();  // Prints the current state
    light.change_state(); // Changes the state to "green"
    light.show_state();  // Prints the updated state

    println!("Success!");
}



#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    // 1. Implement an associated function `new`,
    // 2. It will return a TrafficLight contains color "red"
    // 3. Must use `Self`, DONT use `TrafficLight` in fn signatures or body
    pub fn new() -> Self {
        Self {
            color: "red".to_string(),
        }
    }

    pub fn get_state(&self) -> &str {
        &self.color
    }
}

fn test4() {
    let light = TrafficLight::new();
    assert_eq!(light.get_state(), "red");

    println!("Success!");
}




struct Rectangle {
    width: u32,
    height: u32,
}

// First impl block for area-related methods
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// Second impl block for other utility methods
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn test5() {
    println!("Success!");
}




#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

// Implement TrafficLightColor with a method.
impl TrafficLightColor {
    // The method returns a string representation of the color
    pub fn color(&self) -> &str {
        match self {
            TrafficLightColor::Red => "red",
            TrafficLightColor::Yellow => "yellow",
            TrafficLightColor::Green => "green",
        }
    }
}

fn test6() {
    let c = TrafficLightColor::Yellow;

    // Test the color method
    assert_eq!(c.color(), "yellow");

    // Print the enum variant
    println!("{:?}", c);
}
