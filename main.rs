use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq)]


struct Players {
    name: String,
    country: String,
}
impl Players{
    fn new(name: &str, country: &str) -> Players {
        Players {
            name: name.to_string(),
            country: country.to_string(),
        }
        
    }
    fn main() {
       let players: HashMap<Players, i32> = HashMap::from([
            ("Yves", "France", 90),
            ("Yorgho", "Greece", 78),
            ("Joe", "France", 94),
        ]);

        for (man, health) in players{
            println!("{:?} has {} health after battle.", man, health);
        }
    }  

}