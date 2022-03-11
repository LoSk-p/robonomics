///////////////////////////////////////////////////////////////////////////////
//
//  Copyright 2018-2021 Robonomics Network <research@robonomics.network>
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//
///////////////////////////////////////////////////////////////////////////////
//! Robonomics I/O CLI interface.

#![deny(missing_docs)]

use crate::error::Result;
use clap::Parser

/// Substrate friendly CLI I/O subsystem interaction.
#[derive(Parser, Debug)]
pub struct IoCmd {
    /// I/O device operation to run.
    #[clap(subcommand)]
    pub operation: Operation,
}

impl IoCmd {
    /// Run I/O operation on device.
    pub fn run(&self) -> Result<()> {
        match &self.operation {
            Operation::Read(source) => source.run(),
            Operation::Write(sink) => sink.run(),
        }
    }
}

/// I/O operation command.
#[derive(Parser, Debug)]
pub enum Operation {
    /// Read information from device.
    Read(super::SourceCmd),
    /// Write information into device.
    Write(super::SinkCmd),
}
