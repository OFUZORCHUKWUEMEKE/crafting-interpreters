use std::fmt::Debug;
use std::fmt::Display;

struct Monster {
    health: i32,
}

#[derive(Debug)]
struct Wizard {
    health: i32,
}

#[derive(Debug)]
struct Ranger {
    health: i32,
}

trait Magic {}

trait FightClose {}

trait FightFromDistance {}

impl FightClose for Ranger {}

impl FightClose for Wizard {}

impl FightFromDistance for Ranger {}

impl Magic for Wizard {}

fn print_vec<T: Display>(input: &Vec<T>) {
    for item in input {}
}

fn attack_with_bow<T: FightFromDistance + Debug>(
    character: &T,
    opponent: &mut Monster,
    distance: u32,
) {
    if distance < 15 {
        opponent.health -= 30;
        println!("You raise your hands and cast a fireball : Your opponent now has {} health left. You are now at {:?}",opponent.health,character);
    }
}

fn attack_with_sword<T: FightClose + Debug>(character: &T, opponent: &mut Monster) {
    opponent.health -= 10;

    println!(
        "You attack with your mind . Your oppponent now has {} health left , {:?}",
        opponent.health, character
    );
}

fn fireball<T: Magic + Debug>(character: &T, opponent: &mut Monster, distance: u32) {
    if distance < 15 {
        opponent.health -= 20;
        println!("You raise your hands and cast a fireball : Your opponent now has {} health left. You are now at {:?}",opponent.health,character);
    }
}

fn mains() {
    let radagast = Wizard { health: 60 };
    let aragorn = Ranger { health: 80 };

    let mut uruk_hai = Monster { health: 40 };

    attack_with_bow(&aragorn, &mut uruk_hai, 2);
    attack_with_sword(&radagast, &mut uruk_hai);
    fireball(&radagast, &mut uruk_hai, 8);
}
