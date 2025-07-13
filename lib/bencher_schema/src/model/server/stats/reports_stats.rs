use bencher_json::system::server::{JsonCohort, JsonCohortAvg};
use diesel::{ExpressionMethods as _, QueryDsl as _, RunQueryDsl as _, dsl::count};
use dropshot::HttpError;
use tokio::sync::Mutex;

use crate::{connection_lock, context::DbConnection, error::resource_not_found_err, schema};

use super::{ProjectState, median};

pub(super) struct ReportsStats {
    pub active_projects: JsonCohort,
    pub reports: JsonCohort,
    pub reports_per_project: JsonCohortAvg,
}

impl ReportsStats {
    #[expect(clippy::cast_sign_loss, reason = "count is always positive")]
    pub async fn new(
        db_connection: &Mutex<DbConnection>,
        this_week: i64,
        this_month: i64,
        state: ProjectState,
    ) -> Result<Self, HttpError> {
        let mut weekly_reports =
            get_reports_by_project(db_connection, Some(this_week), &state).await?;
        let weekly_active_projects = weekly_reports.len();
        let weekly_reports_total: i64 = weekly_reports.iter().sum();
        let weekly_reports_per_project = median(&mut weekly_reports);

        let mut monthly_reports =
            get_reports_by_project(db_connection, Some(this_month), &state).await?;
        let monthly_active_projects = monthly_reports.len();
        let monthly_reports_total: i64 = monthly_reports.iter().sum();
        let monthly_reports_per_project = median(&mut monthly_reports);

        let mut total_reports = get_reports_by_project(db_connection, None, &state).await?;
        let total_active_projects = total_reports.len();
        let total_reports_total: i64 = total_reports.iter().sum();
        let total_reports_per_project = median(&mut total_reports);

        let active_projects = JsonCohort {
            week: weekly_active_projects as u64,
            month: monthly_active_projects as u64,
            total: total_active_projects as u64,
        };

        let reports = JsonCohort {
            week: weekly_reports_total as u64,
            month: monthly_reports_total as u64,
            total: total_reports_total as u64,
        };

        let reports_per_project = JsonCohortAvg {
            week: weekly_reports_per_project,
            month: monthly_reports_per_project,
            total: total_reports_per_project,
        };

        Ok(Self {
            active_projects,
            reports,
            reports_per_project,
        })
    }
}

async fn get_reports_by_project(
    db_connection: &Mutex<DbConnection>,
    since: Option<i64>,
    state: &ProjectState,
) -> Result<Vec<i64>, HttpError> {
    let mut query = schema::report::table
        .group_by(schema::report::project_id)
        .select(count(schema::report::id))
        .into_boxed();

    if let Some(since) = since {
        query = query.filter(schema::report::created.ge(since));
    }

    match state {
        ProjectState::All => query
            .load::<i64>(connection_lock!(db_connection))
            .map_err(resource_not_found_err!(Report)),
        ProjectState::Unclaimed | ProjectState::Claimed => {
            let mut query = schema::report::table
                .inner_join(schema::project::table.inner_join(
                    schema::organization::table.left_join(schema::organization_role::table),
                ))
                .group_by(schema::report::project_id)
                .select(count(schema::report::id))
                .into_boxed();

            query = match state {
                #[expect(
                    clippy::unreachable,
                    reason = "match above ensures this is unreachable"
                )]
                ProjectState::All => unreachable!(),
                ProjectState::Unclaimed => query.filter(schema::organization_role::id.is_null()),
                ProjectState::Claimed => query.filter(schema::organization_role::id.is_not_null()),
            };

            if let Some(since) = since {
                query = query.filter(schema::report::created.ge(since));
            }

            query
                .load::<i64>(connection_lock!(db_connection))
                .map_err(resource_not_found_err!(Report))
        },
    }
}
