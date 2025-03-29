use super::post_resignation::PostResignationMutation;
use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub struct MutationRoot(PostResignationMutation);
