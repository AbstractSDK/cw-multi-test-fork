#![cfg(test)]

use clone_cw_multi_test::wasm_emulation::channel::RemoteChannel;
use clone_cw_multi_test::{no_init, App};
use cw_orch::daemon::RUNTIME;

mod test_api;
mod test_app;
mod test_app_builder;
mod test_attributes;
mod test_bank;
mod test_contract_storage;
mod test_module;
mod test_prefixed_storage;
#[cfg(feature = "staking")]
mod test_staking;
mod test_wasm;

extern crate clone_cw_multi_test as cw_multi_test;

mod test_contracts {

    pub mod counter {
        use clone_cw_multi_test::{Contract, ContractWrapper};
        use cosmwasm_schema::cw_serde;
        use cosmwasm_std::{
            to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError,
            WasmMsg,
        };
        use cw_storage_plus::Item;

        const COUNTER: Item<u64> = Item::new("counter");

        #[cw_serde]
        pub enum CounterQueryMsg {
            Counter {},
        }

        #[cw_serde]
        pub struct CounterResponseMsg {
            pub value: u64,
        }

        fn instantiate(
            deps: DepsMut,
            _env: Env,
            _info: MessageInfo,
            _msg: Empty,
        ) -> Result<Response, StdError> {
            COUNTER.save(deps.storage, &1).unwrap();
            Ok(Response::default())
        }

        fn execute(
            deps: DepsMut,
            _env: Env,
            _info: MessageInfo,
            _msg: WasmMsg,
        ) -> Result<Response, StdError> {
            if let Some(mut counter) = COUNTER.may_load(deps.storage).unwrap() {
                counter += 1;
                COUNTER.save(deps.storage, &counter).unwrap();
            }
            Ok(Response::default())
        }

        fn query(deps: Deps, _env: Env, msg: CounterQueryMsg) -> Result<Binary, StdError> {
            match msg {
                CounterQueryMsg::Counter { .. } => Ok(to_json_binary(&CounterResponseMsg {
                    value: COUNTER.may_load(deps.storage).unwrap().unwrap(),
                })?),
            }
        }

        pub fn contract() -> Box<dyn Contract<Empty>> {
            Box::new(ContractWrapper::new_with_empty(execute, instantiate, query))
        }

        #[cfg(feature = "cosmwasm_1_2")]
        pub fn contract_with_checksum() -> Box<dyn Contract<Empty>> {
            Box::new(
                ContractWrapper::new_with_empty(execute, instantiate, query).with_checksum(
                    cosmwasm_std::Checksum::generate(&[1, 2, 3, 4, 5, 6, 7, 8, 9]),
                ),
            )
        }
    }
}

use cw_orch::{daemon::networks::XION_TESTNET_1, prelude::ChainInfo};
pub const CHAIN: ChainInfo = XION_TESTNET_1;
pub fn remote_channel() -> RemoteChannel {
    RemoteChannel::new(
        &RUNTIME,
        CHAIN.grpc_urls,
        CHAIN.chain_id,
        CHAIN.network_info.pub_address_prefix,
    )
    .unwrap()
}

pub fn default_app() -> App {
    App::new(remote_channel(), no_init)
}
