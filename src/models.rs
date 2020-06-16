/// Generic mutation response struct
#[derive(GraphQLObject)]
pub struct MutationResponse {
    /// Any errors which occurred during query
    pub errors: String,
    /// Success or fail status of query
    pub success: bool,
}

/// Radiation counter subsystem
pub mod subsystem;
/// Housekeeping data for radiation counter
pub mod housekeeping;