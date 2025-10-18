use std::io;
fn main() {
    let mut height = 190;
    height = height - 20;
    let result = if height > 180 {
        "Tall"
    } else if height > 170 {
        "Average"
    } else {
        "Short"
    };

    println!("Result: {}", result);

    let health = if height < 180 { "good" } else { "unknown" };
    println!("Health: {}", health);

    // shadowing to a different type
    let _health = if height < 180 { true } else { false };
    // we're able to do this because we're declaring the variable again

    // loops
    let mut x = 1;

    loop {
        println!("x is {}", x);
        x += 1;
        if x > 5 {
            break;
        }
    }

    // Option and Some
    let maybe_number: Option<Option<()>> = Some(None);
    // let maybe_number = Some(42);
    if let Some(number) = maybe_number {
        println!("The number is {:?}", number);
    } else {
        println!("There is no number");
    }
    // this Some() and Option is still unclear to me

    // while
    let mut i = 0;
    while i < 5 {
        println!("i = {}", i);
        i += 1;
    }

    let mut input = String::new();
    while input.trim() != "stop" {
        input.clear();
        println!("Please enter a word (type 'stop' to exit):");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        println!("You entered: {}", input);
    }
    println!("Goodbye!");

    //for loop
    for i in 1..10 {
        println!("i = {}", i);
    }
    for i in (1..=5).rev() {
        println!("{}", i);
    }

    let numbers = vec![1, 2, 3, 4, 5];
    for n in numbers {
        println!("{}", n);
    }

    // breaks and continues

    for i in 1..=10 {
        if i % 2 == 0 {
            continue;
        }
        println!("i {}", i);
        if i == 7 {
            break;
        }
    }

    // match
    // let name = "Hello";
    println!("Please enter a greeting:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");

    match name.to_lowercase().trim() {
        "good bye" => println!("Sorry to see you go."),
        "hello" => println!("Hi, nice to meet you!"),
        //Catch all
        _ => println!("I can't find a greeting, good bye"),
    }
}
