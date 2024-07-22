const BATSMAN_X: u16 = 58;
const BATSMAN_Y: u16 = 1;
const BOWLER_X: u16 = 49;
const BOWLER_Y: u16 = 33;

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
