use batter::{ Batter, ShotPower, ShotType };
use bowler::{ BowlType, Bowler, BowlerType };
use display::print_pitch;

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
            display::print_left_hit_animation();
        }
        if
            batter.shot == ShotType::Left &&
            batter.power == ShotPower::High &&
            bowler.bowl_type == BowlType::Left
        {
            utils::sleep(1000);
            display::print_left_boundary();
            display::print_left_six();
        }
        utils::sleep(2000);

        // print!("{}", speed);
    }
    // sleep(10000000);
    //print_bowler();
    //pitch();
}
