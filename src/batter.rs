use crate::utils::{ self, move_cursor };
#[derive(Debug, PartialEq)]
pub enum ShotType {
    Left,
    Right,
    Straight,
}
#[derive(Debug, PartialEq)]
pub enum ShotPower {
    Low,
    Medium,
    High,
}
pub struct Batter {
    pub name: String,
    pub shot: ShotType,
    pub power: ShotPower,
}

impl Batter {
    pub fn new(name: &str) -> Self {
        Batter {
            name: name.to_string(),
            shot: ShotType::Left, //initially assigning value...
            power: ShotPower::Low,
        }
    }

    fn char_to_enum(&mut self, shot: char, power: char) {
        let lower_shot: char = shot.to_lowercase().next().unwrap(); //Convert to lowercase...
        let lower_power: char = power.to_lowercase().next().unwrap();
        match (lower_shot, lower_power) {
            ('l', 'l') => {
                self.shot = ShotType::Left;
                self.power = ShotPower::Low;
            }
            ('l', 'm') => {
                self.shot = ShotType::Left;
                self.power = ShotPower::Medium;
            }
            ('l', 'h') => {
                self.shot = ShotType::Left;
                self.power = ShotPower::High;
            }
            ('r', 'l') => {
                self.shot = ShotType::Right;
                self.power = ShotPower::Low;
            }
            ('r', 'm') => {
                self.shot = ShotType::Right;
                self.power = ShotPower::Medium;
            }
            ('r', 'h') => {
                self.shot = ShotType::Right;
                self.power = ShotPower::High;
            }
            ('s', 'l') => {
                self.shot = ShotType::Straight;
                self.power = ShotPower::Low;
            }
            ('s', 'm') => {
                self.shot = ShotType::Straight;
                self.power = ShotPower::Medium;
            }
            ('s', 'h') => {
                self.shot = ShotType::Straight;
                self.power = ShotPower::High;
            }
            _ => panic!("Invalid character"),
        }
    }
    fn valid_selection(&self, shot: char, power: char) -> bool {
        (shot == 'l' || shot == 'r' || shot == 'L' || shot == 'R' || shot == 's' || shot == 'S') &&
            (power == 'l' ||
                power == 'L' ||
                power == 'm' ||
                power == 'M' ||
                power == 'h' ||
                power == 'H')
    }
    fn clear_selection(&self) {
        move_cursor(0, 10);
        print!(
            "                                     \n                                \n                              \n                           "
        );
    }
    pub fn take_shot(&mut self) {
        move_cursor(0, 10);
        println!("Shot direction?\n(L)eft , (R)ight, (S)traight");
        let mut shot: char = utils::read_char();
        move_cursor(0, 12);
        println!("Power level?\n(L)ow , (M)edium , (H)igh");
        let mut power: char = utils::read_char();
        self.clear_selection();
        while !self.valid_selection(shot, power) {
            move_cursor(0, 10);
            println!("Shot direction?\n(L)eft , (R)ight, (S)traight");
            shot = utils::read_char();
            move_cursor(0, 12);
            println!("Power level?\nL)ow , (M)edium , (H)igh");
            power = utils::read_char();
            self.clear_selection();
        }
        self.char_to_enum(shot, power);
    }
}
