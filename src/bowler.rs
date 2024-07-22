const BATSMAN_X: u16 = 58;
const BATSMAN_Y: u16 = 1;
const BOWLER_X: u16 = 49;
const BOWLER_Y: u16 = 33;
const TURN: u16 = 27;
const MAX_BALL_DIS: u16 = 31;
use crate::utils;
pub enum BowlerType {
    Spin,
    Fast,
}

pub struct Bowler {
    pub name: String,
    pub bowler_type: BowlerType,
}

impl Bowler {
    pub fn new(name: &str, bowler_type: BowlerType) -> Self {
        Bowler {
            name: name.to_string(),
            bowler_type,
        }
    }

    pub fn bowl(&self) {
        match self.bowler_type {
            BowlerType::Spin => self.bowl_spin(),
            BowlerType::Fast => self.bowl_spin(),
        }
    }

    fn bowl_spin(&self) {
        let mut speed: u64 = 40;
        self.init_bowl(&mut speed);
        let _type = utils::random_num(1, 3);
        if _type == 1 {
            self.bowl_spin_left(&mut speed);
        } else if _type == 2 {
            self.bowl_spin_right(&mut speed);
        } else {
            self.bowl_spin_straight(&mut speed);
        }
    }
    fn init_bowl(&self, speed: &mut u64) {
        let x: u16 = BATSMAN_X - 1;
        let y: u16 = BOWLER_Y;
        for i in 1..TURN {
            utils::move_cursor(x, y - i + 1);
            print!(" ");
            utils::move_cursor(x, y - i);
            print!("*");
            utils::sleep(*speed);
            if i < TURN / 2 {
                *speed += 1;
            } else {
                *speed += 2;
            }
        }
    }
    fn bowl_spin_left(&self, speed: &mut u64) {
        let x: u16 = BATSMAN_X - 1;
        let y: u16 = BOWLER_Y;
        for i in TURN..MAX_BALL_DIS {
            utils::move_cursor(x - (TURN - i), y - i + 1);
            print!(" ");
            utils::move_cursor(x - (TURN - i - 1), y - i);
            print!("*");
            utils::sleep(*speed);
            *speed += 1;
        }
    }
    fn bowl_spin_right(&self, speed: &mut u64) {
        let x: u16 = BATSMAN_X - 1;
        let y: u16 = BOWLER_Y;
        for i in TURN..MAX_BALL_DIS {
            utils::move_cursor(x + (TURN - i), y - i + 1);
            print!(" ");
            utils::move_cursor(x + (TURN - i - 1), y - i);
            print!("*");
            utils::sleep(*speed);
            *speed += 1;
        }
    }
    fn bowl_spin_straight(&self, speed: &mut u64) {
        let x: u16 = BATSMAN_X - 1;
        let y: u16 = BOWLER_Y;
        for i in TURN..MAX_BALL_DIS {
            utils::move_cursor(x, y - i + 1);
            print!(" ");
            utils::move_cursor(x, y - i);
            print!("*");
            utils::sleep(*speed);
            *speed += 1;
        }
    }
}
