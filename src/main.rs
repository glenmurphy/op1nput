// Hide the console on windows release builds
#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

mod keyboard;
mod midi;
mod output;
mod tray;

use std::collections::HashMap;

#[tokio::main]
async fn main() {
    // Specify mapping of midi ids to controls
    // Notes and controls are separate maps because they use the same key ids
    use keyboard::Key;
    use output::Action::*;
    use output::Control::*;

    let controls: HashMap<u8, output::Control> = HashMap::from([
        // Rotary Encoders + Press
        (01, Knob(Key::Minus, Key::Equals)),
        (
            64,
            CustomButton(
                Sequence(vec![Press(Key::Shift), Press(Key::Minus)]), // press
                Sequence(vec![Release(Key::Shift), Release(Key::Minus)]), // release
            ),
        ),
        (02, Knob(Key::BracketLeft, Key::BracketRight)),
        (
            65,
            CustomButton(
                Sequence(vec![Press(Key::Shift), Press(Key::BracketLeft)]),
                Sequence(vec![Release(Key::Shift), Release(Key::BracketLeft)]),
            ),
        ),
        (03, Knob(Key::SemiColon, Key::Quote)),
        (
            66,
            CustomButton(
                Sequence(vec![Press(Key::Shift), Press(Key::SemiColon)]),
                Sequence(vec![Release(Key::Shift), Release(Key::SemiColon)]),
            ),
        ),
        (04, Knob(Key::Comma, Key::Period)),
        (
            67,
            CustomButton(
                Sequence(vec![Press(Key::Shift), Press(Key::Comma)]),
                Sequence(vec![Release(Key::Shift), Release(Key::Comma)]),
            ),
        ),
        // Numbered row below rotary encoders
        (50, Button(Key::F1)),
        (51, Button(Key::F2)),
        (52, Button(Key::F3)),
        (21, Button(Key::F4)),
        (22, Button(Key::F5)),
        (23, Button(Key::F6)),
        (24, Button(Key::F7)),
        (25, Button(Key::F8)),
        // Mic/COM/dots buttons to the right of the encoders+numbers, top down
        (48, Button(Key::F12)), // mic
        (49, Button(Key::F11)), // COM
        (26, Button(Key::F10)), // dots
        // Numbers under the screen
        (11, Button(Key::Num1)),
        (12, Button(Key::Num2)),
        (13, Button(Key::Num3)),
        (14, Button(Key::Num4)),
        // < > keys to the left of Shift
        (
            41,
            CustomButton(
                Sequence(vec![Press(Key::Shift), Press(Key::Tab)]),
                Sequence(vec![Release(Key::Shift), Release(Key::Tab)]),
            ),
        ), // >
        (42, Button(Key::Tab)), // <
        // The nine-grid on the left, above the < > + Shift keys. Mentally maps to a numpad
        (07, Button(Key::Numpad7)), // Blue Squiggle
        (08, Button(Key::Numpad8)), // Green halo
        (09, Button(Key::Numpad9)), // Tape deck
        (15, Button(Key::Numpad4)), // Orange 1-4 Up
        (16, Button(Key::Numpad5)), // Orange Dot down
        (17, Button(Key::Numpad6)), // Orange scissors
        (38, Button(Key::Numpad1)), // Orange Record
        (39, Button(Key::Numpad2)), // Play
        (40, Button(Key::Numpad3)), // Stop
        // Misc keys under the main volume knob
        (10, Button(Key::Numpad0)),        // Chart key
        (05, Button(Key::NumpadMultiply)), // Speech bubble
        (06, Button(Key::NumpadDivide)),   // Metronome
    ]);

    let notes: HashMap<u8, output::Control> = HashMap::from([
        (53, Note(Key::A)),
        (54, Note(Key::B)),
        (55, Note(Key::C)),
        (56, Note(Key::D)),
        (57, Note(Key::E)),
        (58, Note(Key::F)),
        (59, Note(Key::G)),
        (60, Note(Key::H)),
        (61, Note(Key::I)),
        (62, Note(Key::J)),
        (63, Note(Key::K)),
        (64, Note(Key::L)),
        (65, Note(Key::M)),
        (66, Note(Key::N)),
        (67, Note(Key::O)),
        (68, Note(Key::P)),
        (69, Note(Key::Q)),
        (70, Note(Key::R)),
        (71, Note(Key::S)),
        (72, Note(Key::T)),
        (73, Note(Key::U)),
        (74, Note(Key::V)),
        (75, Note(Key::W)),
        (76, Note(Key::X)),
    ]);

    // Spawn our threads for the system tray and midi input
    let (tray_tx, mut tray_rx) = tray::start();
    let mut midi_rx = midi::start("OP-1 Midi Device".to_string()).await;

    // Handle messages from those threads
    loop {
        tokio::select! {
            Some(v) = midi_rx.recv() => {
                println!("{:?}", v);

                match v {
                    midi::MidiMessage::Connected => {
                        let _ = tray_tx.send(tray::TrayMessage::Connected);
                    }
                    midi::MidiMessage::Data(ch, id, val) if ch == 176 => {
                        if let Some(control) = controls.get(&id) {
                            output::handle_control(&control, val);
                        }
                    }
                    midi::MidiMessage::Data(_ch, id, val) => {
                        if let Some(control) = notes.get(&id) {
                            output::handle_control(&control, val);
                        }
                    }
                }

            },
            Some(v) = tray_rx.recv() => {
                match v {
                    tray::TrayMessage::Quit => {
                        println!("Bye! Missu! :*");
                        // Could do a cleaner shutdown here
                        std::process::exit(0);
                    },
                    _ => {}
                }
            }
        }
    }
}
