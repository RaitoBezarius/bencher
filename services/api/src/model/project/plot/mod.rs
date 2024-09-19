use bencher_json::{
    project::plot::{JsonPlotPatch, JsonPlotPatchNull, JsonUpdatePlot, XAxis},
    DateTime, Index, JsonNewPlot, JsonPlot, PlotUuid, ResourceName, Window,
};
use bencher_rank::{Rank, RankGenerator, Ranked};
use diesel::{BelongingToDsl, ExpressionMethods, QueryDsl, RunQueryDsl};
use dropshot::HttpError;

use super::{ProjectId, QueryProject};
use crate::{
    conn_lock,
    context::{ApiContext, DbConnection},
    error::{
        assert_parentage, resource_conflict_err, resource_conflict_error, resource_not_found_err,
        BencherResource,
    },
    schema::plot as plot_table,
};

mod benchmark;
mod branch;
mod measure;
mod testbed;

use benchmark::{InsertPlotBenchmark, QueryPlotBenchmark};
use branch::{InsertPlotBranch, QueryPlotBranch};
use measure::{InsertPlotMeasure, QueryPlotMeasure};
use testbed::{InsertPlotTestbed, QueryPlotTestbed};

crate::util::typed_id::typed_id!(PlotId);

#[derive(
    Debug, Clone, diesel::Queryable, diesel::Identifiable, diesel::Associations, diesel::Selectable,
)]
#[diesel(table_name = plot_table)]
#[diesel(belongs_to(QueryProject, foreign_key = project_id))]
#[allow(clippy::struct_excessive_bools)]
pub struct QueryPlot {
    pub id: PlotId,
    pub uuid: PlotUuid,
    pub project_id: ProjectId,
    pub rank: Rank,
    pub title: Option<ResourceName>,
    pub lower_value: bool,
    pub upper_value: bool,
    pub lower_boundary: bool,
    pub upper_boundary: bool,
    pub x_axis: XAxis,
    pub window: Window,
    pub created: DateTime,
    pub modified: DateTime,
}

impl QueryPlot {
    pub fn get_with_uuid(
        conn: &mut DbConnection,
        query_project: &QueryProject,
        uuid: PlotUuid,
    ) -> Result<Self, HttpError> {
        Self::belonging_to(&query_project)
            .filter(plot_table::uuid.eq(uuid))
            .first::<Self>(conn)
            .map_err(resource_not_found_err!(Plot, (query_project, uuid)))
    }

    fn all_for_project(
        conn: &mut DbConnection,
        query_project: &QueryProject,
    ) -> Result<Vec<Self>, HttpError> {
        Self::belonging_to(query_project)
            .order(plot_table::rank.asc())
            .load::<Self>(conn)
            .map_err(resource_not_found_err!(Plot, &query_project))
    }

    fn all_others_for_project(
        &self,
        conn: &mut DbConnection,
        query_project: &QueryProject,
    ) -> Result<Vec<Self>, HttpError> {
        Self::belonging_to(query_project)
            .filter(plot_table::id.ne(self.id))
            .order(plot_table::rank.asc())
            .load::<Self>(conn)
            .map_err(resource_not_found_err!(Plot, &query_project))
    }

    fn new_rank(
        conn: &mut DbConnection,
        query_project: &QueryProject,
        index: Option<Index>,
    ) -> Result<Rank, HttpError> {
        let index = u8::from(index.unwrap_or_default()).into();

        // Get the current plots.
        let plots = QueryPlot::all_for_project(conn, query_project)?;

        // Try to calculate the rank within the current plots.
        if let Some(rank) = Rank::calculate(&plots, index) {
            return Ok(rank);
        }

        // If the rank cannot be calculated, then we need to redistribute the ranks.
        let plot_ranker = RankGenerator::new(plots.len());
        for (plot, rank) in plots.iter().zip(plot_ranker) {
            UpdatePlot::update_rank(conn, plot, rank)?;
        }

        // Try to calculate the rank within the redistributed plots.
        let redistributed_plots = QueryPlot::all_for_project(conn, query_project)?;
        Rank::calculate(&redistributed_plots, index).ok_or_else(|| {
            resource_conflict_error(
                BencherResource::Plot,
                (redistributed_plots, index),
                "Failed to redistribute plots.",
            )
        })
    }

    fn update_rank(
        &self,
        conn: &mut DbConnection,
        query_project: &QueryProject,
        index: Index,
    ) -> Result<Rank, HttpError> {
        let index = u8::from(index).into();

        // Get the current plots, except for self.
        let other_plots = self.all_others_for_project(conn, query_project)?;

        // Try to calculate the rank within the current plots.
        if let Some(rank) = Rank::calculate(&other_plots, index) {
            return Ok(rank);
        }

        // If the rank cannot be calculated, then we need to redistribute all the ranks.
        let all_plots = QueryPlot::all_for_project(conn, query_project)?;
        let plot_ranker = RankGenerator::new(all_plots.len());
        for (plot, rank) in all_plots.iter().zip(plot_ranker) {
            UpdatePlot::update_rank(conn, plot, rank)?;
        }

        // Try to calculate the rank within the redistributed plots.
        let redistributed_plots = self.all_others_for_project(conn, query_project)?;
        Rank::calculate(&redistributed_plots, index).ok_or_else(|| {
            resource_conflict_error(
                BencherResource::Plot,
                (redistributed_plots, index),
                "Failed to redistribute plots.",
            )
        })
    }

