pub use rdev::Key;
use rdev::{simulate, EventType, SimulateError};

#[derive(Clone)]
#[allow(unused)]
pub enum Action {
    None,
    Delay(u64),
    Tap(Key),
    Press(Key),
    Release(Key),
    Sequence(Vec<Action>),
}

#[derive(Clone)]
#[allow(unused)]
pub enum Control {
    Knob(Key, Key),
    Button(Key),
    Note(Key),

    CustomKnob(Action, Action),
    CustomButton(Action, Action),
    CustomNote(Action, Action),
}

fn send(event_type: &EventType) {
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // Let ths OS catchup (at least MacOS)
    // let delay = std::time::Duration::from_millis(20);
    // std::thread::sleep(delay);
}

fn tap(key: Key) {
    tokio::spawn(async move {
        send(&EventType::KeyPress(key));
        std::thread::sleep(std::time::Duration::from_millis(20));
        send(&EventType::KeyRelease(key));
    });
}

fn press(key: Key) {
    send(&EventType::KeyPress(key));
}

fn release(key: Key) {
    send(&EventType::KeyRelease(key));
}

fn sequence(seq: Vec<Action>) {
    tokio::spawn(async move {
        for a in seq {
            handle_action(&a);
        }
    });
}

fn sleep(ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(ms));
}

fn handle_action(action: &Action) {
    match action {
        Action::None => (),
        Action::Delay(ms) => sleep(*ms),
        Action::Tap(key) => tap(*key),
        Action::Press(key) => press(*key),
        Action::Release(key) => release(*key),
        Action::Sequence(seq) => sequence(seq.to_vec()),
    }
}

pub fn handle_control(control: &Control, data: u8) {
    match control {
        Control::Knob(_left_key, right_key) if data == 127 => tap(*right_key),
        Control::Knob(left_key, _right_key) if data <= 1 => tap(*left_key),
        Control::Button(key) if data == 127 => press(*key),
        Control::Button(key) if data <= 1 => release(*key),
        Control::Note(key) if data > 72 => press(*key),
        Control::Note(key) if data <= 72 => release(*key),

        Control::CustomKnob(_left, right) if data == 127 => handle_action(&right), // knob right
        Control::CustomKnob(left, _right) if data <= 1 => handle_action(&left),    // knob left
        Control::CustomButton(down, _up) if data == 127 => handle_action(&down),   // knob left
        Control::CustomButton(_down, up) if data <= 1 => handle_action(&up),       // knob left
        Control::CustomNote(down, _up) if data > 72 => handle_action(&down),       // knob left
        Control::CustomNote(_down, up) if data <= 72 => handle_action(&up),        // knob left

        _ => println!("Unknown control: {}", data),
    }
}
