extern crate lib_tremor_common;

extern crate lib_foundry_platform;
extern crate lib_foundry_common;
extern crate cgmath;
use lib_foundry_common::*;
use std::time::{Duration, Instant};

enum TimeEvent {
    Update {update_time : f32},
    Display {interpolate_secs : f32}
}

struct TimeEventPump {
    start_time : Instant,
    time_of_last_update : u64
}

impl TimeEventPump {
    fn new() -> TimeEventPump {
        //Get time in NS now
        let time_now = Instant::now();
        //let elapsed_time = time_now.duration_since(  );
        //let current_time = (elapsed_time.as_secs() * 1000000) + (elapsed_time.subsec_nanos() as u64);

        TimeEventPump {
            start_time: time_now,
            time_of_last_update : 0
        }
    }

    fn get_next_time_event(&mut self) -> TimeEvent {
        let ns_in_second = 1000000000;
        let update_rate = ns_in_second / 30;//40; //Set to 40 for even rounding.
        //let update_rate = ns_in_second *2;//40; //Set to 40 for even rounding.

        //Get time in NS now
        let time_now = Instant::now();
        let elapsed_time = time_now.duration_since( self.start_time );
        let current_time = (elapsed_time.as_secs() * 1000000000) + (elapsed_time.subsec_nanos() as u64);

        if self.time_of_last_update + update_rate < current_time {
            //Update
            self.time_of_last_update += update_rate;
            return TimeEvent::Update { update_time: update_rate as f32 }
        }

        return TimeEvent::Display { interpolate_secs: (current_time - self.time_of_last_update) as f32 / 1000000000.0 }
    }
}

fn convert_key(in_key: lib_foundry_common::KeyboardKey) -> lib_tremor_common::input::KeyboardKey {
    
    match in_key {
        lib_foundry_common::KeyboardKey::A => {lib_tremor_common::input::KeyboardKey::A},
        lib_foundry_common::KeyboardKey::B => {lib_tremor_common::input::KeyboardKey::B},
        lib_foundry_common::KeyboardKey::C => {lib_tremor_common::input::KeyboardKey::C},
        lib_foundry_common::KeyboardKey::D => {lib_tremor_common::input::KeyboardKey::D},
        lib_foundry_common::KeyboardKey::E => {lib_tremor_common::input::KeyboardKey::E},
        lib_foundry_common::KeyboardKey::F => {lib_tremor_common::input::KeyboardKey::F},
        lib_foundry_common::KeyboardKey::G => {lib_tremor_common::input::KeyboardKey::G},
        lib_foundry_common::KeyboardKey::H => {lib_tremor_common::input::KeyboardKey::H},
        lib_foundry_common::KeyboardKey::I => {lib_tremor_common::input::KeyboardKey::I},
        lib_foundry_common::KeyboardKey::J => {lib_tremor_common::input::KeyboardKey::J},
        lib_foundry_common::KeyboardKey::K => {lib_tremor_common::input::KeyboardKey::K},
        lib_foundry_common::KeyboardKey::L => {lib_tremor_common::input::KeyboardKey::L},
        lib_foundry_common::KeyboardKey::M => {lib_tremor_common::input::KeyboardKey::M},
        lib_foundry_common::KeyboardKey::N => {lib_tremor_common::input::KeyboardKey::N},
        lib_foundry_common::KeyboardKey::O => {lib_tremor_common::input::KeyboardKey::O},
        lib_foundry_common::KeyboardKey::P => {lib_tremor_common::input::KeyboardKey::P},
        lib_foundry_common::KeyboardKey::Q => {lib_tremor_common::input::KeyboardKey::Q},
        lib_foundry_common::KeyboardKey::R => {lib_tremor_common::input::KeyboardKey::R},
        lib_foundry_common::KeyboardKey::S => {lib_tremor_common::input::KeyboardKey::S},
        lib_foundry_common::KeyboardKey::T => {lib_tremor_common::input::KeyboardKey::T},
        lib_foundry_common::KeyboardKey::U => {lib_tremor_common::input::KeyboardKey::U},
        lib_foundry_common::KeyboardKey::V => {lib_tremor_common::input::KeyboardKey::V},
        lib_foundry_common::KeyboardKey::W => {lib_tremor_common::input::KeyboardKey::W},
        lib_foundry_common::KeyboardKey::X => {lib_tremor_common::input::KeyboardKey::X},
        lib_foundry_common::KeyboardKey::Y => {lib_tremor_common::input::KeyboardKey::Y},
        lib_foundry_common::KeyboardKey::Z => {lib_tremor_common::input::KeyboardKey::Z},
        lib_foundry_common::KeyboardKey::Tab => {lib_tremor_common::input::KeyboardKey::Tab},
        lib_foundry_common::KeyboardKey::Enter => {lib_tremor_common::input::KeyboardKey::Enter},
        lib_foundry_common::KeyboardKey::Left => {lib_tremor_common::input::KeyboardKey::Left},
        lib_foundry_common::KeyboardKey::Up => {lib_tremor_common::input::KeyboardKey::Up},
        lib_foundry_common::KeyboardKey::Right => {lib_tremor_common::input::KeyboardKey::Right},
        lib_foundry_common::KeyboardKey::Down => {lib_tremor_common::input::KeyboardKey::Down},
        lib_foundry_common::KeyboardKey::Unknown => {lib_tremor_common::input::KeyboardKey::Unknown}
    }
}

pub fn main() {
    let platform = lib_foundry_platform::get_platform();
    let window = platform.create_window("Tremor").unwrap();
    let mut event : Option<PlatformEvent>;
    let mut time_event_pump = TimeEventPump::new();

    println!("Created window");
    let mut input = lib_tremor_common::input::InputManager::new();
    let mut renderer = lib_tremor_common::init();
    loop {
        //OS events
        event = lib_foundry_platform::get_platform_event(&window);
        while event.is_some() {
            match event.unwrap() {
                PlatformEvent::WindowClose => {return;},
                PlatformEvent::KeyboardKeydown {key} => {
                    input.set_key_down(convert_key(key), true);
                },
                PlatformEvent::KeyboardKeyup {key} => {
                    input.set_key_down(convert_key(key), false);
                },
            };

            event = lib_foundry_platform::get_platform_event(&window);
        }
        
        //Time events
        match time_event_pump.get_next_time_event() {
            TimeEvent::Update {update_time} => {
                lib_tremor_common::update(&mut renderer, &input)
            }
            TimeEvent::Display {interpolate_secs} => {
                lib_tremor_common::render_frame(&mut renderer);
                let val = lib_tremor_common::get_backbuffer_details(&renderer);
                lib_foundry_platform::draw_bmp(&window, val.size_x, val.size_y, val.backbuffer);
            }
        }
    }
}