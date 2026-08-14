use mattermost::client::Session;

const SERVICE: &str = "mattermost-desktop";
const ACCOUNT: &str = "session";

pub fn save_session(session: &Session) {
    let result = (|| -> anyhow::Result<()> {
        let entry = keyring::Entry::new(SERVICE, ACCOUNT)?;
        let bytes = serde_json::to_vec(session)?;
        entry.set_secret(&bytes)?;
        Ok(())
    })();

    if let Err(err) = result {
        log::warn!("failed to save session to keychain: {err}");
    }
}

pub fn load_session() -> Option<Session> {
    let entry = keyring::Entry::new(SERVICE, ACCOUNT).ok()?;
    let bytes = entry.get_secret().ok()?;
    log::debug!("loaded saved session from keychain, {} bytes", bytes.len());
    serde_json::from_slice(&bytes).ok()
}

pub fn clear_session() {
    let result = keyring::Entry::new(SERVICE, ACCOUNT).and_then(|entry| entry.delete_credential());

    if let Err(err) = result {
        log::warn!("failed to clear saved session from keychain: {err}");
    }
}
