// ENUM DISPATCH
trait Action {
    fn identify(&self);
    fn run(&mut self);
    fn eat(&mut self);
}

#[derive(Debug)]
struct Cat {
    size: u8,
    energy: f32,
    weight: u8,
    race: String,
}

impl Action for Cat {
    fn identify(&self {
        println!("I'm an {:?} cat of {:?} kilos", self.race, self.weight);
    }
    fn run(&mut self) {
        self.energy -= 1.0;
    }
    fn eat(&mut self) {
        self.energy += 1.0;
    }
}

#[derive(Debug)]
struct Dog {
    weight: u8
    size: u8,
    race: String,
    energy: f32,
}

impl Action for Dog {
    fn identify(&self) {
        println!("I'm an {:?} dog of {:?} kilos", self.race, self.weight);
    }
    fn run(&mut self) {
        self.energy -= 1.0;
    }
    fn eat(&mut self) {
        self.energy += 1.0
    }
}

#[derive(Debug)]
enum Animal {
    Dog(Dog),
    Cat(Cat),
}

impl Animal {
    fn energy(&self) -> f32 {
        match self {
            Animal::Dog(dog) => dog.energy,
            Animal::Cat(cat) => cat.energy,
        }
    }
}

#[derive(Debug)]
struct Zoo {
    animals: Vec<Animal>
}

impl Zoo {
    fn new() -> Self {
        Self {
            animals: Vec::new(),
        }
    }

    fn add_Animal(&mut self, animal: Animal) {
        self.animals.push(animal;
    }
}

impl Action for Animal {
    fn identify(&self) {
        match self {
            Animal::Dog(dog) => dog.identify(),
            Animal::Cat(cat) => cat.identify(),
        }
    }

    fn run(&mut self) {
        match self {
            Animal::Dog(dog) => dog.run(),
            Animal::Cat(cat) => cat.run(),
        }
    }

    fn eat(&mut self) {
        match self {
            Animal::Dog(dog) => dog.eat(),
            Animal::Cat(cat) => cat.eat(),
        }
    }
}

impl Default for Zoo {
    fn default() -> Self {
        Self::new()
    }
}
fn main() {
    let mut zoo = Zoo::new();
    let mut dog = Dog {
        weight: 20,
        size: 30,
        energy: 100.0,
        race: "Dalmatien".to_string(),
    };

    let mut cat = Cat {
        weight: 5,
        size: 15,
        energy: 100.0,
        race: "Angora".to_string(),
    };

    zoo.add_Animal(Animal::Cat(cat));
    zoo.add_Animal(Animal::Dog(dog));

    for Animals in zoo.animals.iter_mut() {
        while Animals.energy() > 32.0 {
            Animals.run();
        }
    }

    println!("The presents Animals are: {:#?}", zoo);
}
