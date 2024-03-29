//
// Copyright (C) 2019 The University of Sydney
//
//
//! Kubos Service for interacting with the radiation counter payload
//!
//! # Configuration
//!
//! The service must be configured in `/home/system/etc/config.toml` with the following fields:
//!
//! - `[radiation-counter-service.addr]`
//!
//!     - `ip` - Specifies the service's IP address
//!     - `port` - Specifies the port on which the service will be listening for UDP packets
//!
//! - `[radiation-counter-service.device]`
//!
//!     - `bus` - Specifies the I2C bus
//! 	- `addr` - Specifies the I2C address
//!
//! For example:
//!
//! ```toml
//! [radiation-counter-service.addr]
//! ip = "0.0.0.0"
//! port = 8101
//!
//! [radiation-counter-service.device]
//! bus = "/dev/i2c-0"
//! addr = 0x31
//! ```

// TODO: Commands table

// #![deny(missing_docs, warnings)]

// extern crate juniper;
// extern crate cubeos_service;

/// Service models
pub mod subsystem;
/// Creating service functions for the radiation coutnter
pub mod service;

///include API
use radiation_counter_api::*;

use cubeos_service::{Config,Logger,Service};
use crate::service::*;
use crate::subsystem::Subsystem;  
use std::sync::{Arc};
use log::{error,info};

fn main() -> CounterResult<()>{
    
    let _ = Logger::init();
    
    // Get the radiation-counter-service component from the config file
    let rc_config = Config::new("radiation-counter-service").expect("Failed to load RC config");
    
    // Radiation counter bus and addr
    #[cfg(not(any(feature = "ground",feature = "terminal")))]
    let bus = rc_config
        .get("i2c_bus")
        .ok_or_else(|| {
            error!("Failed to load 'bus' config value");
        })
        .unwrap();
    #[cfg(not(any(feature = "ground",feature = "terminal")))]
    let bus = bus.as_str().unwrap();

    // Alternatively the I2C address can be hardcoded here
    #[cfg(not(any(feature = "ground",feature = "terminal")))]
    let addr = rc_config
        .get("i2c_addr")
        .ok_or_else(|| {
            error!("Failed to load 'bus' config value");
        })
        .unwrap();
    #[cfg(not(any(feature = "ground",feature = "terminal")))]
    let addr = addr.as_str().unwrap();
    #[cfg(not(any(feature = "ground",feature = "terminal")))]
    let addr: u16 = if addr.starts_with("0x") {
        u16::from_str_radix(&addr[2..], 16).unwrap()
    } else {
        u16::from_str_radix(addr, 16).unwrap()
    };    

    // Only needed for the ground feature
    #[cfg(any(feature = "terminal",feature = "ground"))]
    let socket = rc_config
    .get("udp_socket")
    .ok_or_else(|| {
        error!("Failed to load 'udp-socket' config value");
    })
    .unwrap();

    #[cfg(any(feature = "terminal",feature = "ground"))]
    let target = rc_config
    .get("target")
    .ok_or_else(|| {
        error!("Failed to load 'target' config value");
    })
    .unwrap();
    
    // Create the radiation counter subsystem
    #[cfg(not(any(feature = "ground",feature = "terminal")))]
    let subsystem: Box<Subsystem> = Box::new(
        Subsystem::from_path(bus, addr)
            .map_err(|err| {
                error!("Failed to create subsystem: {:?}", err);
                err
            })
            .unwrap(),
    );
    
    #[cfg(feature = "ground")]
    // Start debug service
    Service::new(
        rc_config,
        socket.as_str().unwrap().to_string(),
        target.as_str().unwrap().to_string(),
        Some(Arc::new(json_handler)),
    ).start();

    #[cfg(feature = "terminal")]
    // Start terminal service
    Service::new(
        rc_config,
        socket.as_str().unwrap().to_string(),
        target.as_str().unwrap().to_string(),
        Some(Arc::new(terminal)),
    ).start();

    #[cfg(not(any(feature = "ground",feature = "terminal")))]
    //Start up UDP server
    Service::new(
        rc_config,
        subsystem,
        Some(Arc::new(udp_handler)),
    )
    .start();

    #[cfg(debug)]
    println!("{:?}", rc_config);

    #[cfg(debug)]
    debug();
    
    Ok(())
}
