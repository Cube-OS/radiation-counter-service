// The service.rs file is the core of each service
// It enables the communication via UDP or GraphQL (depending on --features flag during compilation)

use cubeos_service::*;
use radiation_counter_api::*;

service_macro!{
        use Error;
        subsystem::Subsystem {
                query: Ping => fn ping(&self) -> Result<()>;
                // query: GetLastMutation => fn get_last_mutation(&self) -> Result<Mutations>; in:; out: GqlMutations;
                query: GetLastError => fn get_last_error(&self) -> Result<ErrorCode>; 
                        out: ErrorCode;     
                query: GetCounts => fn get_radiation_count(&self) -> Result<RCHk>; 
                        out: RCHk;
                query: GetErrors => fn get_errors(&self) -> Result<Vec<String>>; 
                        out: Vec<String>;
                mutation: ManualReset => fn manual_reset(&self) -> Result<()>; 
                mutation: ResetWatchdog => fn reset_watchdog(&self) -> Result<()>;
        }         
}