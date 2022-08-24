use std::{process::exit, time::SystemTime};

use delta_pico_rust::{interface::{ApplicationFramework, DisplayInterface, ButtonsInterface, ButtonEvent, StorageInterface, ButtonInput}, graphics::Sprite, delta_pico_main};
use minifb::{Window, WindowOptions, Scale, Key, KeyRepeat};
use clap::Parser;
use rand::prelude::SliceRandom;

const STORAGE_SIZE: usize = 1000000;

struct FrameworkImpl {
    window: Window,
    start_time: SystemTime,
    storage: [u8; STORAGE_SIZE],
    should_run_tests: bool,
    should_fuzz: bool,
    fuzzer_first_input: bool,
}

impl ApplicationFramework for FrameworkImpl {
    type DisplayI = Self;
    type ButtonsI = Self;
    type StorageI = Self;

    fn display(&self) -> &Self::DisplayI { self }
    fn display_mut(&mut self) -> &mut Self::DisplayI { self }
    fn buttons(&self) -> &Self::ButtonsI { self }
    fn buttons_mut(&mut self) -> &mut Self::ButtonsI { self }
    fn storage(&self) -> &Self::StorageI { self }
    fn storage_mut(&mut self) -> &mut Self::StorageI { self }

    fn hardware_revision(&self) -> String {
        "Simulator".to_string()
    }

    fn reboot_into_bootloader(&mut self) -> ! {
        panic!("no bootloader on simulator")
    }

    fn millis(&self) -> u64 { self.micros() / 1000 }
    fn micros(&self) -> u64 { SystemTime::now().duration_since(self.start_time).unwrap().as_micros() as u64 }

    fn memory_usage(&self) -> (usize, usize) {
        (0, 0)
    }
    
    fn debug(&self, message: &str) {
        println!("{}", message)
    }

    fn should_run_tests(&mut self) -> bool {
        self.should_run_tests
    }

    fn tests_success_hook(&mut self) {
        println!("Tests passed!");
        exit(0);
    }
}

impl DisplayInterface for FrameworkImpl {
    fn width(&self) -> u16 { 240 }
    fn height(&self) -> u16 { 320 }

    fn draw_display_sprite(&mut self, sprite: &Sprite) {
        let buffer = sprite.data.iter().map(|c| RGB332_TO_RGBA888_LOOKUP_TABLE[c.0 as usize]).collect::<Vec<_>>();
        self.window.update_with_buffer(&buffer, 240, 320).unwrap();

        if !self.window.is_open() {
            exit(0);
        }
    }
}

