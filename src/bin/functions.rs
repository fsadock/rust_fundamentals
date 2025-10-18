// to run use: cargo run --bin functions

fn process_numbers(numbers: &[i32]) {
    let mut sum = 0;

    for number in numbers {
        sum += number;
    }
    println!("Sum: {}", sum);

    if sum % 2 == 0 {
        println!("The sum is even");
    } else {
        println!("The sum is odd");
    }
}

// see more on .collect()
fn split_string(s: String, delimiter: char, field: usize) -> String {
    let parts: Vec<&str> = s.split(delimiter).collect();
    let result = parts.get(field);

    result.expect("Oops! something went wrong").to_string()
}

fn sum(numbers: &[i32]) -> i32 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result
}

fn main() {
    process_numbers(&[1, 2, 3]);
    let chunk = split_string("hello,world".to_string(), ',', 1);
    println!("Split string: {}", chunk);

    let numbers = [1, 2, 3, 4, 5];
    let result = sum(&numbers);
    println!("The sum is {}", result);
}
