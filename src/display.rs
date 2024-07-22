const BATSMAN_X: u16 = 58;
const BATSMAN_Y: u16 = 1;
const BOWLER_X: u16 = 49;
const BOWLER_Y: u16 = 33;
const TURN: u16 = 27;
const MAX_BALL_DIS: u16 = 31;
use crate::utils;

pub fn print_bat_wicket() {
    let x: u16 = BATSMAN_X - 2;
    let y: u16 = BATSMAN_Y;
    utils::move_cursor(x, y);
    print!("|||");
    utils::move_cursor(x, y + 1);
    print!("|||");
}
pub fn print_bowl_wicket() {
    let x: u16 = BATSMAN_X - 2;
    let y: u16 = BOWLER_Y + 4;
    utils::move_cursor(x, y);
    print!("|||");
    utils::move_cursor(x, y + 1);
    print!("|||");
}
pub fn print_batsman() {
    let x: u16 = BATSMAN_X;
    let y: u16 = BATSMAN_Y;
    utils::move_cursor(x + 1, y);
    print!(" 0");
    utils::move_cursor(x + 1, y + 1);
    print!(" /\\");
    utils::move_cursor(x, y + 2);
    print!(" / |");
    utils::move_cursor(x, y + 3);
    print!("/  /");
    utils::move_cursor(x, y + 4);
    print!("  \\");
    utils::move_cursor(x, y + 5);
    print!("  _\\");
}
pub fn print_bowler() {
    let x: u16 = BOWLER_X;
    let y: u16 = BOWLER_Y;
    utils::move_cursor(x + 1, y);
    print!("  0");
    utils::move_cursor(x, y + 1);
    print!("  /|\\");
    utils::move_cursor(x, y + 2);
    print!(" / | \\");
    utils::move_cursor(x, y + 3);
    print!("*  |");
    utils::move_cursor(x, y + 4);
    print!("  / \\");
    utils::move_cursor(x, y + 5);
    print!(" /   \\");
}
pub fn animate_ball_first(speed: &mut u64) {
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
pub fn animate_ball_second(speed: &mut u64) {
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
