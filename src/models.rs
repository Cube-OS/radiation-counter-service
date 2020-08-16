/// Generic mutation response struct
#[derive(GraphQLObject)]
pub struct MutationResponse {
    /// Any errors which occurred during mutation
    pub errors: String,
    /// Success or fail status of mutation
    pub success: bool,
}

/// Generic query response struct
#[derive(GraphQLObject)]
pub struct QueryResponse {
    /// Any errors encountered by the request
    pub errors: String,
    /// Request completion success or failure
    pub success: bool,
}

/// Radiation counter subsystem
pub mod subsystem;
/// Housekeeping data for radiation counter
pub mod housekeeping;