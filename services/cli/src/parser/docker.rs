use clap::{Parser, ValueEnum};
use once_cell::sync::Lazy;

use crate::CLI_VERSION;

pub static CLI_VERSION_TAG: Lazy<String> = Lazy::new(|| format!("v{CLI_VERSION}"));

#[derive(Parser, Debug)]
pub struct CliUp {
    /// Select the container to run.
    /// Similar to the `service` argument for `docker-compose up`.
    #[clap(default_value = "all")]
    pub service: CliService,

    /// Detached mode: Run containers in the background.
    /// Similar to the `--detach` flag for `docker compose up`.
    #[clap(short, long)]
    pub detach: bool,

    /// Pull image before running.
    /// Similar to the `--pull` option for `docker compose up`.
    #[clap(long, value_name = "WHEN", default_value = "always")]
    pub pull: CliUpPull,

    /// Specify the image tag.
    #[clap(long, default_value = CLI_VERSION_TAG.as_str())]
    pub tag: String,

    /// Specify a port number for the Console container.
    /// Similar to the `--expose` option for `docker run`.
    #[clap(long, value_name = "PORT", default_value = "3000")]
    pub console_port: u16,

    /// Specify a port number for the API container.
    /// Similar to the `--expose` option for `docker run`.
    #[clap(long, value_name = "PORT", default_value = "61016")]
    pub api_port: u16,

    /// Pass an environment variable to the Console container.
    /// Expected format is `KEY=value`.
    /// Similar to the `--env` option for `docker run`.
    #[clap(long, value_name = "KEY_VALUE", value_parser = check_env)]
    pub console_env: Option<Vec<String>>,

    /// Pass an environment variable to the API container.
    /// Expected format is `KEY=value`.
    /// Similar to the `--env` option for `docker run`.
    #[clap(long, value_name = "KEY_VALUE", value_parser = check_env)]
    pub api_env: Option<Vec<String>>,

    /// Pass a mount volume to the Console container.
    /// Expected format is `/host/path:/container/path`.
    /// Similar to the `--volume` option for `docker run`.
    #[clap(long, value_name = "HOST_CONTAINER", value_parser = check_volume)]
    pub console_volume: Option<Vec<String>>,

    /// Pass a mount volume to the API container.
    /// Expected format is `/host/path:/container/path`.
    /// Similar to the `--volume` option for `docker run`.
    #[clap(long, value_name = "HOST_CONTAINER", value_parser = check_volume)]
    pub api_volume: Option<Vec<String>>,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
#[clap(rename_all = "snake_case")]
pub enum CliService {
    All,
    Api,
    Console,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
#[clap(rename_all = "snake_case")]
pub enum CliUpPull {
    Always,
    Missing,
    Never,
}

#[derive(Parser, Debug)]
pub struct CliLogs {
    /// Select the container to view logs for.
    /// Similar to the `service` argument for `docker-compose logs`.
    #[clap(default_value = "all")]
    pub service: CliService,
}

#[derive(Parser, Debug)]
pub struct CliDown {
    /// Select the container to stop.
    /// Similar to the `service` argument for `docker-compose down`.
    #[clap(default_value = "all")]
    pub service: CliService,
}

fn check_env(arg: &str) -> Result<String, String> {
    check_key_value::<'='>("KEY", "VALUE", arg, false)
}

fn check_volume(arg: &str) -> Result<String, String> {
    check_key_value::<':'>("HOST", "CONTAINER", arg, true)
}

/// Check that input argument is in the form `left<separator>right`
fn check_key_value<const SEPARATOR: char>(
    left: &str,
    right: &str,
    arg: &str,
    require_right: bool,
) -> Result<String, String> {
    let index = arg.find(SEPARATOR)
        .ok_or_else(|| format!("Failed to parse argument, expected format `{left}{SEPARATOR}{right}` but no `{SEPARATOR}` was found in: `{arg}`"))?;
    if index == 0 {
        return Err(format!("Failed to parse argument, expected format `{left}{SEPARATOR}{right}` but no `{left}` was found in: `{arg}`"));
    }
    if require_right && index == arg.len() - 1 {
        return Err(format!("Failed to parse argument, expected format `{left}{SEPARATOR}{right}` but no `{right}` was found in: `{arg}`"));
    }
    Ok(arg.into())
}
