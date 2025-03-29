use async_graphql::MergedObject;

use super::{
    latest_resignation::LatestResignationQuery, vacation_start_date::VacationStartDateQuery,
};

#[derive(MergedObject, Default)]
pub struct QueryRoot(LatestResignationQuery, VacationStartDateQuery);
