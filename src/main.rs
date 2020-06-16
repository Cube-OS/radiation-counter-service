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
//! bus = "/dev/i2c-1"
//! addr = 0x32
//! ```

#![deny(missing_docs, warnings)]

#[macro_use]
extern crate juniper;
#[macro_use]
extern crate kubos_service;

/// Service models
pub mod models;
/// GraphQL schema for the radiation counter
pub mod schema;

use kubos_service::{Config, Service};
use log::info;
use models::subsystem::Subsystem;
use schema::mutation::Root as MutationRoot;
use schema::query::Root as QueryRoot;
use syslog::Facility;

fn main() {
    syslog::init(
        Facility::LOG_DAEMON,
        log::LevelFilter::Debug,
        Some("radiation-counter-service"),
    )
    .unwrap();
    // Get the radiation-counter-service component from the config file
    let rc_config = Config::new("radiation-counter-service").expect("Failed to load RC config");
    // Radiation counter bus and addr
    // [radiation-counter-service.device]
    let device = rc_config.get("device").unwrap();
    let bus = device["bus"]
        .as_str()
        .expect("Failed to get RC I2C bus value");
    let addr = device["addr"]
        .as_integer()
        .expect("Failed to get RC I2C address value") as u16;
    info!("I2C Bus:     {}", bus);
    info!("I2C Address: {}", addr);
    // Create the radiation counter subsystem
    let subsystem: Box<Subsystem> =
        Box::new(Subsystem::from_path(bus, addr).expect("Failed to create subsystem"));
    // Start the radiation counter service
    Service::new(rc_config, subsystem, QueryRoot, MutationRoot).start();
}
