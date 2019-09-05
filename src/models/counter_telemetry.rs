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

//! Data returned by `telemetry` query

use crate::schema::Context;
use radiation_counter_api::CounterTelemetry::Type as CounterTelemetryType;
use juniper::FieldResult;

/// Telemetry structure
pub struct Telemetry;

macro_rules! make_telemetry {
    (
        $($type: ident,)+
    ) => {
        /// Radiation Counter telemetry values
        #[derive(Clone, Debug, Hash, Eq, GraphQLEnum, PartialEq)]
        pub enum Type {
            $(
                /// $type
                $type,
            )+
        }

        impl Into<CounterTelemetryType> for Type {
            fn into(self) -> CounterTelemetryType {
                match self {
                    $(Type::$type => CounterTelemetryType::$type,)+
                }
            }
        }

        graphql_object!(Telemetry: Context as "Telemetry" |&self| {
            $(
                field $type(&executor) -> FieldResult<f64>
                {
                    Ok(f64::from(executor.context().subsystem().get_telemetry(Type::$type)?))
                }
            )+
        });
    }
}

make_telemetry!(
    Voltage,
    Current,
    Power,
);
