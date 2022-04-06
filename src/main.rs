struct Person {
    name: String,
    age: Option<i32>,
}

impl Person {
    fn new(name: &str, age: Option<i32>) -> Person {
        Person {
            name: String::from(name),
            age,
        }
    }
}

fn main() {
    let guess = "42".parse::<u32>().unwrap();
    let person = Person::new("John", Some(42));
    println!("{}", person.age.unwrap());
}
