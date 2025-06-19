use bencher_json::{BenchmarkName, JsonAny, JsonNewMetric, project::report::JsonAverage};
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::{
    Adaptable, AdapterError, Settings,
    adapters::util::{Units, latency_as_nanos},
    results::adapter_results::AdapterResults,
};

pub struct AdapterCppGoogle;

impl Adaptable for AdapterCppGoogle {
    fn parse(input: &str, settings: Settings) -> Option<AdapterResults> {
        match settings.average {
            Some(JsonAverage::Mean) | None => {},
            Some(JsonAverage::Median) => return None,
        }

        serde_json::from_str::<Google>(input)
            .ok()?
            .try_into()
            .ok()?
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Google {
    pub context: Context,
    pub benchmarks: Vec<Benchmark>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Context {
    pub caches: Vec<JsonAny>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Benchmark {
    pub name: BenchmarkName,
    #[serde(with = "rust_decimal::serde::float")]
    pub real_time: Decimal,
    pub time_unit: Units,
}

impl TryFrom<Google> for Option<AdapterResults> {
    type Error = AdapterError;

    fn try_from(google: Google) -> Result<Self, Self::Error> {
        let mut benchmark_metrics = Vec::with_capacity(google.benchmarks.len());
        for benchmark in google.benchmarks {
            let Benchmark {
                name,
                real_time,
                time_unit,
            } = benchmark;
            let value = latency_as_nanos(real_time, time_unit);
            let json_metric = JsonNewMetric {
                value,
                lower_value: None,
                upper_value: None,
            };

            benchmark_metrics.push((name, json_metric));
        }

        Ok(AdapterResults::new_latency(benchmark_metrics))
    }
}

#[cfg(test)]
pub(crate) mod test_cpp_google {
    use bencher_json::project::report::JsonAverage;
    use pretty_assertions::assert_eq;

    use crate::{
        AdapterResults, Settings,
        adapters::test_util::{convert_file_path, opt_convert_file_path, validate_latency},
    };

    use super::AdapterCppGoogle;

    fn convert_cpp_google(suffix: &str) -> AdapterResults {
        let file_path = format!("./tool_output/cpp/google/{suffix}.txt");
        convert_file_path::<AdapterCppGoogle>(&file_path)
    }

    #[test]
    fn test_adapter_cpp_google_average() {
        let file_path = "./tool_output/cpp/google/two.txt";
        let results = opt_convert_file_path::<AdapterCppGoogle>(
            file_path,
            Settings {
                average: Some(JsonAverage::Mean),
            },
        )
        .unwrap();
        validate_adapter_cpp_google(&results);

        assert_eq!(
            None,
            opt_convert_file_path::<AdapterCppGoogle>(
                file_path,
                Settings {
                    average: Some(JsonAverage::Median)
                }
            )
        );
    }

    #[test]
    fn test_adapter_cpp_google() {
        let results = convert_cpp_google("two");
        validate_adapter_cpp_google(&results);
    }

    pub fn validate_adapter_cpp_google(results: &AdapterResults) {
        assert_eq!(results.inner.len(), 2);

        let metrics = results.get("fib_10").unwrap();
        validate_latency(metrics, 214.989_801_145_479_53, None, None);

        let metrics = results.get("fib_20").unwrap();
        validate_latency(metrics, 27_455.600_415_007_055, None, None);
    }
}
