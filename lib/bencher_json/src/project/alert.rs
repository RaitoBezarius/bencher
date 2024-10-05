#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{DateTime, JsonBenchmark, JsonBoundary, JsonMetric, JsonThreshold};

use super::{boundary::BoundaryLimit, report::Iteration, report::ReportUuid};

crate::typed_uuid::typed_uuid!(AlertUuid);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct JsonAlerts(pub Vec<JsonAlert>);

crate::from_vec!(JsonAlerts[JsonAlert]);

#[typeshare::typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct JsonAlert {
    pub uuid: AlertUuid,
    pub report: ReportUuid,
    pub iteration: Iteration,
    pub benchmark: JsonBenchmark,
    pub metric: JsonMetric,
    pub threshold: JsonThreshold,
    pub boundary: JsonBoundary,
    pub limit: BoundaryLimit,
    pub status: AlertStatus,
    pub created: DateTime,
    pub modified: DateTime,
}

const ACTIVE_INT: i32 = 0;
const DISMISSED_INT: i32 = 1;
const SILENCED_INT: i32 = 10;

#[typeshare::typeshare]
#[derive(Debug, Clone, Copy, Default, derive_more::Display, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "db", derive(diesel::FromSqlRow, diesel::AsExpression))]
#[cfg_attr(feature = "db", diesel(sql_type = diesel::sql_types::Integer))]
#[serde(rename_all = "snake_case")]
#[repr(i32)]
pub enum AlertStatus {
    #[default]
    /// The alert is active.
    Active = ACTIVE_INT,
    /// The alert has been dismissed by a user.
    Dismissed = DISMISSED_INT,
    /// The alert has been silenced by the system.
    Silenced = SILENCED_INT,
}

impl From<UpdateAlertStatus> for AlertStatus {
    fn from(status: UpdateAlertStatus) -> Self {
        match status {
            UpdateAlertStatus::Active => Self::Active,
            UpdateAlertStatus::Dismissed => Self::Dismissed,
        }
    }
}

#[cfg(feature = "db")]
mod alert_status {
    use super::{AlertStatus, ACTIVE_INT, DISMISSED_INT, SILENCED_INT};

    #[derive(Debug, thiserror::Error)]
    pub enum AlertStatusError {
        #[error("Invalid alert status value: {0}")]
        Invalid(i32),
    }

    impl<DB> diesel::serialize::ToSql<diesel::sql_types::Integer, DB> for AlertStatus
    where
        DB: diesel::backend::Backend,
        i32: diesel::serialize::ToSql<diesel::sql_types::Integer, DB>,
    {
        fn to_sql<'b>(
            &'b self,
            out: &mut diesel::serialize::Output<'b, '_, DB>,
        ) -> diesel::serialize::Result {
            match self {
                Self::Active => ACTIVE_INT.to_sql(out),
                Self::Dismissed => DISMISSED_INT.to_sql(out),
                Self::Silenced => SILENCED_INT.to_sql(out),
            }
        }
    }

    impl<DB> diesel::deserialize::FromSql<diesel::sql_types::Integer, DB> for AlertStatus
    where
        DB: diesel::backend::Backend,
        i32: diesel::deserialize::FromSql<diesel::sql_types::Integer, DB>,
    {
        fn from_sql(bytes: DB::RawValue<'_>) -> diesel::deserialize::Result<Self> {
            match i32::from_sql(bytes)? {
                ACTIVE_INT => Ok(Self::Active),
                DISMISSED_INT => Ok(Self::Dismissed),
                SILENCED_INT => Ok(Self::Silenced),
                value => Err(Box::new(AlertStatusError::Invalid(value))),
            }
        }
    }
}

#[typeshare::typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct JsonUpdateAlert {
    /// The new status of the alert.
    pub status: Option<UpdateAlertStatus>,
}

#[typeshare::typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum UpdateAlertStatus {
    /// The alert is active.
    Active,
    /// The alert has been dismissed by a user.
    Dismissed,
}

#[typeshare::typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct JsonPerfAlert {
    pub uuid: AlertUuid,
    pub limit: BoundaryLimit,
    pub status: AlertStatus,
    pub modified: DateTime,
}
