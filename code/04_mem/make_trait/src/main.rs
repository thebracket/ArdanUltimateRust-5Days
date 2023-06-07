use std::{fmt::Debug, rc::Rc};
use std::any::Any;

trait Animal {
    fn speak(&self);
}

struct Cat;

impl Animal for Cat {
    fn speak(&self) {
        println!("Meow");
    }
}

fn speak_twice(animal: &impl Animal) {
    animal.speak();
    animal.speak();
}

fn get_animal() -> impl Animal {
    Cat
}

trait DebuggableClonableAnimal: Animal+Debug+Clone {}

#[derive(Debug, Clone)]
struct Dog;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof");
    }
}

impl DebuggableClonableAnimal for Dog {}

fn clone_and_speak(animal: &impl DebuggableClonableAnimal) {
    animal.speak();
    let cloned_animal = animal.clone();
    println!("{cloned_animal:?}");
}

trait DowncastableAnimal: Animal+Any {
    fn as_any(&self) -> &dyn Any;
}

struct Tortoise;

impl Animal for Tortoise {
    fn speak(&self) {
        println!("What noise does a tortoise make anyway?");
    }
}

impl DowncastableAnimal for Tortoise {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn main() {
    let cat = Cat;
    cat.speak();

    speak_twice(&cat);

    let animal = get_animal();

    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Cat), Box::new(Dog)];

    let (tx, rx) = std::sync::mpsc::channel::<Box<dyn Animal>>();
    let (tx, rx) = std::sync::mpsc::channel::<Rc<dyn Animal>>();

    let more_animals : Vec<Box<dyn DowncastableAnimal>> = vec![Box::new(Tortoise)];
    for animal in more_animals.iter() {
        if let Some(cat) = animal.as_any().downcast_ref::<Tortoise>() {
            println!("We have access to the tortoise");
        }
        animal.speak();
    }
}
