use cosmwasm_std::testing::mock_env;
use cw_multi_test::{denom, AppBuilder, Executor, IntoBech32, TokenFactoryStargate};

#[test]
fn create_denom_should_work() {
    const SUBDENOM: &str = "subdenom";
    let admin_address = "admin".into_bech32();
    let user_address = "user".into_bech32();
    let mint_amount = 100_000u128;

    // prepare the blockchain configuration
    let mut app = AppBuilder::default()
        .with_stargate(TokenFactoryStargate)
        .build(|_router, _api, _storage| {});

    // Create Denom
    app.execute(
        admin_address.clone(),
        cosmwasm_std::CosmosMsg::Any(cosmwasm_std::AnyMsg {
            type_url: cw_multi_test::tokenfactory::osmosis::MsgCreateDenom::TYPE_URL.to_string(),
            value: cw_multi_test::tokenfactory::osmosis::MsgCreateDenom {
                sender: admin_address.to_string(),
                subdenom: SUBDENOM.to_string(),
            }
            .to_proto_bytes()
            .into(),
        }),
    )
    .unwrap();

    // admin should be able to mint tokens
    app.execute(
        admin_address.clone(),
        cosmwasm_std::CosmosMsg::Any(cosmwasm_std::AnyMsg {
            type_url: cw_multi_test::tokenfactory::osmosis::MsgMint::TYPE_URL.to_string(),
            value: cw_multi_test::tokenfactory::osmosis::MsgMint {
                sender: admin_address.to_string(),
                amount: Some(cw_multi_test::tokenfactory::Coin {
                    amount: mint_amount.to_string(),
                    denom: denom(admin_address.as_str(), SUBDENOM),
                }),
                mint_to_address: user_address.to_string(),
            }
            .to_proto_bytes()
            .into(),
        }),
    )
    .unwrap();

    // there should be no more delegations
    let balance = app
        .wrap()
        .query_balance(user_address, denom(admin_address.as_str(), SUBDENOM))
        .unwrap();
    assert_eq!(
        balance,
        cosmwasm_std::Coin {
            amount: mint_amount.into(),
            denom: denom(admin_address.as_str(), SUBDENOM),
        }
    );
}
