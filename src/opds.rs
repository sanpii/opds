pub struct Opds {
    rx: Option<std::sync::mpsc::Receiver<atom_syndication::Feed>>,
    url: url::Url,
}

impl Opds {
    pub fn new(url: &str) -> Self {
        log::debug!("{}", url);

        Self {
            rx: None,
            url: url.parse().unwrap(),
        }
    }

    pub fn root(&mut self) {
        self.send(&self.url.path().to_string());
    }

    pub fn send(&mut self, path: &str) {
        log::debug!("Fetching {}", path);

        let (tx, rx) = std::sync::mpsc::channel();
        let origin = match self.url.origin() {
            url::Origin::Tuple(scheme, host, port) => (scheme, host, port),
            _ => unreachable!(),
        };

        let url = format!("{}://{}:{}{}", origin.0, origin.1, origin.2, path);

        std::thread::spawn(move || {
            match Self::try_send(&url) {
                Ok(feed) => tx.send(feed).unwrap(),
                Err(err) => log::error!("{}", err),
            }
        });

        self.rx = Some(rx);
    }

    fn try_send(url: &str) -> crate::Result<atom_syndication::Feed> {
        let response = attohttpc::get(url)
            .send()
            .unwrap();

        let text = response.text()?;
        let feed = text.parse()?;

        Ok(feed)
    }

    pub fn next(&self) -> Option<atom_syndication::Feed> {
        if let Some(rx) = &self.rx {
            rx.recv().ok()
        } else {
            None
        }
    }
}