const RGB332_TO_RGBA888_LOOKUP_TABLE: [u32; 256] = [
    0x000000, 0x000052, 0x0000ad, 0x0000ff, 0x002400, 0x002452, 0x0024ad, 0x0024ff, 
    0x004900, 0x004952, 0x0049ad, 0x0049ff, 0x006d00, 0x006d52, 0x006dad, 0x006dff, 
    0x009200, 0x009252, 0x0092ad, 0x0092ff, 0x00b600, 0x00b652, 0x00b6ad, 0x00b6ff, 
    0x00db00, 0x00db52, 0x00dbad, 0x00dbff, 0x00ff00, 0x00ff52, 0x00ffad, 0x00ffff, 
    0x210000, 0x210052, 0x2100ad, 0x2100ff, 0x212400, 0x212452, 0x2124ad, 0x2124ff, 
    0x214900, 0x214952, 0x2149ad, 0x2149ff, 0x216d00, 0x216d52, 0x216dad, 0x216dff, 
    0x219200, 0x219252, 0x2192ad, 0x2192ff, 0x21b600, 0x21b652, 0x21b6ad, 0x21b6ff, 
    0x21db00, 0x21db52, 0x21dbad, 0x21dbff, 0x21ff00, 0x21ff52, 0x21ffad, 0x21ffff, 
    0x4a0000, 0x4a0052, 0x4a00ad, 0x4a00ff, 0x4a2400, 0x4a2452, 0x4a24ad, 0x4a24ff, 
    0x4a4900, 0x4a4952, 0x4a49ad, 0x4a49ff, 0x4a6d00, 0x4a6d52, 0x4a6dad, 0x4a6dff, 
    0x4a9200, 0x4a9252, 0x4a92ad, 0x4a92ff, 0x4ab600, 0x4ab652, 0x4ab6ad, 0x4ab6ff, 
    0x4adb00, 0x4adb52, 0x4adbad, 0x4adbff, 0x4aff00, 0x4aff52, 0x4affad, 0x4affff, 
    0x6b0000, 0x6b0052, 0x6b00ad, 0x6b00ff, 0x6b2400, 0x6b2452, 0x6b24ad, 0x6b24ff, 
    0x6b4900, 0x6b4952, 0x6b49ad, 0x6b49ff, 0x6b6d00, 0x6b6d52, 0x6b6dad, 0x6b6dff, 
    0x6b9200, 0x6b9252, 0x6b92ad, 0x6b92ff, 0x6bb600, 0x6bb652, 0x6bb6ad, 0x6bb6ff, 
    0x6bdb00, 0x6bdb52, 0x6bdbad, 0x6bdbff, 0x6bff00, 0x6bff52, 0x6bffad, 0x6bffff, 
    0x940000, 0x940052, 0x9400ad, 0x9400ff, 0x942400, 0x942452, 0x9424ad, 0x9424ff, 
    0x944900, 0x944952, 0x9449ad, 0x9449ff, 0x946d00, 0x946d52, 0x946dad, 0x946dff, 
    0x949200, 0x949252, 0x9492ad, 0x9492ff, 0x94b600, 0x94b652, 0x94b6ad, 0x94b6ff, 
    0x94db00, 0x94db52, 0x94dbad, 0x94dbff, 0x94ff00, 0x94ff52, 0x94ffad, 0x94ffff, 
    0xb50000, 0xb50052, 0xb500ad, 0xb500ff, 0xb52400, 0xb52452, 0xb524ad, 0xb524ff, 
    0xb54900, 0xb54952, 0xb549ad, 0xb549ff, 0xb56d00, 0xb56d52, 0xb56dad, 0xb56dff, 
    0xb59200, 0xb59252, 0xb592ad, 0xb592ff, 0xb5b600, 0xb5b652, 0xb5b6ad, 0xb5b6ff, 
    0xb5db00, 0xb5db52, 0xb5dbad, 0xb5dbff, 0xb5ff00, 0xb5ff52, 0xb5ffad, 0xb5ffff, 
    0xde0000, 0xde0052, 0xde00ad, 0xde00ff, 0xde2400, 0xde2452, 0xde24ad, 0xde24ff, 
    0xde4900, 0xde4952, 0xde49ad, 0xde49ff, 0xde6d00, 0xde6d52, 0xde6dad, 0xde6dff, 
    0xde9200, 0xde9252, 0xde92ad, 0xde92ff, 0xdeb600, 0xdeb652, 0xdeb6ad, 0xdeb6ff, 
    0xdedb00, 0xdedb52, 0xdedbad, 0xdedbff, 0xdeff00, 0xdeff52, 0xdeffad, 0xdeffff, 
    0xff0000, 0xff0052, 0xff00ad, 0xff00ff, 0xff2400, 0xff2452, 0xff24ad, 0xff24ff, 
    0xff4900, 0xff4952, 0xff49ad, 0xff49ff, 0xff6d00, 0xff6d52, 0xff6dad, 0xff6dff, 
    0xff9200, 0xff9252, 0xff92ad, 0xff92ff, 0xffb600, 0xffb652, 0xffb6ad, 0xffb6ff, 
    0xffdb00, 0xffdb52, 0xffdbad, 0xffdbff, 0xffff00, 0xffff52, 0xffffad, 0xffffff, 
];

