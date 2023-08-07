/**
 * Write a small program which constructs a HashMap 
 * with pirate ship names as keys and their crew sizes as values. 
 * For example, "Black Pearl" could have 3 crew members. 
 * Iterate over the elements of the hashmap and 
 * print each ship with its crew size
 * */ 

use std::collections::HashMap;

fn main() {
    let ship = HashMap::from([
        ("Black Pearl", 3),
        ("Anaconda", 5),
        ("Mamba", 6),
    ]);

    for (key, val) in ship.iter() {
        println!("Ship {key} has {val} pirates")
    }
}
