use crate::models::*;
use crate::models::housekeeping::RCHk;
use radiation_counter_api::{CuavaRadiationCounter, RadiationCounter, CounterResult};
use failure::Error;
use rust_i2c::*;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

/// Enum for tracking the last mutation executed
#[derive(Copy, Clone, Debug, Eq, Hash, GraphQLEnum, PartialEq)]
pub enum Mutations {
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

fn watchdog_thread(counter: Arc<Mutex<Box<dyn CuavaRadiationCounter + Send>>>) {
    loop {
        thread::sleep(Duration::from_secs(60));
        let _res_ = counter.lock().unwrap().reset_comms_watchdog();
    }
}

fn counter_thread(counter: Arc<Mutex<Box<dyn CuavaRadiationCounter + Send>>>) {
//     let mut counts = Vec::new();
    
    loop {
        let count_result = counter.lock().unwrap().get_radiation_count();
        match count_result {
            Ok((timestamp, count)) => {
                println!("Got count {} at time {:?}", count, timestamp);
//                 counts.push((timestamp, count));
//                 println!("{:?}", counts);
            },
            Err(e) => println!("Error {}", e),
        }
        thread::sleep(Duration::from_secs(5));
    }
}

/// Main structure for controlling and accessing system resources
#[derive(Clone)]
pub struct Subsystem {
    /// Underlying Radiation Counter object
    pub radiation_counter: Arc<Mutex<Box<dyn CuavaRadiationCounter + Send>>>,
    /// Last mutation executed
    pub last_mutation: Arc<RwLock<Mutations>>,
    /// Errors accumulated over all queries and mutations
    pub errors: Arc<RwLock<Vec<String>>>,
    /// Watchdog kicking thread handle
    pub watchdog_handle: Arc<Mutex<thread::JoinHandle<()>>>,
    /// Count retriever thread handle
    pub counter_handle: Arc<Mutex<thread::JoinHandle<()>>>,
    /// Channel number for EPS connection
    pub power_channel: u8,
}

impl Subsystem {
    /// Create a new subsystem instance for the service to use
    pub fn new(radiation_counter: Box<dyn CuavaRadiationCounter + Send>, power_channel: u8) -> CounterResult<Self> {
        let radiation_counter = Arc::new(Mutex::new(radiation_counter));
        let watchdog_thread_counter = radiation_counter.clone();
        let watchdog = thread::spawn(move || watchdog_thread(watchdog_thread_counter));
        
        let counter_thread_counter = radiation_counter.clone();
        let counter = thread::spawn(move || counter_thread(counter_thread_counter));

        Ok(Self {
            radiation_counter,
            last_mutation: Arc::new(RwLock::new(Mutations::None)),
            errors: Arc::new(RwLock::new(vec![])),
            watchdog_handle: Arc::new(Mutex::new(watchdog)),
            counter_handle: Arc::new(Mutex::new(counter)),
            power_channel: power_channel,
        })
    }
    
    /// Create the underlying Radiation CounterResult object and then create a new subsystem which will use it
    pub fn from_path(bus: &str, addr: u16, power_channel: u8) -> CounterResult<Self> {
        let cuava_radiation_counter: Box<dyn CuavaRadiationCounter + Send> =
            Box::new(RadiationCounter::new(Connection::from_path(bus, addr)));
        Subsystem::new(cuava_radiation_counter, power_channel)
    }

    /// Get the requested telemetry item
//     pub fn get_telemetry(
//         &self,
//         telem_type: counter_telemetry::Type,
//     ) -> Result<f64, String> {
//         let result = run!(
//             self.radiation_counter
//                 .lock()
//                 .unwrap()
//                 .get_telemetry(telem_type.into()),
//             self.errors
//         )?;
// 
//         Ok(result)
//     }

    /// Get the voltage being used by the module
    pub fn get_voltage(&self) -> Result<f64, String> {
        // TODO: Implement
        Ok(10.0)
    }

    /// Get the current being used by the module
    pub fn get_current(&self) -> Result<f64, String> {
        // TODO: Implement
        Ok(1.5)
    }

    /// Get the power being used by the module
    pub fn get_power(&self) -> Result<f64, String> {
        // TODO: Implement
        Ok(15.0)
    }
    
    /// Get the current on/off status of the radiation counter
    pub fn get_power_on_off(&self) -> Result<bool, String> {
        let radiation_counter = self.radiation_counter.lock().unwrap();
        Ok(run!(radiation_counter.get_power_status(), self.errors)?.into())
    }
    
