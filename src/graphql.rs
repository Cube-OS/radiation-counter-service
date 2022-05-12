//
// Copyright (C) 2019 Kubos Corporation
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

// Implementation of GraphQL structs to use in the Cube-OS framework. 
// Only used in the "Ground" and "GraphQL" feature
// 
// GraphQL only allows four different types:
// i32,f64,String and bool
// 
// Therefore it is often necessary and desired (to reduce a data overhead over UDP)
// to translate from those types to types in the corresponding API or input struct 

use juniper::*;
use cubeos_service::*;


/// Housekeeping data for the radiation counter
#[derive(GraphQLObject)]
pub struct GplRCHk {
    /// RC1 sum of the last 30 second period
    pub rc1_reading: i32,
    /// RC2 sum of the last 30 second period
    pub rc2_reading: i32,
    /// RC3 sum of the last 30 second period
	pub rc3_reading: i32,	
}

/// Generic mutation response struct
#[derive(GraphQLObject)]
pub struct GenericResponse {
    /// Any errors which occurred during query
    pub errors: String,
    /// Success or fail status of query
    pub success: bool,
}

use radiation_counter_api::{ErrorCode};

/// Error variants which can be returned by the Radiation Counter
#[derive(GraphQLEnum)]
pub enum GqlError {
    /// No error was encountered
    None = 0x00,
    /// Unknown command received
    UnknownCommand = 0x01,
    /// A reset had to occur
    ResetOccurred = 0x02,
    /// The command to fetch the last error failed
    CommandError = 0x03,
    /// Catch all for future error values
    UnknownError,
}

fn to_error(error_code: ErrorCode) -> GqlError {
    match error_code {
        ErrorCode::None => GqlError::None,
        ErrorCode::CommandError => GqlError::CommandError,
        ErrorCode::ResetOccurred => GqlError::ResetOccurred,
        ErrorCode::UnknownCommand => GqlError::UnknownCommand,
        ErrorCode::UnknownError => GqlError::UnknownError,
    }
}

impl Into<GqlError> for ErrorCode {
    fn into(self) -> GqlError {
        to_error(self)
    }
}

/// Enum for tracking the last mutation executed
#[derive(GraphQLEnum)]
pub enum GqlMutations {
    /// No mutation has been run since the service was started
    None,
    /// No-op
    Noop,
    /// Manual reset
    ManualReset,
    /// Raw passthrough command
    RawCommand,
    /// Watchdog reset
    ResetWatchdog,
    /// Set watchdog period
    SetWatchdogPeriod,
    /// Hardware test
    TestHardware,
}
