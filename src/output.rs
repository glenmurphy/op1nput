use crate::keyboard;
use crate::keyboard::Key;

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

fn tap(key: Key) {
    tokio::spawn(async move {
        keyboard::press(key);
        std::thread::sleep(std::time::Duration::from_millis(20));
        keyboard::release(key);
    });
}

fn press(key: Key) {
    keyboard::press(key);
    //send(&EventType::KeyPress(key));
}

fn release(key: Key) {
    keyboard::release(key);
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
        Control::Knob(left_key, _right_key) if data == 127 => tap(*left_key),
        Control::Knob(_left_key, right_key) if data <= 1 => tap(*right_key),
        Control::Button(key) if data == 127 => press(*key),
        Control::Button(key) if data <= 1 => release(*key),
        Control::Note(key) if data > 72 => press(*key),
        Control::Note(key) if data <= 72 => release(*key),

        Control::CustomKnob(left, _right) if data == 127 => handle_action(&left),
        Control::CustomKnob(_left, right) if data <= 1 => handle_action(&right),
        Control::CustomButton(down, _up) if data == 127 => handle_action(&down),
        Control::CustomButton(_down, up) if data <= 1 => handle_action(&up),
        Control::CustomNote(down, _up) if data > 72 => handle_action(&down),
        Control::CustomNote(_down, up) if data <= 72 => handle_action(&up),

        _ => println!("Unknown control: {}", data),
    }
}
