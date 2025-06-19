use bencher_json::{BenchmarkName, JsonNewMetric, project::report::JsonAverage};
use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, take_till1},
    character::complete::{anychar, space1},
    combinator::{eof, map, map_res},
    multi::many_till,
    sequence::tuple,
};

use crate::{
    Adaptable, Settings,
    adapters::util::{
        NomError, latency_as_nanos, parse_benchmark_name, parse_f64, parse_u64, parse_units,
    },
    results::adapter_results::AdapterResults,
};

pub struct AdapterGoBench;

impl Adaptable for AdapterGoBench {
    fn parse(input: &str, settings: Settings) -> Option<AdapterResults> {
        match settings.average {
            Some(JsonAverage::Mean) | None => {},
            Some(JsonAverage::Median) => return None,
        }

        let mut benchmark_metrics = Vec::new();

        for line in input.lines() {
            if let Ok((remainder, benchmark_metric)) = parse_go(line) {
                if remainder.is_empty() {
                    benchmark_metrics.push(benchmark_metric);
                }
            }
        }

        AdapterResults::new_latency(benchmark_metrics)
    }
}

fn parse_go(input: &str) -> IResult<&str, (BenchmarkName, JsonNewMetric)> {
    map_res(
        tuple((
            take_till1(|c| c == ' ' || c == '\t'),
            space1,
            parse_u64,
            space1,
            parse_go_bench,
            alt((
                map(eof, |_| ()),
                map(
                    tuple((space1, parse_f64, space1, many_till(anychar, eof))),
                    |_| (),
                ),
            )),
        )),
        |(name, _, _iter, _, json_metric, ())| -> Result<(BenchmarkName, JsonNewMetric), NomError> {
            let benchmark_name = parse_benchmark_name(name)?;
            Ok((benchmark_name, json_metric))
        },
    )(input)
}

fn parse_go_bench(input: &str) -> IResult<&str, JsonNewMetric> {
    map_res(
        tuple((parse_f64, space1, parse_units, tag("/op"))),
        |(duration, _, units, _)| -> Result<JsonNewMetric, NomError> {
            let value = latency_as_nanos(duration, units);
            Ok(JsonNewMetric {
                value,
                lower_value: None,
                upper_value: None,
            })
        },
    )(input)
}

#[cfg(test)]
pub(crate) mod test_go_bench {
    use bencher_json::{JsonNewMetric, project::report::JsonAverage};
    use pretty_assertions::assert_eq;

    use crate::{
        AdapterResults, Settings,
        adapters::test_util::{convert_file_path, opt_convert_file_path, validate_latency},
    };

    use super::{AdapterGoBench, parse_go};

    fn convert_go_bench(suffix: &str) -> AdapterResults {
        let file_path = format!("./tool_output/go/bench/{suffix}.txt");
        convert_file_path::<AdapterGoBench>(&file_path)
    }

    #[test]
    fn test_parse_go() {
        for (index, (expected, input)) in [
            (
                Ok((
                    "",
                    (
                        "BenchmarkFib10-8".parse().unwrap(),
                        JsonNewMetric {
                            value: 325.0.into(),
                            lower_value: None,
                            upper_value: None,
                        },
                    ),
                )),
                "BenchmarkFib10-8   		 					5000000		325 ns/op",
            ),
            (
                Ok((
                    "",
                    (
                        "BenchmarkFib20".parse().unwrap(),
                        JsonNewMetric {
                            value: 40_537.123.into(),
                            lower_value: None,
                            upper_value: None,
                        },
                    ),
                )),
                "BenchmarkFib20  	 	   					30000		40537.123 ns/op",
            ),
            (
                Ok((
                    "",
                    (
                        "BenchmarkFib/my_tabled_benchmark_-_10-8".parse().unwrap(),
                        JsonNewMetric {
                            value: 325.0.into(),
                            lower_value: None,
                            upper_value: None,
                        },
                    ),
                )),
                "BenchmarkFib/my_tabled_benchmark_-_10-8    	5000000		325 ns/op",
            ),
            (
                Ok((
                    "",
                    (
                        "BenchmarkFib/my_tabled_benchmark_-_20".parse().unwrap(),
                        JsonNewMetric {
                            value: 40_537.123.into(),
                            lower_value: None,
                            upper_value: None,
                        },
                    ),
                )),
                "BenchmarkFib/my_tabled_benchmark_-_20		30000		40537.123 ns/op",
            ),
            (
                Ok((
                    "",
                    (
                        "BenchmarkFib/my/tabled/benchmark_-_20".parse().unwrap(),
                        JsonNewMetric {
                            value: 40_537.456.into(),
                            lower_value: None,
                            upper_value: None,
                        },
                    ),
                )),
                "BenchmarkFib/my/tabled/benchmark_-_20		30001		40537.456 ns/op",
            ),
            (
                Ok((
                    "",
                    (
                        "BenchmarkFib20WithAuxMetric-8".parse().unwrap(),
                        JsonNewMetric {
                            value: 25_829.0.into(),
                            lower_value: None,
                            upper_value: None,
                        },
                    ),
                )),
                "BenchmarkFib20WithAuxMetric-8              46714             25829 ns/op                 4.000 auxMetricUnits",
            ),
        ]
        .into_iter()
        .enumerate()
        {
            assert_eq!(expected, parse_go(input), "#{index}: {input}");
        }
    }

    #[test]
    fn test_adapter_go_bench_average() {
        let file_path = "./tool_output/go/bench/five.txt";
        let results = opt_convert_file_path::<AdapterGoBench>(
            file_path,
            Settings {
                average: Some(JsonAverage::Mean),
            },
        )
        .unwrap();
        validate_adapter_go_bench(&results);

        assert_eq!(
            None,
            opt_convert_file_path::<AdapterGoBench>(
                file_path,
                Settings {
                    average: Some(JsonAverage::Median)
                }
            )
        );
    }

    #[test]
    fn test_adapter_go_bench() {
        let results = convert_go_bench("five");
        validate_adapter_go_bench(&results);
    }

    pub fn validate_adapter_go_bench(results: &AdapterResults) {
        assert_eq!(results.inner.len(), 5);

        let metrics = results.get("BenchmarkFib10-8").unwrap();
        validate_latency(metrics, 325.0, None, None);

        let metrics = results.get("BenchmarkFib20").unwrap();
        validate_latency(metrics, 40_537.123, None, None);

        let metrics = results
            .get("BenchmarkFib/my_tabled_benchmark_-_10-8")
            .unwrap();
        validate_latency(metrics, 325.0, None, None);

        let metrics = results
            .get("BenchmarkFib/my_tabled_benchmark_-_20")
            .unwrap();
        validate_latency(metrics, 40_537.123, None, None);

        let metrics = results
            .get("BenchmarkFib/my/tabled/benchmark_-_20")
            .unwrap();
        validate_latency(metrics, 40_537.456, None, None);
    }

    #[test]
    fn test_adapter_go_bench_three() {
        let results = convert_go_bench("three");
        assert_eq!(results.inner.len(), 3);

        let metrics = results.get("BenchmarkFib10-8").unwrap();
        validate_latency(metrics, 210.2, None, None);

        let metrics = results.get("BenchmarkFib20-8").unwrap();
        validate_latency(metrics, 26264.0, None, None);

        let metrics = results.get("BenchmarkFib20WithAuxMetric-8").unwrap();
        validate_latency(metrics, 25829.0, None, None);
    }
}
