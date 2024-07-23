use batter::{ Batter, ShotPower, ShotType };
use bowler::{ BowlType, Bowler, BowlerType };
use display::print_pitch;
use rand::random;
use utils::random_num;

mod display;
mod utils;
mod bowler;
mod batter;
fn main() {
    let mut bowler = Bowler::new("Wow", BowlerType::Spin);
    let mut batter = Batter::new("Les go");
    for _i in 0..10 {
        utils::cls();
        print_pitch();
        let mut speed: u64 = utils::random_num(20, 40).into();
        bowler.bowl(&mut speed);
        batter.take_shot();
        bowler.bowl_spin(&mut speed);
        if batter.shot == ShotType::Left {
            display::print_left_bat();
        } else {
            display::print_right_bat();
        }
        if
            batter.shot == ShotType::Left &&
            batter.power == ShotPower::High &&
            bowler.bowl_type == BowlType::Left
        {
            utils::sleep(1000);
            display::print_left_boundary();
            if random_num(1, 2) == 0 {
                display::print_left_six(random_num(0, 1) == 0);
            } else {
                display::print_left_four(random_num(1, 1) != 0);
            }
        } else {
            utils::sleep(1000);
            display::print_right_boundary();
            if random_num(1, 1) == 2 {
                display::print_right_six(random_num(1, 1) != 0);
            } else {
                display::print_right_four(random_num(1, 1) != 0);
            }
        }
        utils::sleep(20000);

        // print!("{}", speed);
    }
    // sleep(10000000);
    //print_bowler();
    //pitch();
}
