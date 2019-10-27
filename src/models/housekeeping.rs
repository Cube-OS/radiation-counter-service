/// Housekeeping data for the radiation counter
#[derive(GraphQLObject)]
pub struct RCHk {
    /// Timestamp of the start of the last 30 second period
    pub timestamp: i32,
    /// Average of the sum of all counter readings in the last 30 second period
    pub avg_sum_30s: i32,
    /// Average of the sum of all counter readings in the 30 second period prior to the last 30 second period
    pub prev_avg_sum_30s: i32,
}