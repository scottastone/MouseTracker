use std::thread;
use std::time::{Duration, Instant};
use mouse_rs::Mouse;
use crossterm::{event::{read, poll, Event, KeyCode}};
use clap::{Arg, App};
use lsl::{self, Pushable};

static mut LSL_ENABLE: bool = true;
static mut DEBUG_OUTPUT: bool = true;

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

fn main() -> () {
    let matches = App::new("Mouse Tracker")
        .version("0.0.1")
        .author("Scott Stone <scott.allan.stone@gmail.com>")
        .about("Mouse tracking, but in Rust!")
        .arg(Arg::with_name("samplerate")
                 .short('s')
                 .long("hertz")
                 .takes_value(false)
                 .default_missing_value(&"100".to_string())
                 .help("Mouse sampling rate."))
        .arg(Arg::with_name("lsl")
                 .short('l')
                 .long("LSL")
                 .takes_value(false)
                 .default_missing_value(&"1".to_string())
                 .help("Enable LSL streaming. On by default."))
        .get_matches();
    
    let sr = matches.value_of("samplerate").unwrap_or("100").parse::<u64>().unwrap();
    unsafe {
        let lsl_enable_value = matches.value_of("lsl").unwrap_or("1").parse::<u64>().unwrap();
        if lsl_enable_value == 1 {
            LSL_ENABLE = true;
        }
        else {
            LSL_ENABLE = false;
        }
    }

    let sample_rate: u64 = sr; // Hz;
    let update_delay_ms: u64 = 1000 / sample_rate;

    let outlet = setup_lsl(
                        "Mouse Tracker",
                        "Mouse",
                        2,
                        sr as f64,
                        lsl::ChannelFormat::Int32,
                        "mouseoutlet1");
    

    let mouse: Mouse = Mouse::new();
    let it: Instant = std::time::Instant::now();
    let mut sample_count: u64 = 0;
    //let mut data: Vec<MousePosition> = Vec::new();

    loop {
        // Handle events - check if we need to quit etc
        event_handler();

        // Update mouse position
        let mouse_pos = MousePosition::update(&mouse, it);
        sample_count += 1;

        // Send data over LSL, if necessary
        unsafe {  
            if LSL_ENABLE {
                send_lsl(&outlet, &mouse_pos).unwrap();
            }
        }

        // Print the data to the console
        unsafe {
            if DEBUG_OUTPUT == true {
                let console_string = format!(" > Count:{:>6} Time:{:>7} X:{:>5} Y:{:>5}\t{:>5}",
                        sample_count,
                        mouse_pos.timestamp,
                        mouse_pos.x,
                        mouse_pos.y,
                        check_lsl_enabled());
                println!("{console_string}");
            }
        }
        // data.push(mouse_pos);
        
        thread::sleep(Duration::from_micros(update_delay_ms * 1000)); // adjust to microseconds for more accuracy due to rounding errors
    }
}

fn send_lsl(outlet: &lsl::StreamOutlet, data: &MousePosition) -> std::io::Result<()> {
    unsafe {
        if LSL_ENABLE {
            let data = vec![data.x, data.y];
            outlet.push_sample(&data).unwrap();
        }
        Ok(())
    }
}

fn check_lsl_enabled() -> &'static str {
    unsafe {
        let lsl_enable_str: &str;
        if LSL_ENABLE == true {
            lsl_enable_str = "[+lsl]";
        }
        else {
            lsl_enable_str = "[-lsl]";
        }
        return lsl_enable_str;
    }
}

fn setup_lsl(stream_name: &str,
             stream_type: &str,
             channel_count: u32,
             nominal_srate: f64,
             channel_format: lsl::ChannelFormat,
             source_id: &str) -> lsl::StreamOutlet {

    let info = lsl::StreamInfo::new(
            stream_name,
            stream_type,
            channel_count,
            nominal_srate,
            channel_format,
            source_id)
            .unwrap();

    let outlet = lsl::StreamOutlet::new(&info, 0,360).unwrap();
    return outlet
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
        KeyCode::Char('l') => {
            unsafe {
                LSL_ENABLE ^= true;
                println!(" > LSL output enabled: {:?}", &LSL_ENABLE);
            }
            return State::None;
        }
        KeyCode::Char('d') => {
            unsafe {
                DEBUG_OUTPUT ^= true;
                if DEBUG_OUTPUT == false {
                    println!(" > Display mode off. Press 'd' to turn display mode on.");
                }
            }
            return State::None;

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

