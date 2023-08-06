fn main() {
    fizzbuzz_up_to(15);
}

fn fizzbuzz_up_to(max: u32) {
    for count in 1..=max {
        if count % 3 == 0 && count % 5 == 0 {
            println!("{count}: FizzBuzz");
        } else if count % 3 == 0 {
            println!("{count}: Fizz");
        } else if count % 5 == 0 {
            println!("{count}: Buzz")
        }
    }
}