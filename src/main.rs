use std::thread;
use std::time::{Duration, Instant};
use mouse_rs::Mouse;
use crossterm::{event::{read, poll, Event, KeyCode}};
use clap::{Arg, App};

fn main() -> () {
    let matches = App::new("My Test Program")
        .version("0.0.1")
        .author("Scott Stone <scott.allan.stone@gmail.com>")
        .about("Mouse tracking, but in Rust!")
        .arg(Arg::with_name("samplerate")
                 .short('s')
                 .long("hertz")
                 .takes_value(false)
                 .default_missing_value(&"100".to_string())
                 .help("Mouse sampling rate."))
        .get_matches();

    let sr = matches.value_of("samplerate").unwrap_or("100").parse::<u64>().unwrap();

    let sample_rate: u64 = sr; // Hz;
    let update_delay_ms: u64 = 1000 / sample_rate;

    let mouse: Mouse = Mouse::new();
    let it: Instant = std::time::Instant::now();
    let mut sample_count: u64 = 0;
    let mut data: Vec<MousePosition> = Vec::new();

    loop {
        let mouse_pos = MousePosition::update(&mouse, it);
        sample_count += 1;

        println!(" > Count:{sc:>6} Time:{ts:>7} X:{x:>5} Y:{y:>5}",
            sc = sample_count,
            ts = mouse_pos.timestamp,
            x  = mouse_pos.x,
            y  = mouse_pos.y); 

        data.push(mouse_pos);
        event_handler();
        
        thread::sleep(Duration::from_micros(update_delay_ms * 1000));
    }
}

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

