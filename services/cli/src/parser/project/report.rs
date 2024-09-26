use bencher_json::{DateTime, GitHash, NameId, ReportUuid, ResourceId};
use clap::{Args, Parser, Subcommand, ValueEnum};

use super::run::{CliRunAdapter, CliRunAverage, CliRunFold, CliRunThresholds};
use crate::parser::{CliBackend, CliPagination};

#[derive(Subcommand, Debug)]
pub enum CliReport {
    /// List reports
    #[clap(alias = "ls")]
    List(CliReportList),
    /// Create a report
    #[clap(alias = "add")]
    Create(Box<CliReportCreate>),
    /// View a report
    #[clap(alias = "get")]
    View(CliReportView),
    /// Delete a report
    #[clap(alias = "rm")]
    Delete(CliReportDelete),
}

#[derive(Parser, Debug)]
pub struct CliReportList {
    /// Project slug or UUID
    pub project: ResourceId,

    /// Branch name, slug, or UUID
    #[clap(long)]
    pub branch: Option<NameId>,

    /// Testbed name, slug, or UUID
    #[clap(long)]
    pub testbed: Option<NameId>,

    /// Start time (seconds since epoch)
    #[clap(long)]
    pub start_time: Option<DateTime>,

    /// End time (seconds since epoch)
    #[clap(long)]
    pub end_time: Option<DateTime>,

    #[clap(flatten)]
    pub pagination: CliPagination<CliReportsSort>,

    /// Filter for reports with an archived branch or testbed
    #[clap(long)]
    pub archived: bool,

    #[clap(flatten)]
    pub backend: CliBackend,
}

#[derive(ValueEnum, Debug, Clone)]
#[clap(rename_all = "snake_case")]
pub enum CliReportsSort {
    /// Date time of the report
    DateTime,
}

#[derive(Parser, Debug)]
pub struct CliReportCreate {
    /// Project slug or UUID
    pub project: ResourceId,

    /// Branch name, slug, or UUID
    #[clap(long)]
    pub branch: NameId,

    /// `git` commit hash
    #[clap(long)]
    pub hash: Option<GitHash>,

    #[clap(flatten)]
    pub start_point: CliReportStartPoint,

    /// Testbed name, slug, or UUID
    #[clap(long)]
    pub testbed: NameId,

    #[clap(flatten)]
    pub thresholds: CliRunThresholds,

    /// Start time (ISO 8601 formatted string)
    #[clap(long)]
    pub start_time: chrono::DateTime<chrono::Utc>,

    /// End time (ISO 8601 formatted string)
    #[clap(long)]
    pub end_time: chrono::DateTime<chrono::Utc>,

    /// Benchmark results
    #[clap(long)]
    pub results: Vec<String>,

    /// Benchmark harness adapter
    #[clap(value_enum, long)]
    pub adapter: Option<CliRunAdapter>,

    /// Benchmark harness suggested central tendency (ie average)
    #[clap(value_enum, long)]
    pub average: Option<CliRunAverage>,

    /// Fold multiple results into a single result
    #[clap(value_enum, long)]
    pub fold: Option<CliRunFold>,

    #[clap(flatten)]
    pub backend: CliBackend,
}

#[allow(clippy::struct_field_names)]
#[derive(Args, Debug)]
pub struct CliReportStartPoint {
    /// Use the specified branch name, slug, or UUID as the start point for `branch`.
    /// If `branch` already exists and the start point is different, a new branch will be created.
    #[clap(long)]
    pub start_point_branch: Option<NameId>,

    /// Use the specified full `git` hash as the start point for `branch` (requires: `--start-point-branch`).
    /// If `branch` already exists and the start point hash is different, a new branch will be created.
    #[clap(long, requires = "start_point_branch")]
    pub start_point_hash: Option<GitHash>,

    /// The maximum number of historical branch versions to include (requires: `--start-point-branch`).
    /// Versions beyond this number will be omitted.
    #[clap(long, requires = "start_point_branch", default_value = "255")]
    pub start_point_max_versions: u32,

    /// Reset `branch` to an empty state.
    /// If `branch` already exists, a new empty branch will be created.
    /// If a start point is provided, the new branch will begin at that start point.
    #[clap(long)]
    pub start_point_reset: bool,
}

#[derive(Parser, Debug)]
pub struct CliReportView {
    /// Project slug or UUID
    pub project: ResourceId,

    /// Report UUID
    pub report: ReportUuid,

    #[clap(flatten)]
    pub backend: CliBackend,
}

#[derive(Parser, Debug)]
pub struct CliReportDelete {
    /// Project slug or UUID
    pub project: ResourceId,

    /// Report UUID
    pub report: ReportUuid,

    #[clap(flatten)]
    pub backend: CliBackend,
}
