const BATSMAN_X: u16 = 58;
const BATSMAN_Y: u16 = 1;
const BOWLER_X: u16 = 49;
const BOWLER_Y: u16 = 33;
const TERMINAL_X: u16 = 120;
const TERMINAL_Y: u16 = 40;
use crate::utils::{ self, cls, move_cursor, sleep };

pub fn print_pitch() {
    print_bat_wicket();
    print_bowler();
    print_bowl_wicket();
    print_batsman();
}
pub fn print_bat_wicket() {
    let x: u16 = BATSMAN_X - 2;
    let y: u16 = BATSMAN_Y;
    move_cursor(x, y);
    print!("|||");
    move_cursor(x, y + 1);
    print!("|||");
}
pub fn print_bowl_wicket() {
    let x: u16 = BATSMAN_X - 2;
    let y: u16 = BOWLER_Y + 4;
    move_cursor(x, y);
    print!("|||");
    move_cursor(x, y + 1);
    print!("|||");
}
pub fn print_batsman() {
    let x: u16 = BATSMAN_X;
    let y: u16 = BATSMAN_Y;
    move_cursor(x + 1, y);
    print!("  0");
    move_cursor(x + 1, y + 1);
    print!("  /\\");
    move_cursor(x, y + 2);
    print!("  / |");
    move_cursor(x, y + 3);
    print!(" /  /");
    move_cursor(x, y + 4);
    print!("/  \\");
    move_cursor(x, y + 5);
    print!("   _\\");
}
pub fn print_bowler() {
    let x: u16 = BOWLER_X;
    let y: u16 = BOWLER_Y;
    print_generic_man(x, y);
}
pub fn print_generic_man(x: u16, y: u16) {
    move_cursor(x + 1, y);
    print!("  0");
    move_cursor(x, y + 1);
    print!("  /|\\");
    move_cursor(x, y + 2);
    print!(" / | \\");
    move_cursor(x, y + 3);
    print!("   |");
    move_cursor(x, y + 4);
    print!("  / \\");
    move_cursor(x, y + 5);
    print!(" /   \\");
}
pub fn print_left_fielder() {
    let x: u16 = TERMINAL_X / 3;
    let y: u16 = TERMINAL_Y / 2;
    print_generic_man(x, y);
}
pub fn print_left_boundary() {
    cls();
    let mut x: u16 = TERMINAL_X / 4;
    let mut y: u16 = 0;
    for i in 0..TERMINAL_Y / 2 {
        move_cursor(x, y);
        print!("|+|");
        y += 1;
        if i < TERMINAL_Y / 4 {
            x -= 1;
        } else if i % 2 == 0 {
            x -= 1;
        }
    }
    for j in TERMINAL_Y / 2..TERMINAL_Y {
        move_cursor(x, y);
        print!("|+|");
        y += 1;
        if j > TERMINAL_Y / 2 + TERMINAL_Y / 4 {
            x += 1;
        } else if j % 2 == 0 {
            x += 1;
        }
    }
    print_left_fielder();
}
pub fn print_left_six() {
    let mut x: u16 = TERMINAL_X;
    let mut y: u16 = 5;
    for i in (10..TERMINAL_X).rev() {
        move_cursor(x, y);
        print!("*");
        sleep(20);
        move_cursor(x, y);
        print!(" ");
        x -= 1;
        if i % 6 == 0 {
            y += 1;
        }
    }
}
pub fn print_left_hit_animation() {
    let x: u16 = BATSMAN_X;
    let y: u16 = BATSMAN_Y;
    move_cursor(x - 3, y + 2);
    print!("====-/");
    move_cursor(x + 1, y + 3);
    print!(" ");
    move_cursor(x, y + 4);
    print!(" ");
    move_cursor(x - 20, y + 2);

    // move_cursor(x - 1, y + 3);
    // print!("==");
    // move_cursor(x - 2, y + 4);
    // print!("=");
    // move_cursor(x, y + 4);
    // print!(" ");
    // utils::sleep(0);
    // move_cursor(x - 2, y + 3);
    // print!("=");
    // move_cursor(x - 2, y + 4);
    // print!(" ");
    // utils::sleep(0);
    // move_cursor(x - 3, y + 2);
    // print!("==");
    // move_cursor(x - 3, y + 3);
    // print!("     ");
    // move_cursor(x - 20, y + 4);
}
