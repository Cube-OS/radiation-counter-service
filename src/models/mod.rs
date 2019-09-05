/// Generic mutation response struct
#[derive(GraphQLObject)]
pub struct MutationResponse {
    /// Any errors which occurred during query
    pub errors: String,
    /// Success or fail status of query
    pub success: bool,
}

/// Generic mutation response struct
#[derive(GraphQLEnum)]
pub enum PowerState {
    /// System is on
    On,
    /// System is off
    Off,
}

pub mod last_error;
pub mod reset_telemetry;
pub mod counter_telemetry;
pub mod subsystem;