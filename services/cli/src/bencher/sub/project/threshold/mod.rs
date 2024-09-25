use crate::{bencher::sub::SubCmd, parser::project::threshold::CliThreshold, CliError};

mod create;
mod delete;
mod list;
pub mod model;
mod update;
mod view;

#[derive(Debug)]
pub enum Threshold {
    List(list::List),
    Create(create::Create),
    View(view::View),
    Update(update::Update),
    Delete(delete::Delete),
}

#[derive(Debug, thiserror::Error)]
pub enum ThresholdError {
    #[error("Failed to find Bencher project. Set the project as the first argument, use the `--project` argument, or the `BENCHER_PROJECT` environment variable.")]
    NoProject,
    #[error("Invalid threshold model: {0}")]
    BadModel(bencher_json::ValidError),
}

impl TryFrom<CliThreshold> for Threshold {
    type Error = CliError;

    fn try_from(threshold: CliThreshold) -> Result<Self, Self::Error> {
        Ok(match threshold {
            CliThreshold::List(list) => Self::List(list.try_into()?),
            CliThreshold::Create(create) => Self::Create(create.try_into()?),
            CliThreshold::View(view) => Self::View(view.try_into()?),
            CliThreshold::Update(update) => Self::Update(update.try_into()?),
            CliThreshold::Delete(delete) => Self::Delete(delete.try_into()?),
        })
    }
}

impl SubCmd for Threshold {
    async fn exec(&self) -> Result<(), CliError> {
        match self {
            Self::List(list) => list.exec().await,
            Self::Create(create) => create.exec().await,
            Self::View(view) => view.exec().await,
            Self::Update(update) => update.exec().await,
            Self::Delete(delete) => delete.exec().await,
        }
    }
}
