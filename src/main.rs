struct Person {
    name: String,
    age: Option<i32>,
}

trait CreatePerson {
    fn create(name: &str) -> Self;
    fn create_with_age(name: &str, age: i32) -> Self;
    fn return_name(&self) -> &str;
    fn return_age(&self) -> Option<i32>;
    fn set_age(&mut self, age: i32);
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
    fn return_name(&self) -> &str {
        &self.name
    }
    fn return_age(&self) -> Option<i32> {
        self.age
    }
    fn set_age(&mut self, age: i32) {
        self.age = Some(age);
    }
}

fn main() {
    let mut person = Person::create("John");
    let person_with_age = Person::create_with_age("John", 30);

    match person.age {
        Some(age) => println!("Check age: {}", age),
        None => person.set_age(0),
    }

    if person.age.is_none() {
        person.set_age(18);
    }

    println!("{}", person.age.unwrap());
    println!("{}", person_with_age.age.unwrap_or_default());
}
