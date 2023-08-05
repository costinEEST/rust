
fn main() {
    // let mut count = 0;
    
    // while count <= 15 {
    //     count += 1;
    //     if count % 3 == 0 && count % 5 == 0 {
    //         println!("{count}: FizzBuzz");
    //     } else if count % 3 == 0 {
    //         println!("{count}: Fizz");
    //     } else if count % 5 == 0 {
    //         println!("{count}: Buzz")
    //     }
    // }

    // for count in 1..16 {
    //     if count % 3 == 0 && count % 5 == 0 {
    //         println!("{count}: FizzBuzz");
    //     } else if count % 3 == 0 {
    //         println!("{count}: Fizz");
    //     } else if count % 5 == 0 {
    //         println!("{count}: Buzz")
    //     }
    // }

    // Bard's solution
    // for i in 1..16 {
    //     match i {
    //         n if n % 3 == 0 && n % 5 == 0 => println!("FizzBuzz"),
    //         n if n % 3 == 0 => println!("Fizz"),
    //         n if n % 5 == 0 => println!("Buzz"),
    //         _ => println!("{}", i),
    //     }
    // }

    // ChatGPT's solution
    for num in 1..=15 {
        if num % 3 == 0 && num % 5 == 0 {
            println!("FizzBuzz");
        } else if num % 3 == 0 {
            println!("Fizz");
        } else if num % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", num);
        }
    }
}