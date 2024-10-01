use cosmwasm_std::testing::MockStorage;
use cosmwasm_std::Empty;
use cw_multi_test::{App, FailingModule, Module};

use crate::default_app;

/// Utility function for asserting outputs returned from failing module.
fn assert_results(failing_module: FailingModule<Empty, Empty, Empty>) {
    let app = default_app();
    let sender_addr = app.api().addr_make("sender");
    let empty_msg = Empty {};
    let mut storage = MockStorage::default();
    assert_eq!(
        format!(r#"Unexpected exec msg Empty from Addr("{}")"#, sender_addr),
        failing_module
            .execute(
                app.api(),
                &mut storage,
                app.router(),
                &app.block_info(),
                sender_addr,
                empty_msg.clone()
            )
            .unwrap_err()
            .to_string()
    );
    assert_eq!(
        "Unexpected custom query Empty",
        failing_module
            .query(
                app.api(),
                &storage,
                &(*app.wrap()),
                &app.block_info(),
                empty_msg.clone()
            )
            .unwrap_err()
            .to_string()
    );
    assert_eq!(
        "Unexpected sudo msg Empty",
        failing_module
            .sudo(
                app.api(),
                &mut storage,
                app.router(),
                &app.block_info(),
                empty_msg
            )
            .unwrap_err()
            .to_string()
    );
}

#[test]
fn failing_module_default_works() {
    assert_results(FailingModule::default());
}

#[test]
fn failing_module_new_works() {
    assert_results(FailingModule::new());
}