const BUTTON_MAPPING: [(bool, Key, ButtonInput); 27] = [
    (false, Key::Left, ButtonInput::MoveLeft),
    (false, Key::Right, ButtonInput::MoveRight),
    (false, Key::Up, ButtonInput::MoveUp),
    (false, Key::Down, ButtonInput::MoveDown),
    (false, Key::Enter, ButtonInput::Exe),
    (false, Key::Escape, ButtonInput::Menu),
    (false, Key::Space, ButtonInput::List),

    (false, Key::Key0, ButtonInput::Digit(0)),
    (false, Key::Key1, ButtonInput::Digit(1)),
    (false, Key::Key2, ButtonInput::Digit(2)),
    (false, Key::Key3, ButtonInput::Digit(3)),
    (false, Key::Key4, ButtonInput::Digit(4)),
    (false, Key::Key5, ButtonInput::Digit(5)),
    (false, Key::Key6, ButtonInput::Digit(6)),
    (false, Key::Key7, ButtonInput::Digit(7)),
    (false, Key::Key8, ButtonInput::Digit(8)),
    (false, Key::Key9, ButtonInput::Digit(9)),

    (false, Key::Tab, ButtonInput::Shift),

    (false, Key::LeftBracket, ButtonInput::Parentheses),
    (false, Key::RightBracket, ButtonInput::Parentheses),
    (false, Key::Slash, ButtonInput::Fraction),
    (true,  Key::Equal, ButtonInput::Add),
    (false, Key::Minus, ButtonInput::Subtract),
    (true,  Key::Key8, ButtonInput::Multiply),
    (true,  Key::Key6, ButtonInput::Power),
    (false, Key::Backspace, ButtonInput::Delete),
    (false, Key::Period, ButtonInput::Point),
];

impl ButtonsInterface for FrameworkImpl {
    fn wait_event(&mut self) -> ButtonEvent {
        if self.should_fuzz { return self.fuzz_input() }

        loop {
            for (shifted, key, input) in BUTTON_MAPPING {
                let pressed_without_modifier = self.window.is_key_pressed(key, KeyRepeat::No);
                let shift_down = self.window.is_key_down(Key::LeftShift) || self.window.is_key_down(Key::RightShift);
                if pressed_without_modifier && ((!shifted && !shift_down) || (shifted && shift_down)) {
                    return ButtonEvent::Press(input)
                }
            }
            
            self.window.update();

            if !self.window.is_open() {
                exit(0);
            }
        }
    }

    fn poll_event(&mut self) -> Option<ButtonEvent> {
        None
    }
}

impl FrameworkImpl {
    fn fuzz_input(&mut self) -> ButtonEvent {
        if self.fuzzer_first_input {
            self.fuzzer_first_input = false;
            return ButtonEvent::Press(ButtonInput::Exe)
        }

        loop {
            // Pick a random item, except MENU
            let (_, _, button) = BUTTON_MAPPING.choose(&mut rand::thread_rng()).unwrap();

            if *button != ButtonInput::Menu {
                return ButtonEvent::Press(*button)
            }
        }
    }
}

impl StorageInterface for FrameworkImpl {
    fn is_connected(&mut self) -> bool { true }
    fn is_busy(&mut self) -> bool { false }

    fn write(&mut self, address: u16, bytes: &[u8]) -> Option<()> {
        self.storage[(address as usize)..(address as usize + bytes.len())].copy_from_slice(bytes);
        Some(())
    }

    fn read(&mut self, address: u16, bytes: &mut [u8]) -> Option<()> {
        bytes.copy_from_slice(&self.storage[(address as usize)..(address as usize + bytes.len())]);
        Some(())
    }

    fn acquire_priority(&mut self) {}
    fn release_priority(&mut self) {}
}

#[derive(Parser)]
struct Cli {
    #[clap(short, long, action)]
    test: bool,

    #[clap(short, long, action, conflicts_with("test"))]
    fuzz: bool,
}

fn main() {
    let args = Cli::parse();

    let framework = FrameworkImpl {
        window: Window::new(
            "Delta Pico",
            240,
            320,
            WindowOptions {
                resize: true,
                scale: Scale::X2,
                ..WindowOptions::default()
            },
        ).unwrap(),
        start_time: SystemTime::now(),
        storage: [0; STORAGE_SIZE],
        should_run_tests: args.test,
        
        should_fuzz: args.fuzz,
        fuzzer_first_input: true,
    };

    delta_pico_main(framework);

    panic!("Main loop exited");
}