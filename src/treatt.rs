use std::fmt::Debug;

#[derive(Debug,Clone)]
struct Monster {
    health: i32,
}
#[derive(Debug, Clone)]
struct Wizard {}
#[derive(Debug, Clone)]
struct Ranger {}

trait FightClose: Debug + Clone {
    fn attack_with_sword(&self, opponent: &mut Monster) {
        opponent.health -= 2;
        println!(
            "You attack with your mind . Your oppponent now has {} health left",
            opponent.health
        );
    }

    fn attck_with_hand(&self, opponent: &mut Monster) {
        opponent.health -= 2;
        println!(
            "You attack with your mind . Your oppponent now has {} health left",
            opponent.health
        );
    }
}

impl FightClose for Wizard {}

impl FightFromDistance for Ranger {}

trait FightFromDistance: Debug + Clone {
    fn attack_with_bow(&self, opponent: &mut Monster, distance: u32) {
        if distance < 10 {
            opponent.health -= 10;
            println!(
                "You attack with your bow. Your opponent has {} health left ",
                opponent.health
            );
        }
    }

    fn attack_with_rock(&self, opponent: &mut Monster, distance: u32) {
        if distance < 3 {
            opponent.health -= 4;
        }
        println!(
            "You attck with your rock. Your opponent now has {} health left",
            opponent.health
        );
    }
}

fn main() {
    let radagast = Wizard {};
    let aragon = Ranger {};

    let mut uruk_hai = Monster { health: 40 };

    radagast.attack_with_sword(&mut uruk_hai);

    let distance = 8;

    radagast.attack_with_sword(&mut uruk_hai);

    aragon.attack_with_bow(&mut uruk_hai, distance);

    aragon.attack_with_bow(&mut uruk_hai, distance);
}
