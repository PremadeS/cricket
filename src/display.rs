const BATSMAN_X: u16 = 58;
const BATSMAN_Y: u16 = 1;
const BOWLER_X: u16 = 49;
const BOWLER_Y: u16 = 33;
const TERMINAL_X: u16 = 120;
const TERMINAL_Y: u16 = 40;
use crate::utils::{ cls, move_cursor, sleep };
use crate::batter::Batter;
use crate::bowler::{ Bowler, BowlerType };

pub fn print_details(
    bowler: &Bowler,
    batter: &Batter,
    curr_score: &u32,
    prev_score: &u32,
    over: &[char]
) {
    move_cursor(0, 0);
    print!("Score: {}", *curr_score);
    move_cursor(0, 38);
    print!("Batter: {}", batter.name);
    move_cursor(0, 40);
    print!("Bowler: {}", bowler.name);
    if bowler.bowler_type == BowlerType::Spin {
        print!("  |  Type: Spin");
    } else {
        print!("  |  Type: Fast");
    }
    move_cursor(TERMINAL_X - 25, TERMINAL_Y);
    print!("|");
    for &scores in over.iter() {
        print!(" {} |", scores);
    }
    move_cursor(TERMINAL_X - 40, TERMINAL_Y);
    print!("Last Ball: {}", curr_score - prev_score);
}
pub fn print_pitch() {
    print_bat_wicket();
    print_bowler();
    print_bowl_wicket();
    print_batsman();
    print_line(TERMINAL_X / 3 - 2, 8, 40);
    print_line(TERMINAL_X / 3 - 2, TERMINAL_Y - 9, 40);
}
pub fn print_line(x: u16, y: u16, length: u16) {
    move_cursor(x, y);
    for _i in 0..length / 2 {
        print!("- ");
    }
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
pub fn erase_generic_man(x: u16, y: u16) {
    move_cursor(x + 1, y);
    print!("   ");
    move_cursor(x, y + 1);
    print!("    ");
    move_cursor(x, y + 2);
    print!("      ");
    move_cursor(x, y + 3);
    print!("     ");
    move_cursor(x, y + 4);
    print!("      ");
    move_cursor(x, y + 5);
    print!("       ");
}

pub fn print_left_bat() {
    let x: u16 = BATSMAN_X;
    let y: u16 = BATSMAN_Y;
    move_cursor(x - 3, y + 2);
    print!("====-/");
    move_cursor(x + 1, y + 3);
    print!(" ");
    move_cursor(x, y + 4);
    print!(" ");
    move_cursor(0, 0);
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
pub fn print_init_left_six() {
    let mut x: u16 = 53;
    let mut y: u16 = 3;
    for i in 0..53 {
        move_cursor(x, y);
        print!("*");
        sleep(12);
        move_cursor(x, y);
        print!(" ");
        if i % 26 == 0 && i != 0 {
            y -= 1;
        }
        x -= 1;
    }
}
pub fn print_left_six(catch: bool) {
    let mut x: u16 = TERMINAL_X;
    let mut y: u16 = 5;
    let end: u16;
    if catch {
        end = 43;
    } else {
        end = 10;
    }
    for i in (end..TERMINAL_X).rev() {
        move_cursor(x, y);
        print!("*");
        sleep(15);
        move_cursor(x, y);
        print!(" ");
        x -= 1;
        if i % 6 == 0 {
            y += 1;
        }
    }
    if catch {
        print_left_catch();
        move_cursor(0, 0);
    }
}
pub fn print_left_catch() {
    let x: u16 = TERMINAL_X / 3;
    let y: u16 = TERMINAL_Y / 2;
    move_cursor(x + 3, y - 1);
    print!("*");
    move_cursor(x + 2, y);
    print!("/");
    move_cursor(x + 2, y + 1);
    print!("\\ ");
    move_cursor(x + 4, y);
    print!("\\");
    move_cursor(x + 4, y + 1);
    print!("/");
    move_cursor(x + 1, y + 2);
    print!(" ");
    move_cursor(x + 5, y + 2);
    print!(" ");
}
pub fn print_init_left_four() {
    let mut x: u16 = 53;
    let mut y: u16 = 3;
    for i in 0..53 {
        move_cursor(x, y);
        print!("*");
        sleep(10);
        move_cursor(x, y);
        print!(" ");
        if i % 8 == 0 && i != 0 {
            y += 1;
        }
        x -= 1;
    }
}
pub fn print_left_four(block: bool) {
    let mut x: u16 = TERMINAL_X;
    let mut y: u16 = TERMINAL_Y / 2 + 3;
    let mut sleep_time: u16;
    let end: u16;
    if block {
        end = 46; // caught by fielder...
        sleep_time = 10;
    } else {
        end = 10; //boundary...
        sleep_time = 5;
    }
    for i in (end..TERMINAL_X).rev() {
        move_cursor(x, y);
        print!("*");
        sleep(sleep_time.into());
        move_cursor(x, y);
        print!(" ");
        x -= 1;
        if i % 20 == 0 {
            y += 1;
            sleep_time += 5;
        }
    }
    if block {
        print_left_block_fielder();
        move_cursor(0, 0);
    }
}
pub fn print_left_block_fielder() {
    let x: u16 = TERMINAL_X / 3;
    let y: u16 = TERMINAL_Y / 2 + 3;
    erase_generic_man(x, y - 5);
    move_cursor(x + 1, y);
    print!("  0");
    move_cursor(x, y + 1);
    print!("   |\\");
    move_cursor(x, y + 2);
    print!("   \\ [*");
    move_cursor(x, y + 3);
    print!("    \\ ");
    move_cursor(x, y + 4);
    print!("    /_");
}
pub fn print_left_tip() {
    let mut x: u16 = 53;
    let mut y: u16 = 3;
    let mut speed: u64 = 30;
    for i in 0..35 {
        move_cursor(x, y);
        print!("*");
        move_cursor(x, y);
        print!(" ");
        x -= 1;
        sleep(speed);
        if i % 4 == 0 {
            y += 1;
            speed += 5;
        }
    }
}

pub fn print_right_bat() {
    let x: u16 = BATSMAN_X;
    let y: u16 = BATSMAN_Y;
    move_cursor(x + 2, y + 2);
    print!(" \\-====");
    move_cursor(x + 1, y + 3);
    print!(" ");
    move_cursor(x, y + 4);
    print!(" ");
    move_cursor(0, 0);
}
pub fn print_right_fielder() {
    let x: u16 = TERMINAL_X / 2;
    let y: u16 = TERMINAL_Y / 2;
    print_generic_man(x, y);
}
pub fn print_right_boundary() {
    cls();
    let mut x: u16 = TERMINAL_X / 2 + TERMINAL_X / 5;
    let mut y: u16 = 0;
    for i in 0..TERMINAL_Y / 2 {
        move_cursor(x, y);
        print!("|+|");
        y += 1;
        if i < TERMINAL_Y / 4 {
            x += 1;
        } else if i % 2 == 0 {
            x += 1;
        }
    }
    for j in TERMINAL_Y / 2..TERMINAL_Y {
        move_cursor(x, y);
        print!("|+|");
        y += 1;
        if j > TERMINAL_Y / 2 + TERMINAL_Y / 4 {
            x -= 1;
        } else if j % 2 == 0 {
            x -= 1;
        }
    }
    print_right_fielder();
}
pub fn print_init_right_six() {
    let mut x: u16 = 67;
    let mut y: u16 = 3;
    for i in 0..53 {
        move_cursor(x, y);
        print!("*");
        sleep(12);
        move_cursor(x, y);
        print!(" ");
        if i % 26 == 0 && i != 0 {
            y -= 1;
        }
        x += 1;
    }
}
pub fn print_right_six(catch: bool) {
    let mut x: u16 = 0;
    let mut y: u16 = 8;
    let end: u16;
    if catch {
        end = 55;
    } else {
        end = 10;
    }
    for i in end..TERMINAL_X {
        move_cursor(x, y);
        print!("*");
        sleep(15);
        move_cursor(x, y);
        print!(" ");
        x += 1;
        if i % 6 == 0 {
            y += 1;
        }
    }
    if catch {
        print_right_catch();
        move_cursor(0, 0);
    }
}
pub fn print_right_catch() {
    let x: u16 = TERMINAL_X / 2;
    let y: u16 = TERMINAL_Y / 2;
    move_cursor(x + 3, y - 1);
    print!("*");
    move_cursor(x + 2, y);
    print!("/");
    move_cursor(x + 2, y + 1);
    print!("\\ ");
    move_cursor(x + 4, y);
    print!("\\");
    move_cursor(x + 4, y + 1);
    print!("/");
    move_cursor(x + 1, y + 2);
    print!(" ");
    move_cursor(x + 5, y + 2);
    print!(" ");
}
pub fn print_init_right_four() {
    let mut x: u16 = 67;
    let mut y: u16 = 3;
    for i in 0..53 {
        move_cursor(x, y);
        print!("*");
        sleep(10);
        move_cursor(x, y);
        print!(" ");
        if i % 8 == 0 && i != 0 {
            y += 1;
        }
        x += 1;
    }
}
pub fn print_right_four(block: bool) {
    let mut x: u16 = 0;
    let mut y: u16 = TERMINAL_Y / 2 + 3;
    let mut sleep_time: u64;
    let end: u16;
    if block {
        end = 57; // caught by fielder...
        sleep_time = 10;
    } else {
        end = 10; //boundary...
        sleep_time = 5;
    }
    for i in (end..TERMINAL_X).rev() {
        move_cursor(x, y);
        print!("*");
        sleep(sleep_time);
        move_cursor(x, y);
        print!(" ");
        x += 1;
        if i % 20 == 0 {
            y += 1;
            sleep_time += 5;
        }
    }
    if block {
        print_right_block_fielder();
        move_cursor(0, 0)
    }
}
pub fn print_right_block_fielder() {
    let x: u16 = TERMINAL_X / 2;
    let y: u16 = TERMINAL_Y / 2 + 3;
    erase_generic_man(x, y - 5);
    move_cursor(x + 1, y);
    print!("   0");
    move_cursor(x, y + 1);
    print!("   /|");
    move_cursor(x, y + 2);
    print!(" *] / ");
    move_cursor(x, y + 3);
    print!("   / ");
    move_cursor(x, y + 4);
    print!("  _\\");
}
pub fn print_right_tip() {
    let mut x: u16 = 64;
    let mut y: u16 = 4;
    let mut speed: u64 = 30;
    for i in 0..35 {
        move_cursor(x, y);
        print!("*");
        move_cursor(x, y);
        print!(" ");
        x += 1;
        sleep(speed);
        if i % 4 == 0 {
            y += 1;
            speed += 5;
        }
    }
}

pub fn print_keeper() {
    let x: u16 = 50;
    let y: u16 = 24;
    move_cursor(x, y);
    print!("   @");
    move_cursor(x, y + 1);
    print!("  /|\\");
    move_cursor(x, y + 2);
    print!("  [ ]");
    move_cursor(x, y + 3);
    print!(" /   \\");
    move_cursor(x, y + 4);
    print!("_\\   /_");
    move_cursor(0, 0);
}
pub fn print_back_boundary() {
    cls();
    let mut x: u16 = 0;
    let mut y: u16 = 7;
    loop {
        if x > TERMINAL_X {
            break;
        }
        move_cursor(x, y);
        print!("|+|");
        x += 2;
        if x == 14 || x == 30 || x == 44 {
            y -= 1;
        } else if x == 74 || x == 90 || x == 104 {
            y += 1;
        }
    }
    print_keeper();
}
pub fn print_back_four() {
    let mut x: u16 = 45;
    let mut y: u16 = 40;
    let mut sleep_time: u64 = 25;
    for i in 0..TERMINAL_Y - 2 {
        move_cursor(x, y);
        print!("*");
        sleep(sleep_time);
        move_cursor(x, y);
        print!(" ");
        y -= 1;
        if i % 5 == 0 {
            x -= 1;
            sleep_time += 5;
        }
    }
}
pub fn print_back_catch(drop: bool) {
    let x: u16 = 53;
    let mut y: u16 = 40;
    while y > 26 {
        move_cursor(x, y);
        print!("*");
        sleep(50);
        move_cursor(x, y);
        print!(" ");
        y -= 1;
    }
    move_cursor(x, y);
    print!("*");
    move_cursor(0, 0);

    if drop {
        sleep(500); //suspense...

        while y < 29 {
            move_cursor(x, y - 1);
            print!(" ");
            move_cursor(x, y); // Catch dropped :(....
            print!("*");
            sleep(50);
            y += 1;
        }
        move_cursor(0, 0);
    }
}
pub fn print_init_back() {
    let mut x: u16 = 54;
    let mut y: u16 = 3;
    for i in 0..3 {
        move_cursor(x, y);
        print!("*");
        sleep(50);
        move_cursor(x, y);
        print!(" ");
        if i % 26 == 0 && i != 0 {
            x -= 1;
        }
        y -= 1;
    }
}

pub fn print_front_bat() {
    let x: u16 = BATSMAN_X - 1;
    let y: u16 = BATSMAN_Y;
    move_cursor(x, y + 2);
    print!(" __");
    move_cursor(x, y + 3);
    print!("|_");
    move_cursor(x, y + 4);
    print!("# ");
    move_cursor(x, y + 5);
    print!("# ");
    move_cursor(0, 0);
}
pub fn print_front_boundary() {
    cls();
    let mut x: u16 = 0;
    let mut y: u16 = 28;
    loop {
        if x > TERMINAL_X {
            break;
        }
        move_cursor(x, y);
        print!("|+|");
        x += 2;
        if x == 14 || x == 30 || x == 44 {
            y += 1;
        } else if x == 74 || x == 90 || x == 104 {
            y -= 1;
        }
    }
}
pub fn print_front_shot(four: bool) {
    let mut x: u16 = 45;
    let mut y: u16 = 0;
    let mut sleep_time: u64;
    let end: u16;
    if four {
        sleep_time = 35;
        end = TERMINAL_Y - 3;
    } else {
        sleep_time = 60;
        end = TERMINAL_Y - 20;
    }
    for i in 0..end {
        move_cursor(x, y);
        print!("*");
        sleep(sleep_time);
        move_cursor(x, y);
        print!(" ");
        y += 1;
        if i % 5 == 0 {
            x -= 1;
            sleep_time += 5;
        }
    }
}
pub fn print_front_tip() {
    let mut x: u16 = 57;
    let mut y: u16 = 7;
    let mut speed: u64 = 50;
    for i in 0..15 {
        move_cursor(x, y);
        print!("*");
        move_cursor(x, y);
        print!(" ");
        y += 1;
        sleep(speed);
        if i % 10 == 0 && i != 0 {
            x -= 1;
            speed += 15;
        }
    }
}
pub fn print_init_front_shot() {
    let mut x: u16 = 54;
    let mut y: u16 = 10;
    for i in 0..30 {
        move_cursor(x, y);
        print!("*");
        sleep(40);
        move_cursor(x, y);
        print!(" ");
        if i % 4 == 0 && i != 0 {
            x -= 1;
        }
        y += 1;
    }
}

pub fn print_spin_wicket(speed: u64, out: bool) {
    let x: u16 = 57;
    let mut y: u16 = 7;
    let end: u16;
    if out {
        end = 6;
    } else {
        end = 4;
    }
    for _i in 0..end {
        move_cursor(x, y);
        print!("*");
        sleep(speed);
        move_cursor(x, y);
        print!(" ");
        y -= 1;
        if y == 3 {
            y -= 1;
        }
    }
}