    pub fn into_json_for_project(
        self,
        conn: &mut DbConnection,
        project: &QueryProject,
    ) -> Result<JsonPlot, HttpError> {
        assert_parentage(
            BencherResource::Project,
            project.id,
            BencherResource::Plot,
            self.project_id,
        );
        let branches = QueryPlotBranch::into_json_for_plot(conn, &self)?;
        let testbeds = QueryPlotTestbed::into_json_for_plot(conn, &self)?;
        let benchmarks = QueryPlotBenchmark::into_json_for_plot(conn, &self)?;
        let measures = QueryPlotMeasure::into_json_for_plot(conn, &self)?;
        let Self {
            uuid,
            title,
            lower_value,
            upper_value,
            lower_boundary,
            upper_boundary,
            x_axis,
            window,
            created,
            modified,
            ..
        } = self;
        Ok(JsonPlot {
            uuid,
            project: project.uuid,
            title,
            lower_value,
            upper_value,
            lower_boundary,
            upper_boundary,
            x_axis,
            window,
            branches,
            testbeds,
            benchmarks,
            measures,
            created,
            modified,
        })
    }
}

impl Ranked for QueryPlot {
    fn rank(&self) -> Rank {
        self.rank
    }
}

#[derive(Debug, diesel::Insertable)]
#[diesel(table_name = plot_table)]
#[allow(clippy::struct_excessive_bools)]
pub struct InsertPlot {
    pub uuid: PlotUuid,
    pub project_id: ProjectId,
    pub rank: Rank,
    pub title: Option<ResourceName>,
    pub lower_value: bool,
    pub upper_value: bool,
    pub lower_boundary: bool,
    pub upper_boundary: bool,
    pub x_axis: XAxis,
    pub window: Window,
    pub created: DateTime,
    pub modified: DateTime,
}

impl InsertPlot {
    pub async fn from_json(
        context: &ApiContext,
        query_project: &QueryProject,
        plot: JsonNewPlot,
    ) -> Result<QueryPlot, HttpError> {
        let JsonNewPlot {
            index,
            title,
            lower_value,
            upper_value,
            lower_boundary,
            upper_boundary,
            x_axis,
            window,
            branches,
            testbeds,
            benchmarks,
            measures,
        } = plot;
        let rank = QueryPlot::new_rank(conn_lock!(context), query_project, index)?;
        let timestamp = DateTime::now();
        let insert_plot = Self {
            uuid: PlotUuid::new(),
            project_id: query_project.id,
            rank,
            title,
            lower_value,
            upper_value,
            lower_boundary,
            upper_boundary,
            x_axis,
            window,
            created: timestamp,
            modified: timestamp,
        };
        diesel::insert_into(plot_table::table)
            .values(&insert_plot)
            .execute(conn_lock!(context))
            .map_err(resource_conflict_err!(Plot, insert_plot))?;

        let query_plot = plot_table::table
            .filter(plot_table::uuid.eq(&insert_plot.uuid))
            .first::<QueryPlot>(conn_lock!(context))
            .map_err(resource_not_found_err!(Plot, insert_plot))?;

        InsertPlotBranch::from_json(context, query_plot.id, branches).await?;
        InsertPlotTestbed::from_json(context, query_plot.id, testbeds).await?;
        InsertPlotBenchmark::from_json(context, query_plot.id, benchmarks).await?;
        InsertPlotMeasure::from_json(context, query_plot.id, measures).await?;

        Ok(query_plot)
    }
}

#[derive(Debug, Clone, diesel::AsChangeset)]
#[diesel(table_name = plot_table)]
pub struct UpdatePlot {
    pub rank: Option<Rank>,
    pub title: Option<Option<ResourceName>>,
    pub window: Option<Window>,
    pub modified: DateTime,
}

impl UpdatePlot {
    pub async fn from_json(
        context: &ApiContext,
        query_project: &QueryProject,
        query_plot: &QueryPlot,
        update: JsonUpdatePlot,
    ) -> Result<Self, HttpError> {
        let (index, title, window) = match update {
            JsonUpdatePlot::Patch(patch) => {
                let JsonPlotPatch {
                    index,
                    title,
                    window,
                } = patch;
                (index, title.map(Some), window)
            },
            JsonUpdatePlot::Null(patch_url) => {
                let JsonPlotPatchNull {
                    index,
                    title: (),
                    window,
                } = patch_url;
                (index, Some(None), window)
            },
        };
        let rank = if let Some(index) = index {
            Some(query_plot.update_rank(conn_lock!(context), query_project, index)?)
        } else {
            None
        };
        Ok(Self {
            rank,
            title,
            window,
            modified: DateTime::now(),
        })
    }

    fn update_rank(
        conn: &mut DbConnection,
        query_plot: &QueryPlot,
        rank: Rank,
    ) -> Result<(), HttpError> {
        let update_plot = UpdatePlot {
            rank: Some(rank),
            title: None,
            window: None,
            modified: DateTime::now(),
        };

        diesel::update(plot_table::table.filter(plot_table::id.eq(query_plot.id)))
            .set(&update_plot)
            .execute(conn)
            .map_err(resource_conflict_err!(Plot, (query_plot, &update_plot)))?;

        Ok(())
    }
}
