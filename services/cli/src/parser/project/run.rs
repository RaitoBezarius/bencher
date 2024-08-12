use bencher_json::{DateTime, GitHash, NameId, ResourceId};
use camino::Utf8PathBuf;
use clap::{ArgGroup, Args, Parser, ValueEnum};

use crate::parser::CliBackend;

#[derive(Parser, Debug)]
#[allow(clippy::option_option, clippy::struct_excessive_bools)]
pub struct CliRun {
    /// Project slug or UUID (or set `BENCHER_PROJECT`)
    #[clap(long)]
    pub project: Option<ResourceId>,

    #[clap(flatten)]
    pub branch: CliRunBranch,

    /// Testbed name, slug, or UUID (or set `BENCHER_TESTBED`) (default is "localhost").
    /// If a name or slug is provided, the testbed will be created if it does not exist
    #[clap(long)]
    pub testbed: Option<NameId>,

    /// Benchmark harness adapter (or set `BENCHER_ADAPTER`) (default is "magic")
    #[clap(value_enum, long)]
    pub adapter: Option<CliRunAdapter>,

    /// Benchmark harness suggested central tendency (ie average)
    #[clap(value_enum, long)]
    pub average: Option<CliRunAverage>,

    /// Number of run iterations (default is 1)
    #[clap(long)]
    pub iter: Option<usize>,

    /// Fold multiple results into a single result
    #[clap(value_enum, long, requires = "iter")]
    pub fold: Option<CliRunFold>,

    /// Backdate the report (seconds since epoch)
    /// NOTE: This will NOT effect the ordering of past reports
    #[clap(long)]
    pub backdate: Option<DateTime>,

    /// Allow benchmark test failure
    #[clap(long)]
    pub allow_failure: bool,

    /// Error on alert
    #[clap(long)]
    pub err: bool,

    #[clap(flatten)]
    pub fmt: CliRunFmt,

    /// CI integrations
    #[clap(flatten)]
    pub ci: CliRunCi,

    #[clap(flatten)]
    pub cmd: CliRunCommand,

    /// Do a dry run (no data is saved)
    #[clap(long)]
    pub dry_run: bool,

    #[clap(flatten)]
    pub backend: CliBackend,
}

#[derive(Args, Debug)]
#[allow(clippy::option_option)]
pub struct CliRunBranch {
    /// Branch name, slug, or UUID (or set `BENCHER_BRANCH`) (default is "main").
    /// If `branch` does not already exist, it will be created if a name or slug is provided.
    #[clap(long, alias = "if-branch")]
    pub branch: Option<NameId>,

    #[clap(flatten)]
    pub hash: CliRunHash,

    /// Use the specified branch name, slug, or UUID as the start point for `branch`.
    /// If `branch` already exists and the start point is different, a new branch will be created.
    /// Specifying more than one start point is now deprecated.
    /// Only the first start point will be used.
    #[clap(long, alias = "else-if-branch")]
    // TODO move this to Option<String> in due time
    pub branch_start_point: Vec<String>,

    /// Use the specified full `git` hash as the start point for `branch` (requires: `--branch-start-point`).
    /// If `branch` already exists and the start point hash is different, a new branch will be created.
    #[clap(long, requires = "branch_start_point")]
    pub branch_start_point_hash: Option<GitHash>,

    /// Reset `branch` to an empty state (conflicts with: `--branch-start-point`).
    /// If `branch` already exists, a new empty branch will be created.
    #[clap(long, conflicts_with = "branch_start_point")]
    pub branch_reset: bool,

    /// Deprecated: Do not use. This will soon be removed.
    #[clap(long, alias = "else-branch", alias = "endif-branch")]
    pub deprecated: bool,
}

#[derive(Args, Debug)]
#[clap(group(
    ArgGroup::new("run_hash")
        .multiple(false)
        .args(&["hash", "no_hash"]),
))]
pub struct CliRunHash {
    /// `git` commit hash (defaults to HEAD)
    #[clap(long)]
    pub hash: Option<GitHash>,

    /// Do not try to find a `git` commit hash
    #[clap(long)]
    pub no_hash: bool,
}

#[derive(Args, Debug)]
pub struct CliRunCommand {
    /// Benchmark command output file path
    #[clap(long, conflicts_with = "file_size")]
    pub file: Option<Utf8PathBuf>,

    /// Track the size of a file at the given file path
    #[clap(long, conflicts_with = "file")]
    pub file_size: Option<Vec<Utf8PathBuf>>,

    #[clap(flatten)]
    pub sh_c: CliRunShell,

