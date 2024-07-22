use bowler::{ Bowler, BowlerType };

mod display;
mod utils;
mod bowler;
fn main() {
    utils::cls();
    display::print_bat_wicket();
    display::print_batsman();
    display::print_bowler();
    display::print_bowl_wicket();
    let bowler = Bowler::new("Wow", BowlerType::Spin);
    for _i in 0..10 {
        let mut speed: u64 = utils::random_num(20, 40).into();
        utils::sleep(1000);
        bowler.bowl(&mut speed);
        bowler.bowl_spin(&mut speed);
        // print!("{}", speed);
    }
    // sleep(10000000);
    //print_bowler();
    //pitch();
}
