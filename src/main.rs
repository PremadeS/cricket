use batter::{ Batter, ShotPower, ShotType };
use bowler::{ BowlType, Bowler, BowlerType };
use display::{ print_back_boundary, print_back_catch, print_back_four, print_pitch };
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
        print_back_boundary();
        print_back_catch(true);
        utils::sleep(20000);

        // print!("{}", speed);
    }
    // sleep(10000000);
    //print_bowler();
    //pitch();
}
