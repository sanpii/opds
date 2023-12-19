pub struct Opds {
    rx: Option<std::sync::mpsc::Receiver<atom_syndication::Feed>>,
    password: Option<String>,
    username: Option<String>,
    url: url::Url,
}

impl Opds {
    pub fn new(url: &str, username: Option<String>, password: Option<String>) -> Self {
        log::debug!("{}", url);
        log::debug!("username={:?} password={:?}", username, password);

        Self {
            rx: None,
            password,
            url: url.parse().unwrap(),
            username,
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

        let url = if path.starts_with("http://") || path.starts_with("https://") {
            path.to_string()
        } else {
            format!("{}://{}:{}{}", origin.0, origin.1, origin.2, path)
        };
        let username = self.username.clone();
        let password = self.password.clone();

        std::thread::spawn(move || {
            match Self::try_send(&url, username.as_deref(), password.as_deref()) {
                Ok(feed) => tx.send(feed).unwrap(),
                Err(err) => log::error!("{}", err),
            }
        });

        self.rx = Some(rx);
    }

    fn try_send(
        url: &str,
        username: Option<&str>,
        password: Option<&str>,
    ) -> crate::Result<atom_syndication::Feed> {
        let mut request = attohttpc::get(url).proxy_settings(attohttpc::ProxySettings::from_env());
        request = if let Some(username) = username {
            request.basic_auth(username, password)
        } else {
            request
        };
        let response = request.send().unwrap();

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
