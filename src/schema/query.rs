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

//! Service queries

use crate::models::*;
use crate::schema::Context;
use juniper::FieldResult;

/// Telemetry query structure
pub struct Telemetry;

graphql_object!(Telemetry: Context as "telemetry" |&self| {
    // Fetch telemetry data for the radiation counter.
    // All returned values are automatically converted from their original raw data.
    //
    // powerTelemetry {
    //     voltage: f64,
    //     current: f64,
    //     power: f64,
    //     powerOnOff: bool,
    // }
    field power_telemetry() -> counter_telemetry::Telemetry
        as "Power Telemetry"
    {
        counter_telemetry::Telemetry {}
    }

    // Get the number of board resets, by category
    //
    // telemetry {
    //     reset {
    //         brownOut: i32,
    //         automaticSoftware: i32,
    //         manual: i32,
    //         watchdog: i32,
    //     }
    // }
    field reset() -> reset_telemetry::Telemetry
        as "Reset Telemetry"
    {
        reset_telemetry::Telemetry {}
    }

    // Fetch the current watchdog timeout period, in minutes
    //
    // telemetry {
    //         watchdogPeriod: u8,
    // }
    field watchdog_period(&executor) -> FieldResult<i32>
        as "Current watchdog period in minutes"
    {
        Ok(i32::from(executor.context().subsystem().get_comms_watchdog_period()?))
    }

    // Fetch the last error which was encountered by the system while executing a command
    //
    // telemetry {
    //         lastRadiationCounterError: last_error::Error
    // }
    field last_radiation_counter_error(&executor) -> FieldResult<last_error::Error>
        as "Last Radiation Counter error reported"
    {
        Ok(executor.context().subsystem().get_last_error()?)
    }
});

/// Top-level query root structure
pub struct Root;

// Base GraphQL query
graphql_object!(Root: Context as "Query" |&self| {

    // Test query to verify service is running without
    // attempting to communicate with hardware
    //
    // {
    //    ping: "pong"
    // }
    field ping() -> FieldResult<String>
        as "Test service query"
    {
        Ok(String::from("pong"))
    }

    // Get the last mutation run
    //
    // {
    //    ack: subsystem::Mutations
    // }
    field ack(&executor) -> FieldResult<subsystem::Mutations>
        as "Last run mutation"
    {
        let last_cmd = executor.context().subsystem().last_mutation.read()?;
        Ok(*last_cmd)
    }

    // Get all errors encountered since the last time
    // this field was queried
    //
    // {
    //    errors: [String]
    // }
    field errors(&executor) -> FieldResult<Vec<String>>
        as "Last errors encountered"
    {
        Ok(executor.context().subsystem().get_errors()?)
    }

    // Get the system power status
    //
    // {
    //     power: PowerState
    // }
//     field power(&executor) -> FieldResult<GetPowerResponse>
//         as "Last errors encountered"
//     {
//         Ok(executor.context().subsystem().get_power()?)
//     }

    // Get telemetry from the Radiation Counter
    field telemetry(&executor) -> FieldResult<Telemetry>
    {
        Ok(Telemetry)
    }
});
