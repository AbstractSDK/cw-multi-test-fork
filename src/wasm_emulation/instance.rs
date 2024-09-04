use cosmwasm_vm::internals::compile;
use cosmwasm_vm::internals::{instance_from_module, make_compiling_engine};
use cosmwasm_vm::{
    Backend, BackendApi, Instance, InstanceOptions, Querier, Size, Storage, VmResult,
};
use wasmer::{Module, Store};

pub fn create_module(code: &[u8]) -> VmResult<Module> {
    let engine = make_compiling_engine(None);
    let module = compile(&engine, code)?;
    Ok(module)
}

/// This is the only Instance constructor that can be called from outside of cosmwasm-vm,
/// e.g. in test code that needs a customized variant of cosmwasm_vm::testing::mock_instance*.
pub fn instance_from_reused_module<A, S, Q>(
    module: Module,
    backend: Backend<A, S, Q>,
    options: InstanceOptions,
    memory_limit: Option<Size>,
) -> VmResult<Instance<A, S, Q>>
where
    A: BackendApi + 'static,
    S: Storage + 'static,
    Q: Querier + 'static,
{
    let engine = make_compiling_engine(memory_limit);
    let store = Store::new(engine);
    instance_from_module(store, &module, backend, options.gas_limit, None)
}
