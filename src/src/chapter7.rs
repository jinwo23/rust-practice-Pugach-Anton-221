fn test1() {
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }
}


fn test2() {
    let n = 5;

    let big_n = 
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            10 * n  // This is an integer
        } else {
            println!(", and is a big number, halve the number");

            n / 2  // Integer division, returns an integer
        };

    println!("{} -> {}", n, big_n);
}



fn test3() {
    for n in 1..100 { // This range excludes 100
        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
    }

    println!("Success!");
}



fn test4() {
    let names = [String::from("liming"), String::from("hanmeimei")];
    for name in &names { // Use a reference to avoid moving the elements
        // Do something with name...
    }

    println!("{:?}", names); // This works now because `names` was not moved

    let numbers = [1, 2, 3];
    // The elements in numbers are Copy, so there is no move here
    for n in numbers {
        // Do something with n...
    }

    println!("{:?}", numbers); // This works because the elements are `Copy`
}



fn test5() {
    let a = [4, 3, 2, 1];

    // Iterate the indexing and value in 'a'
    for (i, v) in a.iter().enumerate() {  // Use `.iter().enumerate()` here
        println!("The {}th element is {}", i + 1, v);
    }
}



fn test6() {
    // A counter variable
    let mut n = 1;

    // Loop while the condition is true
    while n <= 10 {  // Fill in the condition here
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        n += 1;  // Increment the counter to progress the loop
    }

    println!("n reached {}, so loop is over", n);
}



fn test7() {
    let mut n = 0;
    for i in 0..=100 {
       if n == 66 {
           break;  // Break the loop when n reaches 66
       }
       n += 1;
    }

    assert_eq!(n, 66);

    println!("Success!");
}



fn test8() {
    let mut n = 0;
    for i in 0..=100 {
       if n != 66 {
           n += 1;
           continue;  // Skip the remaining code and go to the next iteration
       }
       
       // This part will only be executed when n == 66
       break;  // Exit the loop when n == 66
    }

    assert_eq!(n, 66);

    println!("Success!");
}



fn test9() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;  // Skip the rest of this iteration and continue to the next one
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            break;  // Exit the loop when count reaches 5
        }
    }

    assert_eq!(count, 5);

    println!("Success!");
}



fn test10() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break 20;  // Return 20 when counter reaches 10
        }
    };

    assert_eq!(result, 20);

    println!("Success!");
}



fn test11() {
    let mut count = 0;
    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                // This would break only the inner1 loop
                break 'inner1; // `break` works here
            }
            count += 2;
        }

        count += 5;

        'inner2: loop {
            if count >= 30 {
                // This breaks the outer loop
                break 'outer;
            }

            // This will continue the outer loop
            continue 'outer;
        }
    }

    assert!(count == 30);  // The final value of `count` should be 30

    println!("Success!");
}
