/**
 * Define a struct for a crew member with a name, age, 
 * and any other attributed you would like. 
 * Implement a few methods on this struct, such as one to say who it is.
 * */ 

fn main() {
    let ombladon = Rapper {
        name: "Păstacă".to_string(),
        surname: "Bogdan Ionuț".to_string()
    };

    println!("Ombladon's full name: {}", ombladon.who());
}

struct Rapper {
    name: String,
    surname: String
}

impl Rapper {
    fn who(&self) -> String {
        // self.surname.to_string() + " " + &self.name.to_string()

        // ChatGPT's first alternative
        // self.surname.clone() + " " + &self.name  
        
        // ChatGPT's second alternative
        format!("{} {}", self.surname, self.name)
    }
}