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

#![deny(warnings)]

#[macro_use]
extern crate juniper;

mod model;
mod schema;

use crate::model::Subsystem;
use crate::schema::{MutationRoot, QueryRoot};
use kubos_service::{Config, Service};
use log::error;
use syslog::Facility;

fn main() {
    syslog::init(
        Facility::LOG_DAEMON,
        log::LevelFilter::Debug,
        Some("radiation-counter-service"),
    )
    .unwrap();

    let config = Config::new("radiation-counter-service")
        .map_err(|err| {
            error!("Failed to load service config: {:?}", err);
            err
        })
        .unwrap();

    let device = config.get("device").unwrap();
    let bus = device["bus"].as_str().expect("Failed to get I2C bus value");
    let addr = device["addr"].as_integer().expect("Failed to get I2C address value");

    println!("I2C Bus:     {}", bus);
    println!("I2C Address: {}", addr);

    // let subsystem: Box<Subsystem> = Box::new(
    //     Subsystem::from_path(bus, addr)
    //         .map_err(|err| {
    //             error!("Failed to create subsystem: {:?}", err);
    //             err
    //         })
    //         .unwrap(),
    // );

    Service::new(config, Subsystem::new(), QueryRoot, MutationRoot).start();
}
