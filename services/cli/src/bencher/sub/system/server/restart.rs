use bencher_client::types::JsonRestart;

use crate::{
    CliError,
    bencher::{backend::AuthBackend, sub::SubCmd},
    parser::system::server::CliRestart,
};

#[derive(Debug, Clone)]
pub struct Restart {
    pub delay: u64,
    pub backend: AuthBackend,
}

impl TryFrom<CliRestart> for Restart {
    type Error = CliError;

    fn try_from(restart: CliRestart) -> Result<Self, Self::Error> {
        let CliRestart { delay, backend } = restart;
        Ok(Self {
            delay,
            backend: backend.try_into()?,
        })
    }
}

impl From<Restart> for JsonRestart {
    fn from(restart: Restart) -> Self {
        let Restart { delay, .. } = restart;
        Self { delay: Some(delay) }
    }
}

impl SubCmd for Restart {
    async fn exec(&self) -> Result<(), CliError> {
        let _json =
            self.backend
                .send(|client| async move {
                    client.server_restart_post().body(self.clone()).send().await
                })
                .await?;
        Ok(())
    }
}