    /// Run as an executable not a shell command (default if args > 1)
    #[clap(long)]
    #[clap(
        requires = "command",
        conflicts_with = "shell",
        conflicts_with = "flag"
    )]
    pub exec: bool,

    /// Benchmark command
    #[clap(trailing_var_arg = true, allow_hyphen_values = true)]
    pub command: Option<Vec<String>>,
}

#[derive(Args, Debug)]
pub struct CliRunShell {
    /// Shell command path
    #[clap(long)]
    pub shell: Option<String>,

    /// Shell command flag
    #[clap(long)]
    pub flag: Option<String>,
}

/// Supported Adapters
#[derive(ValueEnum, Debug, Clone)]
#[clap(rename_all = "snake_case")]
pub enum CliRunAdapter {
    /// 🪄 Magic (default)
    Magic,
    /// {...} JSON
    Json,
    /// #️⃣ C#
    CSharp,
    /// #️⃣ C# `DotNet`
    CSharpDotNet,
    /// ➕ C++
    Cpp,
    /// ➕ C++ Catch2
    CppCatch2,
    /// ➕ C++ Google
    CppGoogle,
    /// 🕳 Go
    Go,
    /// 🕳 Go Bench
    GoBench,
    /// ☕️ Java
    Java,
    /// ☕️ Java JMH
    JavaJmh,
    /// 🕸 JavaScript
    Js,
    /// 🕸 JavaScript Benchmark
    JsBenchmark,
    /// 🕸 JavaScript Time
    JsTime,
    /// 🐍 Python
    Python,
    /// 🐍 Python ASV
    PythonAsv,
    /// 🐍 Python Pytest
    PythonPytest,
    /// ♦️ Ruby
    Ruby,
    /// ♦️ Ruby Benchmark
    RubyBenchmark,
    /// 🦀 Rust
    Rust,
    /// 🦀 Rust Bench
    RustBench,
    /// 🦀 Rust Criterion
    RustCriterion,
    /// 🦀 Rust Iai
    RustIai,
    /// 🦀 Rust Iai-Callgrind
    RustIaiCallgrind,
    /// ❯_ Shell
    Shell,
    /// ❯_ Shell Hyperfine
    ShellHyperfine,
}

/// Suggested Central Tendency (Average)
#[derive(ValueEnum, Debug, Clone)]
#[clap(rename_all = "snake_case")]
pub enum CliRunAverage {
    /// Mean and standard deviation
    Mean,
    /// Median and interquartile range
    Median,
}

/// Supported Fold Operations
#[derive(ValueEnum, Debug, Clone)]
#[clap(rename_all = "snake_case")]
pub enum CliRunFold {
    /// Minimum value
    Min,
    /// Maximum value
    Max,
    /// Mean of values
    Mean,
    /// Median of values
    Median,
}

#[derive(Args, Debug)]
pub struct CliRunFmt {
    /// Output the final Report JSON at the given file path
    #[clap(long)]
    pub json_file: Option<Utf8PathBuf>,
    /// Output the final Report as HTML at the given file path
    #[clap(long)]
    pub html_file: Option<Utf8PathBuf>,
    /// Quite mode, only output the final Report JSON to standard out
    #[clap(short, long)]
    pub quiet: bool,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Args, Debug)]
#[clap(group(
    ArgGroup::new("ci_cd")
        .multiple(false)
        .args(&["github_actions"]),
))]
pub struct CliRunCi {
    /// GitHub API authentication token for GitHub Actions to comment on PRs (ie `--github-actions ${{ secrets.GITHUB_TOKEN }}`)
    #[clap(long)]
    pub github_actions: Option<String>,
    /// Only post results to CI if a Threshold exists for the Branch, Testbed, and Measure (requires: `--github-actions`)
    #[clap(long, requires = "ci_cd")]
    pub ci_only_thresholds: bool,
    /// Only start posting results to CI if an Alert is generated (requires: `--github-actions`)
    #[clap(long, requires = "ci_cd")]
    pub ci_only_on_alert: bool,
    /// All links should be to public URLs that do not require a login (requires: `--github-actions`)
    #[clap(long, requires = "ci_cd")]
    pub ci_public_links: bool,
    /// Custom ID for posting results to CI (requires: `--github-actions`)
    #[clap(long, requires = "ci_cd")]
    pub ci_id: Option<String>,
    /// Issue number for posting results to CI (requires: `--github-actions`)
    #[clap(long, requires = "ci_cd")]
    pub ci_number: Option<u64>,
    /// CAUTION: Override safety checks and accept that you are vulnerable to pwn requests (requires: `--github-actions`)
    #[clap(long, requires = "ci_cd")]
    pub ci_i_am_vulnerable_to_pwn_requests: bool,
    /// Deprecated: Do not use. This will soon be removed.
    #[clap(long, alias = "ci-no-metrics")]
    pub ci_deprecated: bool,
}
