use std::collections::HashMap;

use gix::Repository;

use crate::{ContextPath, RunContext};

mod fingerprint;
mod operating_system;

use fingerprint::Fingerprint;
use operating_system::OperatingSystem;

const ROOT: &str = "root";

impl RunContext {
    pub fn current() -> Self {
        let mut context = RunContext::default();
        git_context(&mut context);
        testbed_context(&mut context);
        context
    }

    fn insert(&mut self, path: &str, value: String) -> Option<String> {
        let key = Self::key(path);
        self.0.insert(key, value)
    }
}

#[expect(clippy::implicit_hasher)]
impl From<RunContext> for HashMap<String, String> {
    fn from(context: RunContext) -> Self {
        context.0
    }
}

fn git_context(context: &mut RunContext) {
    let Some(repo) = find_repo() else {
        return;
    };

    if let Some(repo_name) = repo_name(&repo) {
        context.insert(ContextPath::REPO_NAME, repo_name);
    }

    if let Some(root_commit) = repo_hash(&repo) {
        context.insert(ContextPath::REPO_HASH, root_commit);
    }

    if let Some((branch_ref, branch_ref_name)) = branch_ref(&repo) {
        context.insert(ContextPath::BRANCH_REF, branch_ref);
        context.insert(ContextPath::BRANCH_REF_NAME, branch_ref_name);
    }

    if let Some(hash) = branch_hash(&repo) {
        context.insert(ContextPath::BRANCH_HASH, hash);
    }
}

fn testbed_context(context: &mut RunContext) {
    if let Some(os) = OperatingSystem::current() {
        context.insert(ContextPath::TESTBED_OS, os.to_string());
    }

    if let Some(fingerprint) = Fingerprint::current() {
        context.insert(ContextPath::TESTBED_FINGERPRINT, fingerprint.to_string());
    }
}

fn find_repo() -> Option<Repository> {
    let current_dir = std::env::current_dir().ok()?;
    for directory in current_dir.ancestors() {
        if let Ok(repo) = gix::open(directory) {
            return Some(repo);
        }
    }
    None
}

fn repo_name(repo: &Repository) -> Option<String> {
    let Some(parent) = repo.path().parent() else {
        return Some(ROOT.to_owned());
    };
    let file_name = parent.file_name()?;
    file_name.to_str().map(ToOwned::to_owned)
}

fn repo_hash(repo: &Repository) -> Option<String> {
    let head_id = repo.head_id().ok()?;
    let rev_walk = repo.rev_walk([head_id]).all().ok()?;
    if let Some(Ok(commit)) = rev_walk.last() {
        Some(commit.id().object().ok()?.id.to_string())
    } else {
        None
    }
}

fn branch_ref(repo: &Repository) -> Option<(String, String)> {
    if let Ok(Some(name)) = repo.head_name() {
        Some((
            String::from_utf8_lossy(name.as_bstr()).to_string(),
            String::from_utf8_lossy(name.shorten()).to_string(),
        ))
    } else {
        None
    }
}

fn branch_hash(repo: &Repository) -> Option<String> {
    let head_id = repo.head_id().ok()?;
    let head_object = head_id.object().ok()?;
    Some(head_object.id.to_string())
}
