use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Api};
use cw20_ics20::ibc::Ics20Packet;
use serde_json::Value;

use crate::ibc::types::keccak256;

#[cw_serde]
pub struct IbcHooksMemo {
    wasm: Option<IbcHooksMemoWasm>,
}

#[cw_serde]
pub struct IbcHooksMemoWasm {
    contract: String,
    msg: Value,
}

pub fn parse_ibc_hooks_memo(
    api: &dyn Api,
    channel_id: String,
    packet: &mut Ics20Packet,
) -> anyhow::Result<Option<(Addr, String, Value)>> {
    if let Some(memo) = &packet.memo {
        // We match the memo to the IBC hooks format
        // If it matches, we create the ibc hook sender. They will be the recipient of the funds and the sender of the contract call
        if let Ok(json) = serde_json::from_str::<IbcHooksMemo>(memo) {
            if let Some(wasm) = json.wasm {
                let contract = wasm.contract;

                let sender_original_sender_string = format!("{}/{}", channel_id, packet.sender);

                let bytes: Vec<u8> = keccak256("ibc-wasm-hook-intermediary".as_bytes()).into();
                let step = [bytes, sender_original_sender_string.as_bytes().to_vec()].concat();
                let sender = api.addr_humanize(&keccak256(&step).into())?;
                packet.receiver = sender.to_string();
                return Ok(Some((sender, contract, wasm.msg)));
            }
        }
    }

    Ok(None)
}
