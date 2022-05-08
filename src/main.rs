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
    use output::Control::{*};
    use output::Action::{*};
    use output::Key;
    let controls: HashMap<u8, output::Control> = HashMap::from([
        // Blue Rotary Encoder + Press
        (01, Knob(Key::F1, Key::F2)),
        (64, Button(Key::KeyA)),
        (24, Button(Key::KeyA)),
        (25, CustomButton(
                Sequence(vec![
                    Press(Key::ControlLeft),
                    Press(Key::KeyC),
                ]),
                Sequence(vec![
                    Release(Key::ControlLeft),
                    Release(Key::KeyC),
                ]),
            ),
        ),
    ]);

    let notes: HashMap<u8, output::Control> = HashMap::from([
        (53, Note(Key::F1)),
        (55, Note(Key::F2)),
        (57, Note(Key::F3)),
        (59, Note(Key::F4)),
        (60, Note(Key::F5)),
        (62, Note(Key::F6)),
        (64, Note(Key::F7)),
        (65, Note(Key::F8)),
        (67, Note(Key::F9)),
        (69, Note(Key::F10)),
        (71, Note(Key::F11)),
        (72, Note(Key::F12)),
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
                    _ => {}
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
