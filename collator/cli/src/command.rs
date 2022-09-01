// This file is part of Gear.

// Copyright (C) 2021-2022 Gear Technologies Inc.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use crate::cli::{Cli, RelayChainCli, Subcommand};
use codec::Encode;
use cumulus_client_cli::generate_genesis_block;
use cumulus_primitives_core::ParaId;
use frame_benchmarking_cli::{BenchmarkCmd, SUBSTRATE_REFERENCE_HARDWARE};
use log::info;
#[cfg(feature = "rococo-gear-native")]
use rococo_gear_runtime::RuntimeApi;
use runtime_primitives::Block;
use sc_cli::{
    ChainSpec, CliConfiguration, DefaultConfigurationValues, ImportParams, KeystoreParams,
    NetworkParams, Result, RuntimeVersion, SharedParams, SubstrateCli,
};
use sc_service::{
    config::{BasePath, PrometheusConfig},
    TaskManager,
};
use service::{self, chain_spec, IdentifyVariant, RococoGearRuntimeExecutor};
use sp_core::hexdisplay::HexDisplay;
use sp_runtime::traits::{AccountIdConversion, Block as _};
use std::net::SocketAddr;

fn load_spec(id: &str) -> std::result::Result<Box<dyn ChainSpec>, String> {
    Ok(match id {
        #[cfg(feature = "rococo-gear-native")]
        "rococo_gear-dev" => Box::new(chain_spec::rococo_gear::development_config()),

        #[cfg(feature = "rococo-gear-native")]
        "rococo_gear-local" => Box::new(chain_spec::rococo_gear::local_testnet_config()),

        // Fallback (generic chainspec)
        "" => {
            log::warn!(
                "No ChainSpec.id specified, so using default one, based on rococo-gear runtime"
            );
            Box::new(chain_spec::rococo_gear::local_testnet_config())
        }

        path => {
            let path = std::path::PathBuf::from(path);

            let chain_spec = Box::new(
                sc_service::GenericChainSpec::<(), chain_spec::Extensions>::from_json_file(
                    path.clone(),
                )?,
            ) as Box<dyn ChainSpec>;

            // when a new runtime is being added move this block to else-block.
            // Check gear-node for details.
            {
                #[cfg(feature = "rococo-gear-native")]
                {
                    // TODO: remove this assert when there is another runtime
                    assert!(chain_spec.is_rococo_gear() || chain_spec.is_dev());
                    Box::new(chain_spec::rococo_gear::ChainSpec::from_json_file(path)?)
                }

                #[cfg(not(feature = "rococo-gear-native"))]
                return Err("Rococo Gear runtime is not available. Please compile the node with default features to enable it.".into());
            }
        }
    })
}

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "Gear Node".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        format!(
            "Gear Parachain Collator\n\nThe command-line arguments provided first will be \
            passed to the parachain node, while the arguments provided after -- will be passed \
            to the relay chain node.\n\n\
            {} <parachain-args> -- <relay-chain-args>",
            Self::executable_name()
        )
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "gear-tech.io".into()
    }

    fn copyright_start_year() -> i32 {
        2021
    }

    fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
        load_spec(id)
    }

    fn native_runtime_version(spec: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
        match spec {
            #[cfg(feature = "rococo-gear-native")]
            _ => &service::rococo_gear_runtime::VERSION,
            #[cfg(not(feature = "rococo-gear-native"))]
            _ => panic!("Invalid chain spec"),
        }
    }
}

impl SubstrateCli for RelayChainCli {
    fn impl_name() -> String {
        "Gear Parachain Collator".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        format!(
            "Gear Parachain Collator\n\nThe command-line arguments provided first will be \
            passed to the parachain node, while the arguments provided after -- will be passed \
            to the relay chain node.\n\n\
            {} <parachain-args> -- <relay-chain-args>",
            Self::executable_name()
        )
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "gear-tech.io".into()
    }

    fn copyright_start_year() -> i32 {
        2021
    }

    fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
        polkadot_cli::Cli::from_iter([RelayChainCli::executable_name()].iter()).load_spec(id)
    }

    fn native_runtime_version(chain_spec: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
        polkadot_cli::Cli::native_runtime_version(chain_spec)
    }
}

macro_rules! construct_async_run {
    (|$components:ident, $cli:ident, $cmd:ident, $config:ident| $( $code:tt )* ) => {{
        let runner = $cli.create_runner($cmd)?;
        runner.async_run(|$config| {
            let $components = service::new_partial::<
                RuntimeApi,
                RococoGearRuntimeExecutor,
                _
            >(
                &$config,
                service::rococo_gear_build_import_queue,
            )?;
            let task_manager = $components.task_manager;
            { $( $code )* }.map(|v| (v, task_manager))
        })
    }}
}

/// Parse and run command line arguments
pub fn run() -> sc_cli::Result<()> {
    let cli = Cli::from_args();

    match &cli.subcommand {
        Some(Subcommand::BuildSpec(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
        }
        Some(Subcommand::CheckBlock(cmd)) => {
            construct_async_run!(|components, cli, cmd, config| {
                Ok(cmd.run(components.client, components.import_queue))
            })
        }
        Some(Subcommand::ExportBlocks(cmd)) => {
            construct_async_run!(|components, cli, cmd, config| {
                Ok(cmd.run(components.client, config.database))
            })
        }
        Some(Subcommand::ExportState(cmd)) => {
            construct_async_run!(|components, cli, cmd, config| {
                Ok(cmd.run(components.client, config.chain_spec))
            })
        }
        Some(Subcommand::ImportBlocks(cmd)) => {
            construct_async_run!(|components, cli, cmd, config| {
                Ok(cmd.run(components.client, components.import_queue))
            })
        }
        Some(Subcommand::PurgeChain(cmd)) => {
            let runner = cli.create_runner(cmd)?;

            runner.sync_run(|config| {
                let polkadot_cli = RelayChainCli::new(
                    &config,
                    [RelayChainCli::executable_name()]
                        .iter()
                        .chain(cli.relaychain_args.iter()),
                );

                let polkadot_config = SubstrateCli::create_configuration(
                    &polkadot_cli,
                    &polkadot_cli,
                    config.tokio_handle.clone(),
                )
                .map_err(|err| format!("Relay chain argument error: {}", err))?;

                cmd.run(config, polkadot_config)
            })
        }
        Some(Subcommand::ExportGenesisState(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|_config| {
                let spec = cli.load_spec(&cmd.shared_params.chain.clone().unwrap_or_default())?;
                let state_version = Cli::native_runtime_version(&spec).state_version();
                cmd.run::<Block>(&*spec, state_version)
            })
        }
        Some(Subcommand::ExportGenesisWasm(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|_config| {
                let spec = cli.load_spec(&cmd.shared_params.chain.clone().unwrap_or_default())?;
                cmd.run(&*spec)
            })
        }
        Some(Subcommand::Revert(cmd)) => {
            construct_async_run!(|components, cli, cmd, config| {
                Ok(cmd.run(components.client, components.backend, None))
            })
        }
        Some(Subcommand::Benchmark(cmd)) => {
            let runner = cli.create_runner(cmd)?;

            // Switch on the concrete benchmark sub-command
            match cmd {
                BenchmarkCmd::Pallet(cmd) => {
                    if cfg!(feature = "runtime-benchmarks") {
                        runner
                            .sync_run(|config| cmd.run::<Block, RococoGearRuntimeExecutor>(config))
                    } else {
                        Err("Benchmarking wasn't enabled when building the node. \
                            You can enable it with `--features runtime-benchmarks`."
                            .into())
                    }
                }
                BenchmarkCmd::Block(cmd) => runner.sync_run(|config| {
                    let partials = service::new_partial::<RuntimeApi, RococoGearRuntimeExecutor, _>(
                        &config,
                        service::rococo_gear_build_import_queue,
                    )?;
                    cmd.run(partials.client)
                }),
                BenchmarkCmd::Storage(cmd) => runner.sync_run(|config| {
                    let partials = service::new_partial::<RuntimeApi, RococoGearRuntimeExecutor, _>(
                        &config,
                        service::rococo_gear_build_import_queue,
                    )?;
                    let db = partials.backend.expose_db();
                    let storage = partials.backend.expose_storage();

                    cmd.run(config, partials.client.clone(), db, storage)
                }),
                BenchmarkCmd::Overhead(_) => Err("Unsupported benchmarking command".into()),
                BenchmarkCmd::Machine(cmd) => {
                    runner.sync_run(|config| cmd.run(&config, SUBSTRATE_REFERENCE_HARDWARE.clone()))
                }
            }
        }
        Some(Subcommand::GearRuntimeTest(cmd)) => {
            let runner = cli.create_runner(cmd)?;

            runner.sync_run(|config| cmd.run(config))
        }
        Some(Subcommand::TryRuntime(cmd)) => {
            if cfg!(feature = "try-runtime") {
                let runner = cli.create_runner(cmd)?;

                // grab the task manager.
                let registry = &runner
                    .config()
                    .prometheus_config
                    .as_ref()
                    .map(|cfg| &cfg.registry);
                let task_manager =
                    TaskManager::new(runner.config().tokio_handle.clone(), *registry)
                        .map_err(|e| format!("Error: {:?}", e))?;

                runner.async_run(|config| {
                    Ok((
                        cmd.run::<Block, RococoGearRuntimeExecutor>(config),
                        task_manager,
                    ))
                })
            } else {
                Err("Try-runtime must be enabled by `--features try-runtime`.".into())
            }
        }

        Some(Subcommand::Key(cmd)) => Ok(cmd.run(&cli)?),

        None => {
            let runner = cli.create_runner(&cli.run.base.normalize())?;
            let collator_options = cli.run.base.collator_options();

            runner.run_node_until_exit(|config| async move {
                let hwbench = if !cli.run.no_hardware_benchmarks {
                    config.database.path().map(|database_path| {
                        let _ = std::fs::create_dir_all(&database_path);
                        sc_sysinfo::gather_hwbench(Some(database_path))
                    })
                } else {
                    None
                };

                let para_id = chain_spec::Extensions::try_get(&*config.chain_spec)
                    .map(|e| e.para_id)
                    .ok_or("Could not find parachain ID in chain-spec.")?;

                let polkadot_cli = RelayChainCli::new(
                    &config,
                    [RelayChainCli::executable_name()]
                        .iter()
                        .chain(cli.relaychain_args.iter()),
                );

                let id = ParaId::from(para_id);

                let parachain_account =
                    AccountIdConversion::<polkadot_primitives::v2::AccountId>::into_account_truncating(&id);

                let state_version = Cli::native_runtime_version(&config.chain_spec).state_version();
                let block: Block = generate_genesis_block(&*config.chain_spec, state_version)
                    .map_err(|e| format!("{:?}", e))?;
                let genesis_state = format!("0x{:?}", HexDisplay::from(&block.header().encode()));

                let tokio_handle = config.tokio_handle.clone();
                let polkadot_config =
                    SubstrateCli::create_configuration(&polkadot_cli, &polkadot_cli, tokio_handle)
                        .map_err(|err| format!("Relay chain argument error: {}", err))?;

                info!("Parachain id: {:?}", id);
                info!("Parachain Account: {}", parachain_account);
                info!("Parachain genesis state: {}", genesis_state);
                info!(
                    "Is collating: {}",
                    if config.role.is_authority() {
                        "yes"
                    } else {
                        "no"
                    }
                );

                service::start_rococo_gear_node(
                    config,
                    polkadot_config,
                    collator_options,
                    id,
                    hwbench,
                )
                .await
                .map(|(task_manager, ..)| task_manager)
                .map_err(Into::into)
            })
        }
    }
}

impl DefaultConfigurationValues for RelayChainCli {
    fn p2p_listen_port() -> u16 {
        30334
    }

    fn rpc_ws_listen_port() -> u16 {
        9945
    }

    fn rpc_http_listen_port() -> u16 {
        9934
    }

    fn prometheus_listen_port() -> u16 {
        9616
    }
}

impl CliConfiguration<Self> for RelayChainCli {
    fn shared_params(&self) -> &SharedParams {
        self.base.base.shared_params()
    }

    fn import_params(&self) -> Option<&ImportParams> {
        self.base.base.import_params()
    }

    fn network_params(&self) -> Option<&NetworkParams> {
        self.base.base.network_params()
    }

    fn keystore_params(&self) -> Option<&KeystoreParams> {
        self.base.base.keystore_params()
    }

    fn base_path(&self) -> Result<Option<BasePath>> {
        Ok(self
            .shared_params()
            .base_path()
            .or_else(|| self.base_path.clone().map(Into::into)))
    }

    fn rpc_http(&self, default_listen_port: u16) -> Result<Option<SocketAddr>> {
        self.base.base.rpc_http(default_listen_port)
    }

    fn rpc_ipc(&self) -> Result<Option<String>> {
        self.base.base.rpc_ipc()
    }

    fn rpc_ws(&self, default_listen_port: u16) -> Result<Option<SocketAddr>> {
        self.base.base.rpc_ws(default_listen_port)
    }

    fn prometheus_config(
        &self,
        default_listen_port: u16,
        chain_spec: &Box<dyn ChainSpec>,
    ) -> Result<Option<PrometheusConfig>> {
        self.base
            .base
            .prometheus_config(default_listen_port, chain_spec)
    }

    fn init<F>(
        &self,
        _support_url: &String,
        _impl_version: &String,
        _logger_hook: F,
        _config: &sc_service::Configuration,
    ) -> Result<()>
    where
        F: FnOnce(&mut sc_cli::LoggerBuilder, &sc_service::Configuration),
    {
        unreachable!("PolkadotCli is never initialized; qed");
    }

    fn chain_id(&self, is_dev: bool) -> Result<String> {
        let chain_id = self.base.base.chain_id(is_dev)?;

        Ok(if chain_id.is_empty() {
            self.chain_id.clone().unwrap_or_default()
        } else {
            chain_id
        })
    }

    fn role(&self, is_dev: bool) -> Result<sc_service::Role> {
        self.base.base.role(is_dev)
    }

    fn transaction_pool(&self) -> Result<sc_service::config::TransactionPoolOptions> {
        self.base.base.transaction_pool()
    }

    fn state_cache_child_ratio(&self) -> Result<Option<usize>> {
        self.base.base.state_cache_child_ratio()
    }

    fn rpc_methods(&self) -> Result<sc_service::config::RpcMethods> {
        self.base.base.rpc_methods()
    }

    fn rpc_ws_max_connections(&self) -> Result<Option<usize>> {
        self.base.base.rpc_ws_max_connections()
    }

    fn rpc_cors(&self, is_dev: bool) -> Result<Option<Vec<String>>> {
        self.base.base.rpc_cors(is_dev)
    }

    fn default_heap_pages(&self) -> Result<Option<u64>> {
        self.base.base.default_heap_pages()
    }

    fn force_authoring(&self) -> Result<bool> {
        self.base.base.force_authoring()
    }

    fn disable_grandpa(&self) -> Result<bool> {
        self.base.base.disable_grandpa()
    }

    fn max_runtime_instances(&self) -> Result<Option<usize>> {
        self.base.base.max_runtime_instances()
    }

    fn announce_block(&self) -> Result<bool> {
        self.base.base.announce_block()
    }

    fn telemetry_endpoints(
        &self,
        chain_spec: &Box<dyn ChainSpec>,
    ) -> Result<Option<sc_telemetry::TelemetryEndpoints>> {
        self.base.base.telemetry_endpoints(chain_spec)
    }

    fn node_name(&self) -> Result<String> {
        self.base.base.node_name()
    }
}