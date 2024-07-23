const BATSMAN_X: u16 = 58;
const BOWLER_Y: u16 = 33;
const TURN: u16 = 27;
const MAX_BALL_DIS: u16 = 31;
use crate::utils;
pub enum BowlerType {
    Spin,
    Fast,
}
#[derive(Debug, PartialEq)]
pub enum BowlType {
    Left,
    Right,
    Straight,
}
pub struct Bowler {
    pub name: String,
    pub bowler_type: BowlerType,
    pub bowl_type: BowlType,
}

impl Bowler {
    pub fn new(name: &str, bowler_type: BowlerType) -> Self {
        Bowler {
            name: name.to_string(),
            bowler_type,
            bowl_type: BowlType::Left,
        }
    }

    pub fn bowl(&self, speed: &mut u64) {
        match self.bowler_type {
            BowlerType::Spin => self.init_bowl(speed),
            BowlerType::Fast => self.init_bowl(speed),
        }
    }

    pub fn bowl_spin(&mut self, speed: &mut u64) {
        let _type = utils::random_num(1, 2);
        if _type == 1 {
            self.bowl_spin_left(speed);
            self.bowl_type = BowlType::Left;
        } else if _type == 2 {
            self.bowl_spin_right(speed);
            self.bowl_type = BowlType::Right;
        } else {
            self.bowl_spin_straight(speed);
            self.bowl_type = BowlType::Straight;
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
    fn bowl_spin_right(&self, speed: &mut u64) -> u64 {
        let x: u16 = BATSMAN_X - 1;
        let y: u16 = BOWLER_Y;
        for i in TURN..MAX_BALL_DIS {
            utils::move_cursor(x - (TURN - i), y - i + 1);
            print!(" ");
            utils::move_cursor(x - (TURN - i - 1), y - i);
            print!("*");
            utils::sleep(*speed);
            *speed += 1;
            // if i == MAX_BALL_DIS - 1 {
            //     utils::move_cursor(x - (TURN - i - 1), y - i); // remove the ball...
            //     print!(" ");
            // }
        }
        *speed
    }
    fn bowl_spin_left(&self, speed: &mut u64) -> u64 {
        let x: u16 = BATSMAN_X - 1;
        let y: u16 = BOWLER_Y;
        for i in TURN..MAX_BALL_DIS {
            utils::move_cursor(x + (TURN - i), y - i + 1);
            print!(" ");
            utils::move_cursor(x + (TURN - i - 1), y - i);
            print!("*");
            utils::sleep(*speed);
            *speed += 1;
            // if i == MAX_BALL_DIS - 1 {
            //     utils::move_cursor(x + (TURN - i - 1), y - i); // remove the ball...
            //     print!(" ");
            // }
        }

        *speed
    }
    fn bowl_spin_straight(&self, speed: &mut u64) -> u64 {
        let x: u16 = BATSMAN_X - 1;
        let y: u16 = BOWLER_Y;
        for i in TURN..=MAX_BALL_DIS {
            utils::move_cursor(x, y - i + 1);
            print!(" ");
            utils::move_cursor(x, y - i);
            print!("*");
            utils::sleep(*speed);
            *speed += 1;
        }
        *speed
    }
}
