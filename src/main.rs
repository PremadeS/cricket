use batter::{ Batter, ShotType };
use bowler::{ Bowler, BowlerType };
use display::{ print_bowling_action, print_details, print_pitch };
use engine::Engine;
use utils::{ cls, random_num, sleep };

mod display;
mod utils;
mod bowler;
mod batter;
mod engine;
fn main() {
    let mut bowler: Bowler = Bowler::new("temp", BowlerType::Fast);
    let mut batter: Batter = Batter::new("Chris P. Chips");
    let mut cricket_engine: Engine = Engine::new(false, 0);
    let mut over: [char; 6] = ['*', '*', '*', '*', '*', '*'];
    let mut prev_score: u32 = 0;
    for i in 0..12 {
        if i % 6 == 0 {
            over = ['*', '*', '*', '*', '*', '*']; // reset over...
            let rng = random_num(1, 2) == 1;
            if rng {
                bowler = Bowler::new("Max Power", BowlerType::Fast);
            } else {
                bowler = Bowler::new("Kent C. Strait", BowlerType::Spin);
            }
        }
        let mut speed: u64 = random_num(26, 36).into();

        cls();
        print_pitch();
        if i % 6 > 0 {
            over[(i % 6) - 1] = (cricket_engine.score - prev_score + 48) as u8 as char; //covert to char...
        }
        print_details(&bowler, &batter, &cricket_engine.score, &prev_score, &over);
        prev_score = cricket_engine.score; //Update previous score...

        if bowler.bowler_type == BowlerType::Spin {
            print_bowling_action();
            bowler.init_spin_bowl(&mut speed);
        }
        batter.take_shot();
        if bowler.bowler_type == BowlerType::Fast {
            print_bowling_action();
        }
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

        //FIX LEFT SPIN EDGE
    }
}
