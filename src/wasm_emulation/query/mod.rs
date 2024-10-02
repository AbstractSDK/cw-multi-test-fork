pub mod bank;
pub mod mock_querier;
pub mod staking;
pub mod wasm;
use cosmwasm_std::Storage;

pub use mock_querier::MockQuerier;
pub mod gas;

use anyhow::Result as AnyResult;

use super::{
    channel::RemoteChannel,
    input::{BankStorage, WasmStorage},
};

pub trait ContainsRemote {
    fn with_remote(self, remote: RemoteChannel) -> Self;

    fn set_remote(&mut self, remote: RemoteChannel);
}

pub trait AllWasmQuerier {
    fn query_all(&self, storage: &dyn Storage) -> AnyResult<WasmStorage>;
}

pub trait AllBankQuerier {
    fn query_all(&self, storage: &dyn Storage) -> AnyResult<BankStorage>;
}
