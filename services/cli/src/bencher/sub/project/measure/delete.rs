use bencher_json::ResourceId;

use crate::{
    CliError,
    bencher::{backend::AuthBackend, sub::SubCmd},
    parser::project::measure::CliMeasureDelete,
};

#[derive(Debug)]
pub struct Delete {
    pub project: ResourceId,
    pub measure: ResourceId,
    pub backend: AuthBackend,
}

impl TryFrom<CliMeasureDelete> for Delete {
    type Error = CliError;

    fn try_from(delete: CliMeasureDelete) -> Result<Self, Self::Error> {
        let CliMeasureDelete {
            project,
            measure,
            backend,
        } = delete;
        Ok(Self {
            project,
            measure,
            backend: backend.try_into()?,
        })
    }
}

impl SubCmd for Delete {
    async fn exec(&self) -> Result<(), CliError> {
        let _json = self
            .backend
            .send(|client| async move {
                client
                    .proj_measure_delete()
                    .project(self.project.clone())
                    .measure(self.measure.clone())
                    .send()
                    .await
            })
            .await?;
        Ok(())
    }
}
