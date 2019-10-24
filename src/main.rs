//
// Copyright (C) 2017 Kubos Corporation
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

// #![deny(warnings)]

#[macro_use]
extern crate juniper;
#[macro_use]
extern crate kubos_service;

pub mod models;
pub mod schema;

use crate::models::subsystem::Subsystem;
use crate::schema::mutation::Root as MutationRoot;
use crate::schema::query::Root as QueryRoot;
use kubos_service::{Config, Service};
use log::{error,info};
use syslog::Facility;

fn main() {
    syslog::init(
        Facility::LOG_DAEMON,
        log::LevelFilter::Debug,
        Some("radiation-counter-service"),
    )
    .unwrap();

    let rc_config = Config::new("radiation-counter-service").unwrap();

    // TODO: fail gracefully
    // Radiation counter bus and addr
    let device = rc_config.get("device").unwrap();
    let bus = device["bus"].as_str().expect("Failed to get RC I2C bus value");
    let addr = device["addr"].as_integer().expect("Failed to get RC I2C address value") as u16;
    let power_channel = device["power_channel"].as_integer().expect("Failed to get RC power channel value") as u8;
    
    info!("I2C Bus:       {}", bus);
    info!("I2C Address:   {}", addr);
    info!("Power Channel: {}", power_channel);

    let subsystem: Box<Subsystem> = Box::new(
        Subsystem::from_path(bus, addr, power_channel)
            .map_err(|err| {
                error!("Failed to create subsystem: {:?}", err);
                err
            })
            .unwrap(),
    );

    Service::new(rc_config, subsystem, QueryRoot, MutationRoot).start();
}
