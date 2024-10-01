use cw_multi_test::App;
use cw_storage_plus::Map;

use crate::default_app;

const USER: &str = "user";
const USERS: Map<&str, u64> = Map::new("users");
const AMOUNT: u64 = 100;

#[test]
fn initializing_app_should_work() {
    let mut app = default_app();
    let mut amount = 0;
    app.init_modules(|_router, api, storage| {
        USERS
            .save(storage, api.addr_make(USER).as_str(), &AMOUNT)
            .unwrap();
    });
    app.read_module(|_router, api, storage| {
        amount = USERS.load(storage, api.addr_make(USER).as_str()).unwrap()
    });
    assert_eq!(AMOUNT, amount);
}
