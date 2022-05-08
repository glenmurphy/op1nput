use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

fn find_port<T>(device_name: &str, midi_io: &T) -> Option<T::Port>
where
    T: midir::MidiIO,
{
    let mut device_port: Option<T::Port> = None;
    for port in midi_io.ports() {
        if let Ok(port_name) = midi_io.port_name(&port) {
            println!("{}", port_name);
            if port_name.contains(device_name) {
                device_port = Some(port);
                break;
            }
        }
    }
    device_port
}

#[derive(Debug)]
#[allow(unused)]
pub enum MidiMessage {
    Connected,
    Data(u8, u8, u8),
}

pub async fn init(device_name: String, midi_tx: UnboundedSender<MidiMessage>) {
    let midi_input = midir::MidiInput::new("OPINPUT").unwrap();
    let mut device_port;

    println!("Waiting for device...");
    loop {
        // MacOS doesn't support hot-plugging
        // https://github.com/Boddlnagg/midir/issues/86#issuecomment-991967845
        device_port = find_port(&device_name, &midi_input);
        if !device_port.is_none() {
            midi_tx.send(MidiMessage::Connected).unwrap();
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(5000));
    }

    let device_port = device_port.unwrap();
    let _connect_in = midi_input.connect(
        &device_port,
        &device_name,
        move |_timestamp, data, midi_tx| {
            let _ = midi_tx.send(MidiMessage::Data(data[0], data[1], data[2]));
        },
        midi_tx,
    );

    // All the important stuff is happening in stuff we have handles to, but exiting
    // will close those handles, so peace out here. In future, if we need to accept
    // external commands, we can have start() return a channel to the main thread
    // and this can be a recv loop.
    std::thread::park();
}

pub async fn start(device_name: String) -> UnboundedReceiver<MidiMessage> {
    let (midi_tx, midi_rx) = unbounded_channel::<MidiMessage>();

    tokio::spawn(async move {
        init(device_name, midi_tx).await;
    });

    midi_rx
}
