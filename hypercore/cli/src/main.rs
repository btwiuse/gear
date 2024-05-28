// This file is part of Gear.
//
// Copyright (C) 2024 Gear Technologies Inc.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Cargo extension for building gear programs.

mod args;
mod config;

use anyhow::Context;
use crate::{args::Args, config::Config};
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let config = Config::try_from(args)
        .with_context(|| "Failed to create configuration")?;

    env_logger::try_init()
        .with_context(|| "Failed to initialize logger")?;

    log::info!("Ethereum observerl RPC: {}", config.ethereum_rpc);
    log::info!("Database directory: {:?}", config.database_path);

    Ok(())

}
