use std::fmt;

use bencher_valid::{DateTime, Index, ResourceName, Window};
#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};

use crate::{BenchmarkUuid, BranchUuid, MeasureUuid, ProjectUuid, TestbedUuid};

crate::typed_uuid::typed_uuid!(PlotUuid);

#[typeshare::typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[expect(clippy::struct_excessive_bools)]
pub struct JsonNewPlot {
    /// The index of the plot.
    /// Maximum index is 64.
    pub index: Option<Index>,
    /// The title of the plot.
    /// Maximum length is 64 characters.
    pub title: Option<ResourceName>,
    /// Display metric lower values.
    pub lower_value: bool,
    /// Display metric upper values.
    pub upper_value: bool,
    /// Display lower boundary limits.
    pub lower_boundary: bool,
    /// Display upper boundary limits.
    pub upper_boundary: bool,
    /// The x-axis to use for the plot.
    pub x_axis: XAxis,
    /// The window of time for the plot, in seconds.
    /// Metrics outside of this window will be omitted.
    pub window: Window,
    /// The branches to include in the plot.
    /// At least one branch must be specified.
    pub branches: Vec<BranchUuid>,
    /// The testbeds to include in the plot.
    /// At least one testbed must be specified.
    pub testbeds: Vec<TestbedUuid>,
    /// The benchmarks to include in the plot.
    /// At least one benchmark must be specified.
    pub benchmarks: Vec<BenchmarkUuid>,
    /// The measures to include in the plot.
    /// At least one measure must be specified.
    pub measures: Vec<MeasureUuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct JsonPlots(pub Vec<JsonPlot>);

crate::from_vec!(JsonPlots[JsonPlot]);

#[typeshare::typeshare]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[expect(clippy::struct_excessive_bools)]
pub struct JsonPlot {
    pub uuid: PlotUuid,
    pub project: ProjectUuid,
    pub title: Option<ResourceName>,
    pub lower_value: bool,
    pub upper_value: bool,
    pub lower_boundary: bool,
    pub upper_boundary: bool,
    pub x_axis: XAxis,
    pub window: Window,
    pub branches: Vec<BranchUuid>,
    pub testbeds: Vec<TestbedUuid>,
    pub benchmarks: Vec<BenchmarkUuid>,
    pub measures: Vec<MeasureUuid>,
    pub created: DateTime,
    pub modified: DateTime,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(untagged)]
pub enum JsonUpdatePlot {
    Patch(JsonPlotPatch),
    Null(JsonPlotPatchNull),
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct JsonPlotPatch {
    /// The new index for the plot.
    /// Maximum index is 64.
    pub index: Option<Index>,
    /// The new title of the plot.
    /// Set to `null` to remove the current title.
    /// Maximum length is 64 characters.
    pub title: Option<ResourceName>,
    /// The window of time for the plot, in seconds.
    /// Metrics outside of this window will be omitted.
    pub window: Option<Window>,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct JsonPlotPatchNull {
    pub index: Option<Index>,
    pub title: (),
    pub window: Option<Window>,
}

impl<'de> Deserialize<'de> for JsonUpdatePlot {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        const INDEX_FIELD: &str = "index";
        const TITLE_FIELD: &str = "title";
        const WINDOW_FIELD: &str = "window";
        const FIELDS: &[&str] = &[INDEX_FIELD, TITLE_FIELD, WINDOW_FIELD];

        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            Index,
            Title,
            Window,
        }

        struct UpdatePlotVisitor;

        impl<'de> Visitor<'de> for UpdatePlotVisitor {
            type Value = JsonUpdatePlot;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("JsonUpdatePlot")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                let mut index = None;
                let mut title = None;
                let mut window = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Index => {
                            if index.is_some() {
                                return Err(de::Error::duplicate_field(INDEX_FIELD));
                            }
                            index = Some(map.next_value()?);
                        },
                        Field::Title => {
                            if title.is_some() {
                                return Err(de::Error::duplicate_field(TITLE_FIELD));
                            }
                            title = Some(map.next_value()?);
                        },
                        Field::Window => {
                            if window.is_some() {
                                return Err(de::Error::duplicate_field(WINDOW_FIELD));
                            }
                            window = Some(map.next_value()?);
                        },
                    }
                }

                Ok(match title {
                    Some(Some(title)) => Self::Value::Patch(JsonPlotPatch {
                        index,
                        title: Some(title),
                        window,
                    }),
                    Some(None) => Self::Value::Null(JsonPlotPatchNull {
                        index,
                        title: (),
                        window,
                    }),
                    None => Self::Value::Patch(JsonPlotPatch {
                        index,
                        title: None,
                        window,
                    }),
                })
            }
        }

        deserializer.deserialize_struct("JsonUpdatePlot", FIELDS, UpdatePlotVisitor)
    }
}

#[typeshare::typeshare]
#[derive(Debug, Clone, Copy, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum PlotKey {
    LowerValue,
    UpperValue,
    LowerBoundary,
    UpperBoundary,
    XAxis,
}

pub const LOWER_VALUE: &str = "lower_value";
pub const UPPER_VALUE: &str = "upper_value";
pub const LOWER_BOUNDARY: &str = "lower_boundary";
pub const UPPER_BOUNDARY: &str = "upper_boundary";
pub const X_AXIS: &str = "x_axis";

const DATE_TIME_INT: i32 = 0;
const VERSION_INT: i32 = 1;

#[typeshare::typeshare]
#[derive(Debug, Clone, Copy, Default, derive_more::Display, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "db", derive(diesel::FromSqlRow, diesel::AsExpression))]
#[cfg_attr(feature = "db", diesel(sql_type = diesel::sql_types::Integer))]
#[serde(rename_all = "snake_case")]
#[repr(i32)]
pub enum XAxis {
    #[default]
    DateTime = DATE_TIME_INT,
    Version = VERSION_INT,
}

#[cfg(feature = "db")]
mod plot_x_axis {
    use super::{XAxis, DATE_TIME_INT, VERSION_INT};

    #[derive(Debug, thiserror::Error)]
    pub enum XAxisError {
        #[error("Invalid plot axis value: {0}")]
        Invalid(i32),
    }

    impl<DB> diesel::serialize::ToSql<diesel::sql_types::Integer, DB> for XAxis
    where
        DB: diesel::backend::Backend,
        i32: diesel::serialize::ToSql<diesel::sql_types::Integer, DB>,
    {
        fn to_sql<'b>(
            &'b self,
            out: &mut diesel::serialize::Output<'b, '_, DB>,
        ) -> diesel::serialize::Result {
            match self {
                Self::DateTime => DATE_TIME_INT.to_sql(out),
                Self::Version => VERSION_INT.to_sql(out),
            }
        }
    }

    impl<DB> diesel::deserialize::FromSql<diesel::sql_types::Integer, DB> for XAxis
    where
        DB: diesel::backend::Backend,
        i32: diesel::deserialize::FromSql<diesel::sql_types::Integer, DB>,
    {
        fn from_sql(bytes: DB::RawValue<'_>) -> diesel::deserialize::Result<Self> {
            match i32::from_sql(bytes)? {
                DATE_TIME_INT => Ok(Self::DateTime),
                VERSION_INT => Ok(Self::Version),
                value => Err(Box::new(XAxisError::Invalid(value))),
            }
        }
    }
}
