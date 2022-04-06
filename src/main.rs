struct Person {
    name: String,
    age: Option<i32>,
}

trait CreatePerson {
    fn create(name: &str) -> Self;
    fn create_with_age(name: &str, age: i32) -> Self;
}

impl CreatePerson for Person {
    fn create(name: &str) -> Self {
        Self {
            name: String::from(name),
            age: None
        }
    }
    fn create_with_age(name: &str, age: i32) -> Self {
        Self {
            name: String::from(name),
            age: Some(age),
        }
    }
}

fn main() {
    let person = Person::create("John");
    let person_with_age = Person::create_with_age("John", 30);
    println!("{}", person.age.unwrap_or_default());
    println!("{}", person_with_age.age.unwrap_or_default());
}
