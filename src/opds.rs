pub struct Opds {
    rx: Option<std::sync::mpsc::Receiver<atom_syndication::Feed>>,
    url: String,
}

impl Opds {
    pub fn new(url: &str) -> Self {
        Self {
            rx: None,
            url: url.to_string(),
        }
    }

    pub fn root(&mut self) {
        self.send("/");
    }

    fn send(&mut self, path: &str) {
        let (tx, rx) = std::sync::mpsc::channel();
        let url = format!("{}/{}", self.url, path);

        std::thread::spawn(move || {
            let response = attohttpc::get(url)
                .send()
                .unwrap();

            let text = response.text()
                .unwrap();
            let feed = text.parse().unwrap();
            tx.send(feed).unwrap();
        });

        self.rx = Some(rx);
    }

    pub fn next(&self) -> Option<atom_syndication::Feed> {
        if let Some(rx) = &self.rx {
            rx.recv().ok()
        } else {
            None
        }
    }
}
