use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tray_item::TrayItem;

#[allow(unused)]
pub enum TrayMessage {
    Connected,
    Disconnected,
    Quit,
}

pub async fn init(
    mut main_rx: UnboundedReceiver<TrayMessage>,
    tray_tx: UnboundedSender<TrayMessage>,
) {
    let mut tray = TrayItem::new("OP1NPUT", "op1nput-off").unwrap();

    tray.add_label("OP1INPUT").unwrap();

    tray.add_menu_item("Quit", move || {
        let _ = tray_tx.send(TrayMessage::Quit);
    })
    .unwrap();

    // All the important stuff is happening in stuff we have handles to, but exiting
    // will close those handles, so peace out here. In future, if we need to accept
    // external commands, we can have start() return a channel to the main thread
    // and this can be a recv loop.
    loop {
        let message = main_rx.recv().await;
        match message.unwrap() {
            TrayMessage::Connected => {
                let _ = tray.set_icon("op1nput-on");
            }
            TrayMessage::Disconnected => {
                let _ = tray.set_icon("op1nput-off");
            }
            _ => {}
        }
    }
}

pub struct TrayInterface {
    pub tx : UnboundedSender<TrayMessage>,
    pub rx : UnboundedReceiver<TrayMessage>
}

impl TrayInterface {
    pub fn send(&self, message: TrayMessage) {
        let _ = self.tx.send(message);
    }

    //pub fn clone_tx(&self) -> UnboundedSender<TrayMessage> {
    //    self.tx.clone()
    //}

    pub async fn recv(&mut self) -> Option<TrayMessage> {
        tokio::macros::support::poll_fn(|cx| self.rx.poll_recv(cx)).await
    }
}

pub fn start() -> TrayInterface {
    let (tray_tx, tray_rx) = unbounded_channel::<TrayMessage>();
    let (main_tx, main_rx) = unbounded_channel::<TrayMessage>();

    // TODO: make this cross platform
    // TrayItem seems to have a different API on different platforms
    #[cfg(target_os = "windows")]
    tokio::spawn(async move { init(main_rx, tray_tx).await });

    TrayInterface {
        tx : main_tx,
        rx : tray_rx
    }
}
