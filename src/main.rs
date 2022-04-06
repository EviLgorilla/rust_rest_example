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
    let person = Person::create("John");
    let mut person_with_age = Person::create_with_age("John", 30);
    person_with_age.set_age(40);
    println!("{}", person.age.unwrap_or_default());
    println!("{}", person_with_age.age.unwrap_or_default());
}
