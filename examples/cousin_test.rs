fn main() {
    test().unwrap()
}
use std::path::Path;

use anyhow::Result as AnyResult;
use clone_cw_multi_test::{
    wasm_emulation::{channel::RemoteChannel, query::ContainsRemote},
    App, AppBuilder, BankKeeper, ContractWrapper, Executor, MockApiBech32, WasmKeeper,
};
use cosmwasm_std::{Addr, Empty};
use counter::msg::{ExecuteMsg, GetCountResponse, QueryMsg};
use cw_orch::daemon::networks::PHOENIX_1;
use tokio::runtime::Runtime;

mod counter;

pub const SENDER: &str = "terra17c6ts8grcfrgquhj3haclg44le8s7qkx6l2yx33acguxhpf000xqhnl3je";
fn increment(app: &mut App<BankKeeper, MockApiBech32>, contract: Addr) -> AnyResult<()> {
    let sender = Addr::unchecked(SENDER);
    app.execute_contract(
        sender.clone(),
        contract.clone(),
        &ExecuteMsg::Increment {},
        &[],
    )?;
    Ok(())
}

fn count(app: &App<BankKeeper, MockApiBech32>, contract: Addr) -> AnyResult<GetCountResponse> {
    Ok(app
        .wrap()
        .query_wasm_smart(contract.clone(), &QueryMsg::GetCount {})?)
}

fn raw_cousin_count(
    app: &App<BankKeeper, MockApiBech32>,
    contract: Addr,
) -> AnyResult<GetCountResponse> {
    Ok(app
        .wrap()
        .query_wasm_smart(contract.clone(), &QueryMsg::GetRawCousinCount {})?)
}

fn cousin_count(
    app: &App<BankKeeper, MockApiBech32>,
    contract: Addr,
) -> AnyResult<GetCountResponse> {
    Ok(app
        .wrap()
        .query_wasm_smart(contract.clone(), &QueryMsg::GetCousinCount {})?)
}

fn test() -> AnyResult<()> {
    env_logger::init();
    let rust_contract = ContractWrapper::new(
        counter::contract::execute,
        counter::contract::instantiate,
        counter::contract::query,
    );

    let code = std::fs::read(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("artifacts")
            .join("counter_contract_with_cousin.wasm"),
    )
    .unwrap();

    let runtime = Runtime::new()?;
    let chain = PHOENIX_1;
    let remote_channel = RemoteChannel::new(
        &runtime,
        chain.grpc_urls,
        chain.chain_id,
        chain.network_info.pub_address_prefix,
    )?;

    let wasm = WasmKeeper::<Empty, Empty>::new().with_remote(remote_channel.clone());

    let bank = BankKeeper::new().with_remote(remote_channel.clone());

    // First we instantiate a new app
    let mut app = AppBuilder::default()
        .with_wasm(wasm)
        .with_bank(bank)
        .with_remote(remote_channel)
        .with_api(MockApiBech32::new(chain.network_info.pub_address_prefix))
        .build(|_, _, _| {});

    let sender = Addr::unchecked(SENDER);
    let rust_code_id = app.store_code(Box::new(rust_contract));
    let wasm_code_id = app.store_wasm_code(code);

    let counter_rust = app
        .instantiate_contract(
            rust_code_id,
            sender.clone(),
            &counter::msg::InstantiateMsg { count: 1 },
            &[],
            "cousin-counter",
            Some(sender.to_string()),
        )
        .unwrap();

    let counter_wasm = app
        .instantiate_contract(
            wasm_code_id,
            sender.clone(),
            &counter::msg::InstantiateMsg { count: 1 },
            &[],
            "cousin-counter",
            Some(sender.to_string()),
        )
        .unwrap();

    app.execute_contract(
        sender.clone(),
        counter_rust.clone(),
        &ExecuteMsg::SetCousin {
            cousin: counter_wasm.to_string(),
        },
        &[],
    )?;

    app.execute_contract(
        sender.clone(),
        counter_wasm.clone(),
        &ExecuteMsg::SetCousin {
            cousin: counter_rust.to_string(),
        },
        &[],
    )?;

    // Increment the count on both and see what's what
    increment(&mut app, counter_rust.clone())?;
    increment(&mut app, counter_rust.clone())?;
    increment(&mut app, counter_wasm.clone())?;

    // Assert the count
    assert_eq!(count(&app, counter_rust.clone())?.count, 3);
    assert_eq!(count(&app, counter_wasm.clone())?.count, 2);

    // Assert the raw cousin count
    assert_eq!(raw_cousin_count(&app, counter_rust.clone())?.count, 2);
    assert_eq!(raw_cousin_count(&app, counter_wasm.clone())?.count, 3);

    // Assert the cousin count
    assert_eq!(cousin_count(&app, counter_rust.clone())?.count, 2);
    assert_eq!(cousin_count(&app, counter_wasm.clone())?.count, 3);

    Ok(())
}