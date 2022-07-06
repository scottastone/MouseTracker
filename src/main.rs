use std::thread;
use std::time::{Duration, Instant};
use mouse_rs::Mouse;
use crossterm::{event::{read, poll, Event, KeyCode}};

enum State {
    Exit,
    Pause,
    Save,
    None,
}

struct MousePosition {
    timestamp: u128,
    x: i32, 
    y: i32,
}

impl MousePosition{
    fn new(timestamp: u128, x: i32, y: i32) -> MousePosition {
        MousePosition {
            timestamp,
            x,
            y,
        }
    }

    fn update(mouse: &Mouse, instant: std::time::Instant) -> MousePosition {
        let timestamp = instant.elapsed().as_millis();
        let x = mouse.get_position().unwrap().x;
        let y = mouse.get_position().unwrap().y;

        return MousePosition::new(timestamp, x, y)
    }
}

fn get_events() -> KeyCode {
    if poll(Duration::from_millis(0)).unwrap() {
        let ev = read().unwrap();
        match ev {
            Event::Key(key) => {
                return key.code;
            }
            _ => {todo!("Handle other events");}
        }
    }
    return KeyCode::Null; // default value
}

fn keycode_handler(key: KeyCode) -> State {
    match key {
        KeyCode::Esc => {
            return State::Exit;
        }
        KeyCode::Char('q') => {
            return State::Exit;
        }
        KeyCode::Char('p') => {
            return State::Pause;
        }
        KeyCode::Char('s') => {
            return State::Save;
        }
        _ => {
            return State::None;
        }
    }
}

fn state_handler(state: State) -> () {
    match state {
        State::Exit => {
            println!("Exiting...");
            std::process::exit(0);
        }
        State::Pause => {
            println!("Pausing, press a key to continue...");
            let _ = read(); // blocking
        }
        State::Save => {
            println!("Saving...");
        }
        State::None => {
            // do nothing
        }
    }
}

fn event_handler() -> () {
    let keycode = get_events();
    if keycode != KeyCode::Null {
        //println!("Keycode: {:?}", keycode);
        state_handler(keycode_handler(keycode));
    }
}

fn main() -> () {
    const SAMPLE_RATE: u64 = 100;
    const UPDATE_DELAY_MS: u64 = 1000 / SAMPLE_RATE;

    let mouse: Mouse = Mouse::new();
    let it: Instant = std::time::Instant::now();
    let mut sample_count: u64 = 0;
    let mut data: Vec<MousePosition> = Vec::new();

    loop {
        let mouse_pos = MousePosition::update(&mouse, it);
        sample_count += 1;

        println!("Count:{sc:>6} Time:{ts:>7} X:{x:>5} Y:{y:>5}",
            sc = sample_count,
            ts = mouse_pos.timestamp,
            x  = mouse_pos.x,
            y  = mouse_pos.y); 

        data.push(mouse_pos);
        event_handler();
        
        thread::sleep(Duration::from_millis(UPDATE_DELAY_MS));
    }
}
