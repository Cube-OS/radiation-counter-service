// use crate::models::housekeeping::RCHk;
use radiation_counter_api::*;
// use radiation_counter_api::commands::last_error;

// use failure::Error;
use rust_i2c::*;
use std::sync::{Arc, Mutex, RwLock};
use cubeos_service::*;
use cubeos_error::*;
use std::convert::From;
use serde::*;

/// Enum for tracking the last mutation executed
#[derive(Clone, Debug, Eq, PartialEq, Serialize,Deserialize)]
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

/// Main structure for controlling and accessing system resources
#[derive(Clone)]
pub struct Subsystem {
    /// Underlying Radiation Counter object
    pub radiation_counter: Arc<Mutex<Box<dyn CuavaRadiationCounter + Send>>>,
    /// Last mutation executed
    pub last_mutation: Arc<RwLock<Mutations>>,
    /// Errors accumulated over all queries and mutations
    pub errors: Arc<RwLock<Vec<String>>>,
   // /// Watchdog kicking thread handle
    //    pub watchdog_handle: Arc<Mutex<thread::JoinHandle<()>>>,
    // / Count retriever thread handle
   // pub counter_handle: Arc<Mutex<thread::JoinHandle<()>>>,
}

impl Subsystem {
    /// Create a new subsystem instance for the service to use
    pub fn new(radiation_counter: Box<dyn CuavaRadiationCounter + Send>) -> CounterResult<Self> {
        let radiation_counter = Arc::new(Mutex::new(radiation_counter));
        // let watchdog_thread_counter = radiation_counter.clone();
        // let watchdog = thread::spawn(move || watchdog_thread(watchdog_thread_counter));
        
        // let counter_thread_counter = radiation_counter.clone();
        // let counter = thread::spawn(move || counter_thread(counter_thread_counter));

        Ok(Self {
            radiation_counter,
            last_mutation: Arc::new(RwLock::new(Mutations::None)),
            errors: Arc::new(RwLock::new(vec![])),
         //   watchdog_handle: Arc::new(Mutex::new(watchdog)),
           // counter_handle: Arc::new(Mutex::new(counter)),
        })
    }
    
    /// Create the underlying Radiation CounterResult object and then create a new subsystem which will use it
    pub fn from_path(bus: &str, addr: u16) -> CounterResult<Self> {
        let cuava_radiation_counter: Box<dyn CuavaRadiationCounter + Send> =
            Box::new(RadiationCounter::new(Connection::from_path(bus, addr)));
        Subsystem::new(cuava_radiation_counter)
    }

    /// Record the last mutation executed by the service
    pub fn set_last_mutation(&self, mutation: Mutations) {
        if let Ok(mut last_cmd) = self.last_mutation.write() {
            *last_cmd = mutation;
        }
    }

    // Ping
    pub fn ping(&self) -> Result<GenericResponse> {
        Ok(GenericResponse::new())
    }
    
    // Get the last excuated mutation
    pub fn get_last_mutation(&self) -> Result<Mutations> {
        println!("get_last_mutation");
        Ok(self.radiation_counter.lock().unwrap().last_mutation.read()?)
    }


    /// Get the last error the Radiation Counter encountered
    pub fn get_last_error(&self) -> Result<ErrorCode> {
        // let radiation_counter = self.radiation_counter.lock().unwrap();
        // Ok(run!(radiation_counter.get_last_error(), self.errors)?.into())
        println!("get_last_error");
        match self.radiation_counter.lock().unwrap().get_last_error() {
            Ok(x) => Ok(x),
            Err(e) => Err(Error::from(e)),
        }
    }

    /// Trigger a manual reset of the Radiation Counter
    pub fn manual_reset(&self) -> Result<()> {
        println!("manual_reset");
        // match self.radiation_counter.lock().unwrap().manual_reset() {
        //     Ok(()) => Ok(()),
        //     Err(e) => Err(Error::from(e)),
        // }
        Ok(self.radiation_counter.lock().unwrap().manual_reset()?)
    }

    /// Get radiation count over i2c
    pub fn get_radiation_count(&self) -> Result<(u16, u16, u16)> {
        println!("get_radiation_count");
        match self.radiation_counter.lock().unwrap().get_radiation_count() {
            Ok(x) => Ok(x),
            Err(e) => Err(Error::from(e)),
        }
    }

    /// Fetch all errors since the last time this function was called, then clear the errors storage
    pub fn get_errors(&self) -> Result<Vec<String>> {
        println!("get_last_error");
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

    /// Kick the I2C watchdog
    pub fn reset_watchdog(&self) -> Result<()> {
        println!("reset_watchdog");
        // match self.radiation_counter.lock().unwrap().reset_comms_watchdog() {
        //     Ok(()) => Ok(()),
        //     Err(e) => Err(Error::from(e)),
        // }
        Ok(self.radiation_counter.lock().unwrap().reset_comms_watchdog()?)
    }
}
