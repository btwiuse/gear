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

//! CLI arguments in one place.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::config;

#[derive(Clone, Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// URL of Ethereum RPC endpoint
    #[arg(
        long = "ethereum-rpc",
        alias = "rpc",
        default_value = "http://localhost:8545"
    )]
    pub ethereum_rpc: String,

    /// URL of Ethereum Beacon Chain RPC endpoint
    #[arg(
        long = "ethereum-beacon-rpc",
        alias = "beacon-rpc",
        default_value = "http://localhost:5052"
    )]
    pub ethereum_beacon_rpc: String,

    /// Base path where application settings are stored
    #[arg(long = "base-path")]
    pub base_path: Option<PathBuf>,

    #[command(subcommand)]
    pub extra_command: Option<ExtraCommands>,
}

#[derive(Clone, Debug, Subcommand)]
pub enum ExtraCommands {
    GenerateKey,
    ListKeys,
    ClearKeys,
    Sign(SigningArgs),
}

#[derive(Clone, Debug, Parser)]
pub struct SigningArgs {
    message: String,
}

impl ExtraCommands {
    pub fn run(&self, config: &config::Config) -> anyhow::Result<()> {
        match self {
            ExtraCommands::GenerateKey => {
                let signer = hypercore_signer::Signer::new(config.key_path.clone())?;

                let new_pub = signer.generate_key()?;

                println!("New public key stored: {}", new_pub);
                println!("Ethereum address: {}", new_pub.to_address());
            }

            ExtraCommands::ClearKeys => {
                let signer = hypercore_signer::Signer::new(config.key_path.clone())?;

                println!("Total {} keys will be cleared: ", signer.list_keys()?.len());
                signer.clear_keys()?;
                println!("Total {} keys left: ", signer.list_keys()?.len());
            }

            ExtraCommands::ListKeys => {
                let signer = hypercore_signer::Signer::new(config.key_path.clone())?;

                let key_list = signer.list_keys()?;

                for key in &key_list {
                    println!("Ethereum Address: {}, public: {}", key.to_address(), key);
                }

                println!("Total {}", key_list.len())
            }

            ExtraCommands::Sign(ref signing_args) => {
                let signer = hypercore_signer::Signer::new(config.key_path.clone())?;

                let message = &signing_args.message;

                let key_list = signer.list_keys()?;

                if key_list.is_empty() {
                    println!("No keys found, please generate a key first");
                    return Ok(());
                }

                println!("Signing with all ({}) keys:", key_list.len());

                for key in &key_list {
                    println!("Ethereum Address: {}, public: {}", key.to_address(), key);
                    println!("Signature: {}", &signer.sign(*key, message.as_bytes())?);
                    println!("--------------------------------------------");
                }
            }
        }

        Ok(())
    }
}