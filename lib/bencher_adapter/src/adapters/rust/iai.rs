use bencher_json::{
    project::{
        measure::built_in::{
            iai::{EstimatedCycles, Instructions, L1Accesses, L2Accesses, RamAccesses},
            BuiltInMeasure as _,
        },
        report::JsonAverage,
    },
    BenchmarkName, JsonNewMetric,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{space0, space1},
    combinator::{eof, map},
    sequence::{delimited, tuple},
    IResult,
};

use crate::{
    adapters::util::{parse_f64, parse_u64},
    results::adapter_results::{AdapterResults, IaiMeasure},
    Adaptable, Settings,
};

pub struct AdapterRustIai;

const IAI_METRICS_LINE_COUNT: usize = 6;

impl Adaptable for AdapterRustIai {
    fn parse(input: &str, settings: Settings) -> Option<AdapterResults> {
        match settings.average {
            None => {},
            Some(JsonAverage::Mean | JsonAverage::Median) => return None,
        }

        let mut benchmark_metrics = Vec::new();
        let lines = input.lines().collect::<Vec<_>>();
        for lines in lines.windows(IAI_METRICS_LINE_COUNT) {
            let Ok(lines) = lines.try_into() else {
                debug_assert!(
                    false,
                    "Windows struct should always be convertible to array of the same size."
                );
                continue;
            };
            if let Some((benchmark_name, metrics)) = parse_iai_lines(lines) {
                benchmark_metrics.push((benchmark_name, metrics));
            }
        }

        AdapterResults::new_iai(benchmark_metrics)
    }
}

fn parse_iai_lines(
    lines: [&str; IAI_METRICS_LINE_COUNT],
) -> Option<(BenchmarkName, Vec<IaiMeasure>)> {
    let [benchmark_name_line, instructions_line, l1_accesses_line, l2_accesses_line, ram_accesses_line, estimated_cycles_line] =
        lines;

    let name = benchmark_name_line.parse().ok()?;
    #[expect(trivial_casts)]
    let metrics = [
        (
            Instructions::NAME_STR,
            instructions_line,
            IaiMeasure::Instructions as fn(JsonNewMetric) -> IaiMeasure,
        ),
        (
            L1Accesses::NAME_STR,
            l1_accesses_line,
            IaiMeasure::L1Accesses,
        ),
        (
            L2Accesses::NAME_STR,
            l2_accesses_line,
            IaiMeasure::L2Accesses,
        ),
        (
            RamAccesses::NAME_STR,
            ram_accesses_line,
            IaiMeasure::RamAccesses,
        ),
        (
            EstimatedCycles::NAME_STR,
            estimated_cycles_line,
            IaiMeasure::EstimatedCycles,
        ),
    ]
    .into_iter()
    .map(|(measure, input, into_variant)| {
        parse_iai_metric(input, measure).map(|(_remainder, json_metric)| into_variant(json_metric))
    })
    .collect::<Result<Vec<_>, _>>()
    .ok()?;

    Some((name, metrics))
}

#[expect(clippy::cast_precision_loss)]
fn parse_iai_metric<'a>(input: &'a str, measure: &'static str) -> IResult<&'a str, JsonNewMetric> {
    map(
        tuple((
            space0,
            tag(measure),
            tag(":"),
            space1,
            parse_u64,
            alt((
                map(eof, |_| ()),
                map(
                    tuple((
                        space1,
                        delimited(
                            tag("("),
                            alt((
                                map(tag("No change"), |_| ()),
                                map(
                                    tuple((alt((tag("+"), tag("-"))), parse_f64, tag("%"))),
                                    |_| (),
                                ),
                            )),
                            tag(")"),
                        ),
                        eof,
                    )),
                    |_| (),
                ),
            )),
        )),
        |(_, _, _, _, metric, ())| JsonNewMetric {
            value: (metric as f64).into(),
            lower_value: None,
            upper_value: None,
        },
    )(input)
}

#[cfg(test)]
pub(crate) mod test_rust_iai {

    use crate::{
        adapters::test_util::convert_file_path, results::adapter_metrics::AdapterMetrics,
        Adaptable as _, AdapterResults,
    };
    use bencher_json::{
        project::measure::built_in::{
            iai::{EstimatedCycles, Instructions, L1Accesses, L2Accesses, RamAccesses},
            BuiltInMeasure as _,
        },
        JsonNewMetric,
    };
    use ordered_float::OrderedFloat;
    use pretty_assertions::assert_eq;

