use batter::{ Batter, ShotType };
use bowler::{ Bowler, BowlerType };
use display::{
    print_bowling_action,
    print_details,
    print_out_screen,
    print_pitch,
    print_over_screen,
};
use engine::Engine;
use utils::{ cls, move_cursor, random_num, sleep };
use text_io::read;
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
    let mut over_screen: char = ' ';
    cls();
    move_cursor(35, 15);
    print!("Enter total overs (1 over = 6 balls): "); //initial screen...
    let total_overs: usize = read!();

    for balls in 1..=total_overs * 6 {
        if (balls - 1) % 6 == 0 && balls != 1 {
            if over_screen != 'd' && over_screen != 'D' {
                over_screen = print_over_screen(&cricket_engine.score, &balls, &total_overs);
            }
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

        print_details(&bowler, &batter, &cricket_engine.score, &prev_score, &over, &(balls - 1));
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

        over[(balls - 1) % 6] = (cricket_engine.score - prev_score + 48) as u8 as char; //covert to char...

        sleep(1000);
        if cricket_engine.game_end {
            cls();
            print_out_screen(&cricket_engine.score, &balls, &total_overs);
            break;
        }
    }
}
