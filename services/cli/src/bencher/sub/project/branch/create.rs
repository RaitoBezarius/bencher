use bencher_client::types::{JsonNewBranch, JsonNewStartPoint};
use bencher_json::{BranchName, GitHash, NameId, ResourceId, Slug};

use crate::{
    bencher::{backend::AuthBackend, sub::SubCmd},
    parser::project::branch::{CliBranchCreate, CliBranchStartPoint},
    CliError,
};

#[derive(Debug, Clone)]
pub struct Create {
    pub project: ResourceId,
    pub name: BranchName,
    pub slug: Option<Slug>,
    pub start_point_branch: Option<NameId>,
    pub start_point_hash: Option<GitHash>,
    pub start_point_max_versions: u32,
    pub backend: AuthBackend,
}

impl TryFrom<CliBranchCreate> for Create {
    type Error = CliError;

    fn try_from(create: CliBranchCreate) -> Result<Self, Self::Error> {
        let CliBranchCreate {
            project,
            name,
            slug,
            start_point,
            backend,
        } = create;
        let CliBranchStartPoint {
            start_point_branch,
            start_point_hash,
            start_point_max_versions,
        } = start_point;
        Ok(Self {
            project,
            name,
            slug,
            start_point_branch,
            start_point_hash,
            start_point_max_versions,
            backend: backend.try_into()?,
        })
    }
}

impl From<Create> for JsonNewBranch {
    fn from(create: Create) -> Self {
        let Create {
            name,
            slug,
            start_point_branch,
            start_point_hash,
            start_point_max_versions,
            ..
        } = create;
        let start_point = start_point_branch.map(|branch| JsonNewStartPoint {
            branch: branch.into(),
            hash: start_point_hash.map(Into::into),
            max_versions: Some(start_point_max_versions),
        });
        Self {
            name: name.into(),
            slug: slug.map(Into::into),
            start_point,
        }
    }
}

impl SubCmd for Create {
    async fn exec(&self) -> Result<(), CliError> {
        let _json = self
            .backend
            .send(|client| async move {
                client
                    .proj_branch_post()
                    .project(self.project.clone())
                    .body(self.clone())
                    .send()
                    .await
            })
            .await?;
        Ok(())
    }
}
