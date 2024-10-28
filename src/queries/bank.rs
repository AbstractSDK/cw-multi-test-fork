use anyhow::Result as AnyResult;
use async_std::task::block_on;
use cosmwasm_std::{Addr, Coin};
use cw_orch::daemon::RUNTIME;

use crate::wasm_emulation::channel::RemoteChannel;

pub struct BankRemoteQuerier;

impl BankRemoteQuerier {
    pub fn get_balance(remote: RemoteChannel, account: &Addr) -> AnyResult<Vec<Coin>> {
        let querier = cw_orch::daemon::queriers::Bank {
            channel: remote.channel,
            rt_handle: Some(RUNTIME.handle().clone()),
        };
        let distant_amounts: Vec<Coin> = block_on(querier._balance(account, None)).unwrap();
        Ok(distant_amounts)
    }
}
