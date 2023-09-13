fn main() {
    let my_name = "Billy".to_string();
    let other_name = "Billy".to_string();
    println!("{}", my_name == &other_name);
    // println!("{}", &my_name == &&other_name);
}