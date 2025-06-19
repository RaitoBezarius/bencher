use bencher_json::{BenchmarkName, JsonNewMetric, project::report::JsonAverage};
use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{anychar, space1},
    combinator::{eof, map, map_res},
    multi::many_till,
    sequence::{delimited, tuple},
};

use crate::{
    Adaptable, Settings,
    adapters::util::{NomError, Units, latency_as_nanos, parse_benchmark_name_chars, parse_f64},
    results::adapter_results::AdapterResults,
};

pub struct AdapterRubyBenchmark;

impl Adaptable for AdapterRubyBenchmark {
    fn parse(input: &str, settings: Settings) -> Option<AdapterResults> {
        if let Some(JsonAverage::Mean | JsonAverage::Median) = settings.average {
            return None;
        }

        let mut benchmark_metrics = Vec::new();

        let mut header = false;
        for line in input.lines() {
            if !header {
                header = parse_header(line).is_ok();
                continue;
            }

            if let Ok((remainder, benchmark_metric)) = parse_ruby(line) {
                if remainder.is_empty() {
                    benchmark_metrics.push(benchmark_metric);
                    continue;
                }
            }

            header = false;
        }

        AdapterResults::new_latency(benchmark_metrics)
    }
}

fn parse_header(input: &str) -> IResult<&str, ()> {
    map(
        tuple((
            space1,
            tag("user"),
            space1,
            tag("system"),
            space1,
            tag("total"),
            space1,
            tag("real"),
            eof,
        )),
        |_| (),
    )(input)
}

fn parse_ruby(input: &str) -> IResult<&str, (BenchmarkName, JsonNewMetric)> {
    map_res(
        many_till(anychar, parse_ruby_benchmark),
        |(name, json_metric)| -> Result<(BenchmarkName, JsonNewMetric), NomError> {
            let benchmark_name = parse_benchmark_name_chars(&name)?;
            Ok((benchmark_name, json_metric))
        },
    )(input)
}

fn parse_ruby_benchmark(input: &str) -> IResult<&str, JsonNewMetric> {
    map_res(
        tuple((
            space1,
            parse_f64,
            space1,
            parse_f64,
            space1,
            parse_f64,
            space1,
            delimited(tag("("), tuple((space1, parse_f64)), tag(")")),
            eof,
        )),
        |(_, _user, _, _system, _, _total, _, (_, real), _)| -> Result<JsonNewMetric, NomError> {
            let units = Units::Sec;
            let value = latency_as_nanos(real, units);
            Ok(JsonNewMetric {
                value,
                lower_value: None,
                upper_value: None,
            })
        },
    )(input)
}

#[cfg(test)]
pub(crate) mod test_ruby_benchmark {
    use bencher_json::project::report::JsonAverage;
    use pretty_assertions::assert_eq;

    use crate::{
        AdapterResults, Settings,
        adapters::test_util::{convert_file_path, opt_convert_file_path, validate_latency},
    };

    use super::AdapterRubyBenchmark;

    fn convert_ruby_benchmark(suffix: &str) -> AdapterResults {
        let file_path = format!("./tool_output/ruby/benchmark/{suffix}.txt");
        convert_file_path::<AdapterRubyBenchmark>(&file_path)
    }

    #[test]
    fn test_adapter_ruby_average() {
        let file_path = "./tool_output/ruby/benchmark/two.txt";
        assert_eq!(
            None,
            opt_convert_file_path::<AdapterRubyBenchmark>(
                file_path,
                Settings {
                    average: Some(JsonAverage::Mean)
                }
            )
        );

        assert_eq!(
            None,
            opt_convert_file_path::<AdapterRubyBenchmark>(
                file_path,
                Settings {
                    average: Some(JsonAverage::Median)
                }
            )
        );
    }

    #[test]
    fn test_adapter_ruby_benchmark_two() {
        let results = convert_ruby_benchmark("two");
        assert_eq!(results.inner.len(), 2);

        let metrics = results.get("sort!").unwrap();
        validate_latency(metrics, 1_460_465_000.0, None, None);

        let metrics = results.get("sort").unwrap();
        validate_latency(metrics, 1_448_327_000.0, None, None);
    }

    #[test]
    fn test_adapter_ruby_benchmark_five() {
        let results = convert_ruby_benchmark("five");
        validate_adapter_ruby_benchmark(&results);
    }

    pub fn validate_adapter_ruby_benchmark(results: &AdapterResults) {
        assert_eq!(results.inner.len(), 5);

        let metrics = results.get("for:").unwrap();
        validate_latency(metrics, 952_039_000.0, None, None);

        let metrics = results.get("times:").unwrap();
        validate_latency(metrics, 984_938_000.0, None, None);

        let metrics = results.get("upto:").unwrap();
        validate_latency(metrics, 946_787_000.0, None, None);

        let metrics = results.get(">total:").unwrap();
        validate_latency(metrics, 2_883_764_000.0, None, None);

        let metrics = results.get(">avg:").unwrap();
        validate_latency(metrics, 961_255_000.0, None, None);
    }
}
