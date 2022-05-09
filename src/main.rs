// Hide the console on windows release builds
#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

mod midi;
mod output;
mod tray;

use std::collections::HashMap;

#[tokio::main]
async fn main() {
    // Specify mapping of midi ids to controls
    // Notes and controls are separate maps because they use the same key ids
    use output::Action::*;
    use output::Control::*;
    use output::Key;

    let controls: HashMap<u8, output::Control> = HashMap::from([
        // Rotary Encoders + Press
        (01, Knob(Key::Minus, Key::Equal)),
        (
            64,
            CustomButton(
                Sequence(vec![Press(Key::ShiftLeft), Press(Key::Minus)]),
                Sequence(vec![Release(Key::ShiftLeft), Release(Key::Minus)]),
            ),
        ),
        (02, Knob(Key::LeftBracket, Key::RightBracket)),
        (
            65,
            CustomButton(
                Sequence(vec![Press(Key::ShiftLeft), Press(Key::LeftBracket)]),
                Sequence(vec![Release(Key::ShiftLeft), Release(Key::LeftBracket)]),
            ),
        ),
        (03, Knob(Key::SemiColon, Key::Quote)),
        (
            66,
            CustomButton(
                Sequence(vec![Press(Key::ShiftLeft), Press(Key::SemiColon)]),
                Sequence(vec![Release(Key::ShiftLeft), Release(Key::SemiColon)]),
            ),
        ),
        (04, Knob(Key::Comma, Key::Dot)),
        (
            67,
            CustomButton(
                Sequence(vec![Press(Key::ShiftLeft), Press(Key::Comma)]),
                Sequence(vec![Release(Key::ShiftLeft), Release(Key::Comma)]),
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
                Sequence(vec![Press(Key::ShiftLeft), Press(Key::Tab)]),
                Sequence(vec![Release(Key::ShiftLeft), Release(Key::Tab)]),
            ),
        ), // >
        (42, Button(Key::Tab)), // <
        // The nine-grid on the left, above the < > + Shift keys. Mentally maps to a numpad
        (07, Button(Key::Num7)), // Blue Squiggle
        (08, Button(Key::Num8)), // Green halo
        (09, Button(Key::Num9)), // Tape deck
        (15, Button(Key::Num4)), // Orange 1-4 Up
        (16, Button(Key::Num5)), // Orange Dot down
        (17, Button(Key::Num6)), // Orange scissors
        (38, Button(Key::Num1)), // Orange Record
        (39, Button(Key::Num2)), // Play
        (40, Button(Key::Num3)), // Stop
        // Misc keys under the main volume knob
        (10, Button(Key::Num0)),    // Chart key
        (05, Button(Key::KpMinus)), // Speech bubble
        (06, Button(Key::KpPlus)),  // Metronome
    ]);

    let notes: HashMap<u8, output::Control> = HashMap::from([
        (53, Note(Key::KeyA)),
        (54, Note(Key::KeyB)),
        (55, Note(Key::KeyC)),
        (56, Note(Key::KeyD)),
        (57, Note(Key::KeyE)),
        (58, Note(Key::KeyF)),
        (59, Note(Key::KeyG)),
        (60, Note(Key::KeyH)),
        (61, Note(Key::KeyI)),
        (62, Note(Key::KeyJ)),
        (63, Note(Key::KeyK)),
        (64, Note(Key::KeyL)),
        (65, Note(Key::KeyM)),
        (66, Note(Key::KeyN)),
        (67, Note(Key::KeyO)),
        (68, Note(Key::KeyP)),
        (69, Note(Key::KeyQ)),
        (70, Note(Key::KeyR)),
        (71, Note(Key::KeyS)),
        (72, Note(Key::KeyT)),
        (73, Note(Key::KeyU)),
        (74, Note(Key::KeyV)),
        (75, Note(Key::KeyW)),
        (76, Note(Key::KeyX)),
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
                    midi::MidiMessage::Data(ch, id, val) => {
                        if ch == 176 {
                            if let Some(control) = controls.get(&id) {
                                output::handle_control(&control, val);
                            }
                        } else {
                            if let Some(control) = notes.get(&id) {
                                output::handle_control(&control, val);
                            }
                        }
                    }
                }

            },
            Some(v) = tray_rx.recv() => {
                match v {
                    tray::TrayMessage::Quit => {
                        println!("bye!");
                        // Could do a cleaner shutdown here
                        std::process::exit(0);
                    },
                    _ => {}
                }
            }
        }
    }
}