    /// Set the radiation counter power status
    pub fn set_power(&self, status: bool) -> Result<MutationResponse, String> {
        let mut radiation_counter = self.radiation_counter.lock().unwrap();
        match run!(radiation_counter.set_power_status(status), self.errors) {
            Ok(_v) => Ok(MutationResponse {
                success: true,
                errors: "".to_string(),
            }),
            Err(e) => Ok(MutationResponse {
                success: false,
                errors: e,
            }),
        }
    }

    /// Get the specific type of reset counts
    pub fn get_reset_telemetry(
        &self,
        telem_type: reset_telemetry::Type,
    ) -> Result<u8, String> {
        let radiation_counter = self.radiation_counter.lock().unwrap();
        Ok(run!(radiation_counter.get_reset_telemetry(telem_type.into()), self.errors)?.into())
    }

    /// Get the current watchdog period setting
    pub fn get_comms_watchdog_period(&self) -> Result<u8, String> {
        let radiation_counter = self.radiation_counter.lock().unwrap();
        Ok(run!(radiation_counter.get_comms_watchdog_period(), self.errors)?)
    }

    /// Get the last error the Radiation Counter encountered
    pub fn get_last_error(&self) -> Result<last_error::Error, String> {
        let radiation_counter = self.radiation_counter.lock().unwrap();
        Ok(run!(radiation_counter.get_last_error(), self.errors)?.into())
    }

    /// Trigger a manual reset of the Radiation Counter
    pub fn manual_reset(&self) -> Result<MutationResponse, String> {
        let radiation_counter = self.radiation_counter.lock().unwrap();
        match run!(radiation_counter.manual_reset(), self.errors) {
            Ok(_v) => Ok(MutationResponse {
                success: true,
                errors: "".to_string(),
            }),
            Err(e) => Ok(MutationResponse {
                success: false,
                errors: e,
            }),
        }
    }

    /// Kick the I2C watchdog
    pub fn reset_watchdog(&self) -> Result<MutationResponse, String> {
        let radiation_counter = self.radiation_counter.lock().unwrap();
        match run!(radiation_counter.reset_comms_watchdog(), self.errors) {
            Ok(_v) => Ok(MutationResponse {
                success: true,
                errors: "".to_string(),
            }),
            Err(e) => Ok(MutationResponse {
                success: false,
                errors: e,
            }),
        }
    }

    /// Set the I2C watchdog timeout period
    pub fn set_watchdog_period(&self, period: u8) -> Result<MutationResponse, String> {
        let radiation_counter = self.radiation_counter.lock().unwrap();
        match run!(radiation_counter.set_comms_watchdog_period(period), self.errors) {
            Ok(_v) => Ok(MutationResponse {
                success: true,
                errors: "".to_string(),
            }),
            Err(e) => Ok(MutationResponse {
                success: false,
                errors: e,
            }),
        }
    }

    /// Pass raw command values through to the EPS
    pub fn raw_command(&self, command: u8, data: Vec<u8>) -> Result<MutationResponse, String> {
        let radiation_counter = self.radiation_counter.lock().unwrap();
        match run!(radiation_counter.raw_command(command, data), self.errors) {
            Ok(_v) => Ok(MutationResponse {
                success: true,
                errors: "".to_string(),
            }),
            Err(e) => Ok(MutationResponse {
                success: false,
                errors: e,
            }),
        }
    }

    /// Record the last mutation executed by the service
    pub fn set_last_mutation(&self, mutation: Mutations) {
        if let Ok(mut last_cmd) = self.last_mutation.write() {
            *last_cmd = mutation;
        }
    }

    /// Fetch all errors since the last time this function was called, then clear the errors storage
    pub fn get_errors(&self) -> CounterResult<Vec<String>> {
        match self.errors.write() {
            Ok(mut master_vec) => {
                let current = master_vec.clone();
                master_vec.clear();
                master_vec.shrink_to_fit();
                Ok(current)
            }
            _ => Ok(vec![
                "Error: Failed to borrow master errors vector".to_string()
            ]),
        }
    }
    
    /// Get radiation count over i2c
    pub fn get_radiation_count(&self) -> Result<(Duration, u8), String> {
        let mut radiation_counter = self.radiation_counter.lock().unwrap();
        Ok(run!(radiation_counter.get_radiation_count(), self.errors)?)
    }
    
    /// Get housekeeping data
    pub fn get_housekeeping(&self) -> CounterResult<RCHk> {
        println!("Get radiation counter housekeeping data");
        
        let mut radiation_counter = self.radiation_counter.lock().unwrap();
        let result = run!(radiation_counter.get_housekeeping()).unwrap_or_default();
        
        let rchk = RCHk {
            voltage: result.voltage as i32,
            current: result.current as i32,
            timestamps: result.timestamps as Vec<i32>,
            readings: result.readings as Vec<i32>,
        };
        Ok(rchk)
    }
}
