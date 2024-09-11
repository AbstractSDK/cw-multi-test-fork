use cosmwasm_std::{Binary, Response};

#[derive(Debug)]
pub enum WasmOutput<T> {
    Execute(Response<T>),
    Instantiate(Response<T>),
    Query(Binary),
    Sudo(Response<T>),
    Reply(Response<T>),
    Migrate(Response<T>),
}

#[derive(Debug)]
pub struct StorageChanges {
    pub current_keys: Vec<(Vec<u8>, Vec<u8>)>,
    pub removed_keys: Vec<Vec<u8>>,
}

#[derive(Debug)]
pub struct WasmRunnerOutput<T> {
    pub wasm: WasmOutput<T>,
    pub storage: StorageChanges,
    pub gas_used: u64,
}
