/// Generic mutation response struct
#[derive(GraphQLObject)]
pub struct MutationResponse {
    /// Any errors which occurred during query
    pub errors: String,
    /// Success or fail status of query
    pub success: bool,
}

pub mod last_error;
pub mod reset_telemetry;
pub mod counter_telemetry;
pub mod subsystem;