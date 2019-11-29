/// Housekeeping data for the radiation counter
#[derive(GraphQLObject)]
pub struct RCHk {
    /// RC1 sum of the last 30 second period
    pub rc1_reading: i32,
    /// RC2 sum of the last 30 second period
    pub rc2_reading: i32,
    /// RC3 sum of the last 30 second period
	pub rc3_reading: i32,	
    // /// Timestamp of the start of the last 30 second period
    // pub timestamp: i32,
    // /// Average of the sum of all counter readings in the last 30 second period
    // pub sum_30s: i32,
    // /// Average of the sum of all counter readings in the 30 second period prior to the last 30 second period
    // pub pre_sum_30s: i32,
}