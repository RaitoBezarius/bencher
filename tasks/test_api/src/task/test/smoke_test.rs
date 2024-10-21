use std::sync::LazyLock;
use std::{
    net::TcpStream,
    process::{Child, Command},
    thread,
    time::Duration,
};

use bencher_json::{
    JsonApiVersion, Jwt, Url, DEV_BENCHER_API_URL, LOCALHOST_BENCHER_API_URL, PROD_BENCHER_API_URL,
    TEST_BENCHER_API_URL,
};

use crate::{
    parser::{TaskExamples, TaskSeedTest, TaskSmokeTest, TaskTestEnvironment},
    task::test::{examples::Examples, seed_test::SeedTest},
    API_VERSION,
};

const DEV_BENCHER_API_TOKEN_STR: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJhdWQiOiJhcGlfa2V5IiwiZXhwIjo1OTkzNjQyMTU2LCJpYXQiOjE2OTg2NzQ4NjEsImlzcyI6Imh0dHBzOi8vZGV2ZWwtLWJlbmNoZXIubmV0bGlmeS5hcHAvIiwic3ViIjoibXVyaWVsLmJhZ2dlQG5vd2hlcmUuY29tIiwib3JnIjpudWxsfQ.9z7jmM53TcVzc1inDxTeX9_OR0PQPpZAsKsCE7lWHfo";
pub static DEV_BENCHER_API_TOKEN: LazyLock<Jwt> =
    LazyLock::new(|| DEV_BENCHER_API_TOKEN_STR.parse().expect("Invalid test JWT"));

#[derive(Debug)]
pub struct SmokeTest {
    pub environment: Environment,
}

#[derive(Debug, Clone, Copy)]
pub enum Environment {
    Ci,
    Localhost,
    Docker,
    Dev,
    Test,
    Prod,
}

impl TryFrom<TaskSmokeTest> for SmokeTest {
    type Error = anyhow::Error;

    fn try_from(test: TaskSmokeTest) -> Result<Self, Self::Error> {
        let TaskSmokeTest { environment } = test;
        Ok(Self {
            environment: environment.unwrap_or_default().into(),
        })
    }
}

impl From<TaskTestEnvironment> for Environment {
    fn from(environment: TaskTestEnvironment) -> Self {
        match environment {
            TaskTestEnvironment::Ci => Self::Ci,
            TaskTestEnvironment::Localhost => Self::Localhost,
            TaskTestEnvironment::Docker => Self::Docker,
            TaskTestEnvironment::Dev => Self::Dev,
            TaskTestEnvironment::Test => Self::Test,
            TaskTestEnvironment::Prod => Self::Prod,
        }
    }
}

impl SmokeTest {
    pub fn exec(&self) -> anyhow::Result<()> {
        let child = match self.environment {
            Environment::Ci | Environment::Localhost => Some(api_run()?),
            Environment::Docker => bencher_up().map(|()| None)?,
            Environment::Dev | Environment::Test | Environment::Prod => None,
        };

        let api_url = self.environment.as_url();
        test_api_version(&api_url)?;

        match self.environment {
            Environment::Ci => {
                test(&api_url, None, false)?;
                kill_child(child)?;
            },
            Environment::Localhost => {
                test(&api_url, None, true)?;
                kill_child(child)?;
            },
            Environment::Docker => bencher_down()?,
            Environment::Dev => test(&api_url, Some(&DEV_BENCHER_API_TOKEN), false)?,
            Environment::Test | Environment::Prod => {},
        }

        Ok(())
    }
}

impl Environment {
    fn as_url(self) -> Url {
        match self {
            Self::Ci | Self::Localhost | Self::Docker => LOCALHOST_BENCHER_API_URL.clone(),
            Self::Dev => DEV_BENCHER_API_URL.clone(),
            Self::Test => TEST_BENCHER_API_URL.clone(),
            Self::Prod => PROD_BENCHER_API_URL.clone(),
        }
        .into()
    }
}

fn api_run() -> anyhow::Result<Child> {
    let child = Command::new("cargo")
        .args(["run"])
        .current_dir("./services/api")
        .spawn()?;

    while TcpStream::connect("localhost:61016").is_err() {
        thread::sleep(Duration::from_secs(1));
        println!("Waiting for API server to start...");
    }

    Ok(child)
}

fn bencher_up() -> anyhow::Result<()> {
    // Use the `latest`` image tag so this test doesn't fail when releasing a new version.
    let status = Command::new("cargo")
        .args(["run", "--", "up", "--detach", "--tag", "latest", "api"])
        .current_dir("./services/cli")
        .status()?;
    assert!(status.success(), "{status}");

    while TcpStream::connect("localhost:61016").is_err() {
        thread::sleep(Duration::from_secs(1));
        println!("Waiting for API server to start...");
    }

    Ok(())
}

fn bencher_down() -> anyhow::Result<()> {
    let status = Command::new("cargo")
        .args(["run", "--", "down", "api"])
        .current_dir("./services/cli")
        .status()?;
    assert!(status.success(), "{status}");

    Ok(())
}

fn test_api_version(api_url: &Url) -> anyhow::Result<()> {
    println!("Testing API deploy is version {API_VERSION} at {api_url}");

    let output = Command::new("cargo")
        .args(["run", "--", "server", "version", "--host", api_url.as_ref()])
        .current_dir("./services/cli")
        .output()?;

    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    println!("{}", String::from_utf8_lossy(&output.stdout));
    output.status.success().then_some(()).ok_or_else(|| {
        anyhow::anyhow!(
            "Failed to get server version. Exit code: {:?}",
            output.status.code()
        )
    })?;

    let api_version =
        serde_json::from_str::<JsonApiVersion>(std::str::from_utf8(&output.stdout)?)?.version;
    if api_version != API_VERSION {
        return Err(anyhow::anyhow!(
            "API version {api_version} does not match current version {API_VERSION}"
        ));
    }

    Ok(())
}

fn test(api_url: &Url, token: Option<&Jwt>, run_examples: bool) -> anyhow::Result<()> {
    seed(api_url, token)?;
    if run_examples {
        examples(api_url, token)?;
    }
    Ok(())
}

fn seed(api_url: &Url, token: Option<&Jwt>) -> anyhow::Result<()> {
    println!("Seeding API deploy at {api_url}");
    let seed_test = SeedTest::try_from(TaskSeedTest {
        url: Some(api_url.clone()),
        token: token.cloned(),
    })?;
    seed_test.exec()
}

fn examples(api_url: &Url, token: Option<&Jwt>) -> anyhow::Result<()> {
    println!("Running examples at {api_url}");
    let examples = Examples::try_from(TaskExamples {
        url: Some(api_url.clone()),
        token: token.cloned(),
        example: None,
    })?;
    examples.exec()
}

fn kill_child(child: Option<Child>) -> anyhow::Result<()> {
    child
        .expect("Child process is expected for `localhost`")
        .kill()
        .map_err(Into::into)
}
