use bowler::{ Bowler, BowlerType };
use utils::sleep;

mod display;
mod utils;
mod bowler;
fn main() {
    //let mut speed: u64 = 40;
    utils::cls();
    display::print_bat_wicket();
    display::print_batsman();
    display::print_bowler();
    display::print_bowl_wicket();
    let bowler = Bowler::new("Wow", BowlerType::Spin);
    for _i in 0..10 {
        sleep(1000);
        bowler.bowl();
    }
    // sleep(10000000);
    //print_bowler();
    //pitch();
}
