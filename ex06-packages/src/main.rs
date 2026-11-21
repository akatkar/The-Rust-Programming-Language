mod sound {
    pub mod instrument {
        pub fn clarinet() {
            println!("clarinet is called!");
            super::breath_in();
        }
    }

    fn breath_in() {
        println!("breath is called!");
    }
}

mod plant {
    pub struct Vegetable {
        pub name: String,
        id: i32
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1
            }
        }
    }
}

use sound::instrument::clarinet;
use sound::instrument;

fn main() {
    crate::sound::instrument::clarinet();

    sound::instrument::clarinet();


    let mut v = plant::Vegetable::new("squash");
    v.name = String::from("butternut squash");

    println!("{} are delicious", v.name);

    clarinet();
    instrument::clarinet();
}


