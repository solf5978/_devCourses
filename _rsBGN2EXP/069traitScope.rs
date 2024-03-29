// -------------------------------------------
// 		   1    - Scope of a Trait
//      	    - Trait methods with same name for a type
// 		        - Super Traits
//      	    - Marker Traits
//      	    - Auto Traits
// -------------------------------------------

// -------------- Code for library Starts -----------
/*
pub mod basic {
    pub trait personal_info {
        fn how_old(self) -> u8;
    }

    pub struct Person {
        pub name: String,
        pub age: u8,
    }

    impl personal_info for Person {
        fn how_old(self) -> u8 {
            self.age
        }
    }
}

fn some_fn() {
    use basic::{personal_info, Person};
    let p_1 = crate::basic::Person {
        name: "Nouman".to_string(),
        age: 40,
    };

    p_1.how_old();
}

mod basic_1 {
    use crate::basic::personal_info;

    struct Person_1 {
        name: String,
        age: u8,
    }

    impl personal_info for Person_1 {
        fn how_old(self) -> u8 {
            self.age
        }
    }
}
*/
// -------------- Code for library ends -----------
/*
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;
impl Pilot for Human {
    fn fly(&self) {
        println!("This is you captain speaking");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("I can fly due to magical powers");
    }
}

impl Human {
    fn fly(&self) {
        println!("Waving arms but unable to fly");
    }
}
fn main() {
    let person = Human;
    person.fly();

    Pilot::fly(&person);
    Wizard::fly(&person);
}
*/

/*
trait Person {
    fn name(&self) -> &str;
}

trait Student: Person {
    fn complete_info(&self) -> (&str, u8, &str);
}

struct uni_student {
    name_std: String,
    age: u8,
    university: String,
}

impl Student for uni_student {
    fn complete_info(&self) -> (&str, u8, &str) {
        (&self.name(), self.age, &self.university)
    }
}

impl Person for uni_student {
    fn name(&self) -> &str {
        &self.name_std
    }
}

fn info<S: Student>(s: &S) {
    println!("{:?}", s.complete_info());
    println!("{:?}", s.name());
}

trait Programmer {
    fn fav_language(&self) -> String;
}

trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}
fn main() {
    let s = uni_student {
        name_std: "SomeOne".to_string(),
        age: 40,
        university: "NUCES".to_string(),
    };

    s.complete_info();
    s.name();

    info(&s);
}
*/

/*
trait Some_properties: Clone + PartialEq + Default {}

#[derive(Default, Clone, PartialEq)]
struct Student {
    name: String,
    age: u8,
    nationality: String,
}
impl Some_properties for Student {}

fn main() {}
*/

#[derive(Default)]
struct Customer {
    name: String,
    age: u8,
    relationship: Visit,
}

enum Visit {
    casual,
    new,
    frequent,
}

impl Default for Visit {
    fn default() -> Self {
        Self::new
    }
}

fn main() {
    let c_1 = Customer::default();
}
