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

//! Service mutations

use crate::models::subsystem::Mutations;
use crate::models::{MutationResponse};
use crate::schema::Context;
use juniper::FieldResult;

/// Top-level mutation root structure
pub struct Root;

// Base GraphQL mutation model
graphql_object!(Root: Context as "Mutation" |&self| {
    //  Manually reset the Radiation Counter
    //
    //  mutation {
    //      manualReset {
    //          success: Boolean!
    //          errors: String!
    //      }
    //  }
    field manual_reset(&executor) -> FieldResult<MutationResponse>
        as "Perform manual reset of Radiation Counter"
    {
        executor.context().subsystem().set_last_mutation(Mutations::ManualReset);
        Ok(executor.context().subsystem().manual_reset()?)
    }
});
