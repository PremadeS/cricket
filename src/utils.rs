use std::{ thread, time };
use crossterm::{ ExecutableCommand, cursor::MoveTo };
use std::io::stdout;
use rand::Rng;

pub fn cls() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
pub fn sleep(time_in_millis: u64) {
    thread::sleep(time::Duration::from_millis(time_in_millis));
}
pub fn move_cursor(x: u16, y: u16) {
    let mut stdout = stdout();
    stdout.execute(MoveTo(x, y)).unwrap();
}
pub fn random_num(from: u32, to: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(from..=to)
}
