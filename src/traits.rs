struct Animal {
    name: String,
}

trait Canine {
    fn bark(&self) {
        println!("woof woof");
    }

    fn run(&self) {
        println!("The dog is running");
    }
}

impl Canine for Animal {
    fn run(&self){
        let my_number =5;
        println!("Dog number {} is running",my_number);
    }
}

fn mains() {
    let rover = Animal {
        name: "Rover".to_string(),
    };

    rover.bark();
    rover.run();
}
