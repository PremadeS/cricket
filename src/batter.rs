use crossterm::event::{ self, Event, KeyCode, KeyEvent, KeyModifiers };
use crossterm::terminal::{ disable_raw_mode, enable_raw_mode };
use std::process;
use crate::utils::move_cursor;
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

    fn read_char(&self) -> char {
        //To read char without pressing enter...
        enable_raw_mode().expect("Failed to enable raw mode");
        loop {
            if event::poll(std::time::Duration::from_millis(100)).expect("Failed to poll") {
                if
                    let Event::Key(KeyEvent { code, modifiers, .. }) = event
                        ::read()
                        .expect("Failed to read event")
                {
                    if code == KeyCode::Char('c') && modifiers.contains(KeyModifiers::CONTROL) {
                        //Check for ctrl + c...
                        disable_raw_mode().expect("Failed to disable raw mode");
                        process::exit(0);
                    }
                    if let KeyCode::Char(c) = code {
                        disable_raw_mode().expect("Failed to disable raw mode");
                        return c;
                    }
                }
            }
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
        let mut shot: char = self.read_char();
        move_cursor(0, 12);
        println!("Power level?\n(L)ow , (M)edium , (H)igh");
        let mut power: char = self.read_char();
        self.clear_selection();
        while !self.valid_selection(shot, power) {
            move_cursor(0, 10);
            println!("Shot direction?\n(L)eft , (R)ight, (S)traight");
            shot = self.read_char();
            move_cursor(0, 12);
            println!("Power level?\nL)ow , (M)edium , (H)igh");
            power = self.read_char();
            self.clear_selection();
        }
        self.char_to_enum(shot, power);
    }
}
