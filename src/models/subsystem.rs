use crate::models::*;
use clyde_3g_eps_api::{Checksum, Clyde3gEps, Eps};
use eps_api::EpsResult;
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

fn watchdog_thread(eps: Arc<Mutex<Box<Clyde3gEps + Send>>>) {
    loop {
        thread::sleep(Duration::from_secs(60));
        let _res_ = eps.lock().unwrap().reset_comms_watchdog();
    }
}

/// Main structure for controlling and accessing system resources
#[derive(Clone)]
pub struct Subsystem {
    /// Underlying EPS object
    pub radiation_counter: Arc<Mutex<Box<Clyde3gEps + Send>>>,
    /// Last mutation executed
    pub last_mutation: Arc<RwLock<Mutations>>,
    /// Errors accumulated over all queries and mutations
    pub errors: Arc<RwLock<Vec<String>>>,
    /// Watchdog kicking thread handle
    pub watchdog_handle: Arc<Mutex<thread::JoinHandle<()>>>,
}

impl Subsystem {
    /// Create a new subsystem instance for the service to use
    pub fn new(eps: Box<SugarRadCounter + Send>) -> CounterResult<Self> {
        let eps = Arc::new(Mutex::new(eps));
        let thread_eps = eps.clone();
        let watchdog = thread::spawn(move || watchdog_thread(thread_eps));

        Ok(Self {
            eps,
            last_mutation: Arc::new(RwLock::new(Mutations::None)),
            errors: Arc::new(RwLock::new(vec![])),
            watchdog_handle: Arc::new(Mutex::new(watchdog)),
            checksum: Arc::new(Mutex::new(Checksum::default())),
        })
    }
}
