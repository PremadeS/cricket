use batter::{ Batter, ShotType };
use bowler::{ Bowler, BowlerType };
use display::{ print_details, print_pitch };
use engine::Engine;
use utils::{ cls, move_cursor, random_num, sleep };

mod display;
mod utils;
mod bowler;
mod batter;
mod engine;
fn main() {
    let mut bowler: Bowler = Bowler::new("Bilqees", BowlerType::Spin);
    let mut batter: Batter = Batter::new("Suwanjana");
    let mut cricket_engine: Engine = Engine::new(false, 0);
    let mut over: [char; 6] = ['*', '*', '*', '*', '*', '*'];
    let mut prev_score: u32 = 0;
    for i in 0..12 {
        let mut speed: u64 = random_num(26, 36).into();
        cls();
        print_pitch();
        if i % 6 > 0 {
            over[(i % 6) - 1] = (cricket_engine.score - prev_score + 48) as u8 as char;
        }
        print_details(&bowler, &batter, &cricket_engine.score, &prev_score, &over);
        prev_score = cricket_engine.score; //Update previous score...

        bowler.init_spin_bowl(&mut speed);
        batter.take_shot();
        bowler.bowl(&mut speed);
        if batter.shot == ShotType::Left {
            display::print_left_bat();
        } else if batter.shot == ShotType::Right {
            display::print_right_bat();
        } else {
            display::print_front_bat();
        }
        cricket_engine.run(&bowler, &batter, speed);
        sleep(1000);
        if cricket_engine.game_end {
            cls();
            break;
        }
        if i % 5 == 0 && i != 0 {
            over = ['*', '*', '*', '*', '*', '*']; // reset over...
        }

        //FIX LEFT SPIN EDGE
    }
}
