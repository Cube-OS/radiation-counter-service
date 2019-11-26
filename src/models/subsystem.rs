use crate::models::*;
use crate::models::housekeeping::RCHk;
use radiation_counter_api::{CuavaRadiationCounter, RadiationCounter, CounterResult};
use failure::Error;
use rust_i2c::*;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
// use std::time::Duration;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use log::info;

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
    let mut last_30s = 0;
    
    loop {
        thread::sleep(Duration::from_secs(5));
        
        // Get the current time
        let now: Duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let timestamp = now.as_secs() as i32;
        
        // Get the radiation counter
        let mut radiation_counter = counter.lock().unwrap();
        
        // Every 30 seconds, start a new sum
        if timestamp - last_30s >= 30 {
            last_30s = timestamp;
            radiation_counter.swap_30s_block(timestamp);
        }
        
        let count_result = radiation_counter.get_radiation_count();
        match count_result {
            Ok((count1, count2, count3)) => {
                println!("Got counts ({}, {}, {}) at time {:?}", count1, count2, count3, timestamp);
            },
            Err(e) => info!("Error {}", e),
        }
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
}

impl Subsystem {
    /// Create a new subsystem instance for the service to use
    pub fn new(radiation_counter: Box<dyn CuavaRadiationCounter + Send>) -> CounterResult<Self> {
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
        })
    }
    
    /// Create the underlying Radiation CounterResult object and then create a new subsystem which will use it
    pub fn from_path(bus: &str, addr: u16) -> CounterResult<Self> {
        let cuava_radiation_counter: Box<dyn CuavaRadiationCounter + Send> =
            Box::new(RadiationCounter::new(Connection::from_path(bus, addr)));
        Subsystem::new(cuava_radiation_counter)
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
    pub fn get_radiation_count(&self) -> Result<(u8, u8, u8), String> {
        let mut radiation_counter = self.radiation_counter.lock().unwrap();
        Ok(run!(radiation_counter.get_radiation_count(), self.errors)?)
    }
    
    /// Get housekeeping data
    pub fn get_housekeeping(&self) -> CounterResult<RCHk> {
        info!("RC housekeeping data requested");
        
        let radiation_counter = self.radiation_counter.lock().unwrap();
        let result = run!(radiation_counter.get_housekeeping()).unwrap_or_default();
        
        let rchk = RCHk {
            rc1_reading: result.rc1_reading as i32,
            rc2_reading: result.rc2_reading as i32,
            rc3_reading: result.rc1_reading as i32,
            timestamp: result.timestamp as i32,
            avg_sum_30s: result.avg_sum_30s as i32,
            prev_avg_sum_30s: result.prev_avg_sum_30s as i32,
        };
        Ok(rchk)
    }
}
