fn main() {
    println!("sum(1, 23): {}", sum(1, 23));

    let mut inc_by_two = make_increment(0, 2);
    println!("inc_by_two: {}", inc_by_two());
    println!("inc_by_two: {}", inc_by_two());
    println!("inc_by_two: {}", inc_by_two());
}

// Write a closure which takes in two numbers and adds them together
fn sum(x: u32, y: u32) -> u32 {
    x + y
}

// Write a function which takes in an initial value and an increment and returns a closure which increments the value by that amount each time it's called
fn make_increment(initial: i32, increment: i32) -> impl FnMut() -> i32 {
    let mut value = initial;
    move || {
        value += increment;
        value
    }
}