    use super::AdapterRustIai;

    fn convert_rust_iai(suffix: &str) -> AdapterResults {
        let file_path = format!("./tool_output/rust/iai/{suffix}.txt");
        convert_file_path::<AdapterRustIai>(&file_path)
    }

    pub fn validate_iai(metrics: &AdapterMetrics, results: [(&str, f64); 5]) {
        assert_eq!(metrics.inner.len(), 5);
        for (key, value) in results {
            let metric = metrics.get(key).unwrap();
            assert_eq!(metric.value, OrderedFloat::from(value));
            assert_eq!(metric.lower_value, None);
            assert_eq!(metric.upper_value, None);
        }
    }

    #[test]
    fn test_adapter_rust_iai_parse_line() {
        assert_eq!(
            super::parse_iai_metric("  Instructions:  1234", Instructions::NAME_STR),
            Ok((
                "",
                JsonNewMetric {
                    value: 1234.0.into(),
                    upper_value: None,
                    lower_value: None
                }
            ))
        );

        assert_eq!(
            super::parse_iai_metric("  Instructions:  1234 (No change)", Instructions::NAME_STR),
            Ok((
                "",
                JsonNewMetric {
                    value: 1234.0.into(),
                    upper_value: None,
                    lower_value: None
                }
            ))
        );

        assert_eq!(
            super::parse_iai_metric("  Instructions:  1234 (+3.14%)", Instructions::NAME_STR),
            Ok((
                "",
                JsonNewMetric {
                    value: 1234.0.into(),
                    upper_value: None,
                    lower_value: None
                }
            ))
        );
    }

    #[test]
    fn test_adapter_rust_iai_parse_multiple_lines() {
        let input = "bench_fibonacci_short
  Instructions:                1735
  L1 Accesses:                 2364
  L2 Accesses:                    1
  RAM Accesses:                   1
  Estimated Cycles:            2404";
        let output = AdapterRustIai::parse(input, crate::Settings::default());
        assert!(output.is_some());
    }

    #[test]
    fn test_adapter_rust_iai() {
        let results = convert_rust_iai("two");
        validate_adapter_rust_iai(&results);
    }

    pub fn validate_adapter_rust_iai(results: &AdapterResults) {
        assert_eq!(results.inner.len(), 2);

        let metrics = results.get("bench_fibonacci_short").unwrap();
        validate_iai(
            metrics,
            [
                (Instructions::SLUG_STR, 1735.0),
                (L1Accesses::SLUG_STR, 2364.0),
                (L2Accesses::SLUG_STR, 1.0),
                (RamAccesses::SLUG_STR, 1.0),
                (EstimatedCycles::SLUG_STR, 2404.0),
            ],
        );
        let metrics = results.get("bench_fibonacci_long").unwrap();
        validate_iai(
            metrics,
            [
                (Instructions::SLUG_STR, 26_214_735.0),
                (L1Accesses::SLUG_STR, 35_638_623.0),
                (L2Accesses::SLUG_STR, 2.0),
                (RamAccesses::SLUG_STR, 1.0),
                (EstimatedCycles::SLUG_STR, 35_638_668.0),
            ],
        );
    }

    #[test]
    fn test_adapter_rust_iai_change() {
        let results = convert_rust_iai("change");
        assert_eq!(results.inner.len(), 2);

        let metrics = results.get("iai_benchmark_short").unwrap();
        validate_iai(
            metrics,
            [
                (Instructions::SLUG_STR, 1243.0),
                (L1Accesses::SLUG_STR, 1580.0),
                (L2Accesses::SLUG_STR, 1.0),
                (RamAccesses::SLUG_STR, 2.0),
                (EstimatedCycles::SLUG_STR, 1655.0),
            ],
        );
        let metrics = results.get("iai_benchmark_long").unwrap();
        validate_iai(
            metrics,
            [
                (Instructions::SLUG_STR, 18_454_953.0),
                (L1Accesses::SLUG_STR, 23_447_195.0),
                (L2Accesses::SLUG_STR, 6.0),
                (RamAccesses::SLUG_STR, 2.0),
                (EstimatedCycles::SLUG_STR, 23_447_295.0),
            ],
        );
    }
}
