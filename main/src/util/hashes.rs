use std::collections::HashMap;
use std::hash::Hash;

struct Person<'a> {
    name: &'a str,
    age: u32,
}

impl<'a> Hash for Person<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.age.hash(state);
    }
}
impl<'a> Eq for Person<'a> {}

impl<'a> std::cmp::PartialEq for Person<'a> {
    fn eq(&self, other: &Self) -> bool {
        return self.name == other.name && self.age == other.age;
    }
}

pub fn main() {
    let people = vec![
        Person {
            name: "John",
            age: 42,
        },
        Person {
            name: "Lucy",
            age: 38,
        },
    ];
    let age_by_person: HashMap<Person, u32> = HashMap::from_iter(people.into_iter().map(|p| {
        let age = p.age;
        (p, age)
    }));

    for (p, age) in age_by_person {
        println!("Person: {}, with age: {}", p.name, age);
    }
}
