// ENUM DISPATCH
trait Action {
    fn identify(&self);
    fn run(&mut self);
    fn eat(&mut self);
}

#[derive(Debug)]
struct Cat {
    weight: u8,
    size: u8,
    energy: f32,
    race: String,
}

impl Action for Cat {
    fn identify(&self) {
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
    weight: u8,
    size: u8,
    energy: f32,
    race: String,
}

impl Action for Dog {
    fn identify(&self) {
        println!("I'm an {:?} dog of {:?} kilos", self.race, self.weight);
    }
    fn run(&mut self) {
        self.energy -= 1.0;
    }
    fn eat(&mut self) {
        self.energy += 1.0;
    }
}

#[derive(Debug)]
enum Animal {
    Dog(Dog),
    Cat(Cat),
}

#[derive(Debug)]
struct Zoo {
    animals: Vec<Animal>,
}

impl Zoo {
    fn new() -> Self {
        Self {
            animals: Vec::new(),
        }
    }

    fn add_animal(&mut self, animal: Animal) {
        self.animals.push(animal);
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

    cat.run();
    zoo.add_animal(Animal::Cat(cat));
    zoo.add_animal(Animal::Dog(dog));

    println!("The presents animals are: {:#?}", zoo);
}
