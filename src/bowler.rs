const BATSMAN_X: u16 = 58;
const BOWLER_Y: u16 = 33;
const TURN: u16 = 27;
const MAX_BALL_DIS: u16 = 31;
use crate::utils::{ self, move_cursor, sleep };
#[derive(Debug, PartialEq)]
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

    pub fn bowl(&mut self, speed: &mut u64) {
        match self.bowler_type {
            BowlerType::Spin => self.bowl_spin(speed),
            BowlerType::Fast => self.fast_bowl(speed),
        }
    }

    pub fn bowl_spin(&mut self, speed: &mut u64) {
        let direction: u32 = utils::random_num(1, 10);
        if (1..=4).contains(&direction) {
            self.bowl_spin_left(speed);
            self.bowl_type = BowlType::Left;
        } else if (5..=8).contains(&direction) {
            self.bowl_spin_right(speed);
            self.bowl_type = BowlType::Right;
        } else {
            self.bowl_spin_straight(speed);
            self.bowl_type = BowlType::Straight;
        }
    }
    pub fn init_spin_bowl(&self, speed: &mut u64) {
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
    fn bowl_spin_right(&self, speed: &mut u64) {
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
    fn bowl_spin_left(&self, speed: &mut u64) {
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
        let end: u16 = MAX_BALL_DIS - 10;
        for i in TURN..=end {
            utils::move_cursor(x, y - i + 1);
            print!(" ");
            utils::move_cursor(x, y - i);
            print!("*");
            utils::sleep(*speed);
            *speed += 1;
        }
    }

    fn fast_bowl(&mut self, speed: &mut u64) {
        let direction: u32 = utils::random_num(1, 10);
        if (1..4).contains(&direction) {
            self.bowl_fast_left(speed);
            self.bowl_type = BowlType::Left;
        } else if (5..8).contains(&direction) {
            self.bowl_fast_right(speed);
            self.bowl_type = BowlType::Right;
        } else {
            self.bowl_fast_straight(speed);
            self.bowl_type = BowlType::Straight;
        }
    }
    fn bowl_fast_left(&self, speed: &mut u64) {
        let mut x: u16 = BATSMAN_X - 3;
        let y: u16 = BOWLER_Y;
        for i in 1..MAX_BALL_DIS / 2 {
            utils::move_cursor(x, y - i + 1);
            print!(" ");
            utils::move_cursor(x, y - i);
            print!("*");
            utils::sleep(30);
            if i % 2 == 0 && i < MAX_BALL_DIS / 2 - 3 {
                x -= 1;
                move_cursor(x + 1, y - i);
                print!(" ");
                *speed += 1;
            }
            if y - i == 30 {
                move_cursor(x, y - i + 1); //Don't mind this.... (reprints erased part of pitch line)
                print!("-");
            }
        }
        for i in MAX_BALL_DIS / 2..MAX_BALL_DIS {
            utils::move_cursor(x, y - i + 1);
            print!(" ");
            utils::move_cursor(x, y - i);
            print!("*");
            utils::sleep(30);
            if i % 2 == 0 && i > MAX_BALL_DIS / 2 + 3 {
                x += 1;
                move_cursor(x - 1, y - i);
                print!(" ");
            }
        }
    }
    fn bowl_fast_right(&self, speed: &mut u64) {
        let mut x: u16 = BATSMAN_X + 5;
        let y: u16 = BOWLER_Y;
        for i in 1..MAX_BALL_DIS / 2 {
            utils::move_cursor(x, y - i + 1);
            print!(" ");
            utils::move_cursor(x, y - i);
            print!("*");
            utils::sleep(30);
            if i % 2 == 0 && i < MAX_BALL_DIS / 2 - 3 {
                x += 1;
                move_cursor(x - 1, y - i);
                print!(" ");
                *speed += 1;
            }
            if y - i == 30 {
                move_cursor(x, y - i + 1); //Don't mind this.... (reprints erased part of pitch line)
                print!("-");
            }
        }
        for i in MAX_BALL_DIS / 2..MAX_BALL_DIS {
            utils::move_cursor(x, y - i + 1);
            print!(" ");
            utils::move_cursor(x, y - i);
            print!("*");
            utils::sleep(30);
            if i % 2 == 0 && i > MAX_BALL_DIS / 2 + 3 {
                x -= 1;
                move_cursor(x + 1, y - i);
                print!(" ");
            }
        }
    }
    fn bowl_fast_straight(&self, speed: &mut u64) {
        let x: u16 = BATSMAN_X - 1;
        let y: u16 = BOWLER_Y;
        let end: u16 = MAX_BALL_DIS - 4;
        for i in 1..=end {
            utils::move_cursor(x, y - i + 1);
            print!(" ");
            utils::move_cursor(x, y - i);
            print!("*");
            utils::sleep(30);
            if i % 2 == 0 {
                *speed += 1;
            }
        }
    }
}
