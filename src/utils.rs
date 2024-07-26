use crossterm::event::{ self, Event, KeyCode, KeyEvent, KeyModifiers };
use crossterm::terminal::{ disable_raw_mode, enable_raw_mode };
use std::{ thread, time };
use crossterm::{ ExecutableCommand, cursor::MoveTo };
use std::io::stdout;
use rand::Rng;
use std::process;

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
pub fn read_char() -> char {
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
