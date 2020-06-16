/// Housekeeping data for the radiation counter
#[derive(GraphQLObject)]
pub struct RCHk {
    /// RC1 sum of the last 30 second period
    pub rc1_reading: i32,
    /// RC2 sum of the last 30 second period
    pub rc2_reading: i32,
    /// RC3 sum of the last 30 second period
    pub rc3_reading: i32,
    /// RC4 sum of the last 30 second period
    pub rc4_reading: i32,
    /// RC5 sum of the last 30 second period
    pub rc5_reading: i32,
}
