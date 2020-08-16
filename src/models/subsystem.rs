use crate::models::housekeeping::RCHk;
use crate::models::*;
use failure::Error;
use log::info;
use radiation_counter_api::{CounterResult, CuavaRadiationCounter, RadiationCounter};
use rust_i2c::*;
use std::sync::{Arc, Mutex, RwLock};

/// Enum for tracking the last mutation executed
#[derive(Copy, Clone, Debug, Eq, Hash, GraphQLEnum, PartialEq)]
pub enum Mutations {
    /// No mutation has been run since the service was started
    None,
    /// Manual reset
    ManualReset,
    /// Control power to Rasperry Pi
    RpiPower,
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
}

impl Subsystem {
    /// Create a new subsystem instance for the service to use
    pub fn new(radiation_counter: Box<dyn CuavaRadiationCounter + Send>) -> CounterResult<Self> {
        let radiation_counter = Arc::new(Mutex::new(radiation_counter));

        Ok(Self {
            radiation_counter,
            last_mutation: Arc::new(RwLock::new(Mutations::None)),
            errors: Arc::new(RwLock::new(vec![])),
        })
    }

    /// Create the underlying Radiation CounterResult object and then create a new subsystem which will use it
    pub fn from_path(bus: &str, addr: u16) -> CounterResult<Self> {
        let cuava_radiation_counter: Box<dyn CuavaRadiationCounter + Send> =
            Box::new(RadiationCounter::new(Connection::from_path(bus, addr)));
        Subsystem::new(cuava_radiation_counter)
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

    /// Ping system using I2C
    pub fn test_ping(&self) -> Result<GenericResponse, String> {
        let radiation_counter = self.radiation_counter.lock().unwrap();
        match run!(radiation_counter.test_ping(), self.errors) {
            Ok(_v) => Ok(GenericResponse {
                success: true,
                errors: "".to_string(),
            }),
            Err(e) => Ok(GenericResponse {
                success: false,
                errors: e,
            }),
        }
    }

    /// Control power mode of Rasperry Pi
    pub fn rpi_power(&self, state : bool) -> Result<MutationResponse, String> {
        let radiation_counter = self.radiation_counter.lock().unwrap();
        match run!(radiation_counter.rpi_power(state), self.errors) {
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

    /// Get housekeeping data
    pub fn get_housekeeping(&self) -> CounterResult<RCHk> {
        info!("RC housekeeping data requested");

        let mut radiation_counter = self.radiation_counter.lock().unwrap();

        //Count result ignored here, counter data saved in rc_readings.
        let _rc_count = radiation_counter.get_radiation_count();
        let result = run!(radiation_counter.get_housekeeping()).unwrap_or_default();
        let rchk = RCHk {
            rc1_reading: result.rc1_reading,
            rc2_reading: result.rc2_reading,
            rc3_reading: result.rc3_reading,
            rc4_reading: result.rc4_reading,
            rc5_reading: result.rc5_reading,
        };
        Ok(rchk)
    }
}
