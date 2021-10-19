use termion::event::Key;

pub struct Events {
    rx: std::sync::mpsc::Receiver<Key>,
}

impl Events {
    pub fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

        std::thread::spawn(move || {
            use termion::input::TermRead;

            let stdin = std::io::stdin();

            for key in stdin.keys().flatten() {
                if let Err(err) = tx.send(key) {
                    eprintln!("{}", err);
                    return;
                }
            }
        });

        Self {
            rx,
        }
    }

    pub fn next(&self) -> Result<Key, std::sync::mpsc::RecvError> {
        self.rx.recv()
    }
}
