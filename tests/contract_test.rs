use cosmwasm_std::coin;
use cosmwasm_std::coins;
use cosmwasm_std::Addr;
use cosmwasm_std::Empty;
use cw_multi_test::App;
use cw_multi_test::Contract;
use cw_multi_test::ContractWrapper;

use cw_multi_test::Executor;
use kv_store_contract;
use kv_store_contract::msg::ConstructorMsg;
use kv_store_contract::msg::ExecuteMsg;
use kv_store_contract::msg::QueryMsg;
use kv_store_contract::msg::ValueResp;
use kv_store_contract::ContractError;

fn store_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        kv_store_contract::execute,
        kv_store_contract::instantiate,
        kv_store_contract::query,
    );
    Box::new(contract)
}

fn get_app() -> App {
    let sender = Addr::unchecked("sender");

    App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender, coins(20, "sei"))
            .unwrap()
    })
}

#[test]
fn test_get_value() {
    let mut app = get_app();

    let contract_id = app.store_code(store_contract());

    let contract_addr = app
        .instantiate_contract(
            contract_id,
            Addr::unchecked("sender"),
            &ConstructorMsg {
                base_fee: coin(2, "sei"),
            },
            &[],
            "KV Store Contract",
            None,
        )
        .unwrap();

    let resp: ValueResp = app
        .wrap()
        .query_wasm_smart(
            contract_addr,
            &QueryMsg::Value {
                key: "it works".to_string(),
            },
        )
        .unwrap();

    assert_eq!(
        resp,
        ValueResp {
            key: "it works".to_string(),
            value: None,
        }
    );
}

#[test]
fn test_set_value() {
    let mut app = get_app();

    let contract_id = app.store_code(store_contract());

    let contract_addr = app
        .instantiate_contract(
            contract_id,
            Addr::unchecked("sender"),
            &ConstructorMsg {
                base_fee: coin(2, "sei"),
            },
            &[],
            "KV Store Contract",
            None,
        )
        .unwrap();

    app.execute_contract(
        Addr::unchecked("sender"),
        contract_addr.clone(),
        &ExecuteMsg::SetValue {
            key: "it works".to_string(),
            value: "first".to_string(),
        },
        &coins(2, "sei"),
    )
    .unwrap();

    let resp: ValueResp = app
        .wrap()
        .query_wasm_smart(
            contract_addr,
            &QueryMsg::Value {
                key: "it works".to_string(),
            },
        )
        .unwrap();

    assert_eq!(
        resp,
        ValueResp {
            key: "it works".to_string(),
            value: Some("first".to_string()),
        }
    );
}

#[test]
fn test_update_value() {
    let mut app = get_app();

    let contract_id = app.store_code(store_contract());

    let contract_addr = app
        .instantiate_contract(
            contract_id,
            Addr::unchecked("sender"),
            &ConstructorMsg {
                base_fee: coin(2, "sei"),
            },
            &[],
            "KV Store Contract",
            None,
        )
        .unwrap();

    app.execute_contract(
        Addr::unchecked("sender"),
        contract_addr.clone(),
        &ExecuteMsg::SetValue {
            key: "it works".to_string(),
            value: "first".to_string(),
        },
        &coins(2, "sei"),
    )
    .unwrap();

    app.execute_contract(
        Addr::unchecked("sender"),
        contract_addr.clone(),
        &ExecuteMsg::UpdateValue {
            key: "it works".to_string(),
            value: "second".to_string(),
        },
        &[],
    )
    .unwrap();

    let resp: ValueResp = app
        .wrap()
        .query_wasm_smart(
            contract_addr,
            &QueryMsg::Value {
                key: "it works".to_string(),
            },
        )
        .unwrap();

    assert_eq!(
        resp,
        ValueResp {
            key: "it works".to_string(),
            value: Some("second".to_string()),
        }
    );
}

#[test]
fn test_delete_value() {
    let mut app = get_app();

    let contract_id = app.store_code(store_contract());

    let contract_addr = app
        .instantiate_contract(
            contract_id,
            Addr::unchecked("sender"),
            &ConstructorMsg {
                base_fee: coin(2, "sei"),
            },
            &[],
            "KV Store Contract",
            None,
        )
        .unwrap();

    app.execute_contract(
        Addr::unchecked("sender"),
        contract_addr.clone(),
        &ExecuteMsg::SetValue {
            key: "it works".to_string(),
            value: "first".to_string(),
        },
        &coins(2, "sei"),
    )
    .unwrap();

    app.execute_contract(
        Addr::unchecked("sender"),
        contract_addr.clone(),
        &ExecuteMsg::DeleteValue {
            key: "it works".to_string(),
        },
        &[],
    )
    .unwrap();

    let resp: ValueResp = app
        .wrap()
        .query_wasm_smart(
            contract_addr,
            &QueryMsg::Value {
                key: "it works".to_string(),
            },
        )
        .unwrap();

    assert_eq!(
        resp,
        ValueResp {
            key: "it works".to_string(),
            value: None,
        }
    );
}

#[test]
fn test_withdraw() {
    let mut app = get_app();

    let contract_id = app.store_code(store_contract());

    let contract_addr = app
        .instantiate_contract(
            contract_id,
            Addr::unchecked("sender"),
            &ConstructorMsg {
                base_fee: coin(2, "sei"),
            },
            &coins(10, "sei"),
            "KV Store Contract",
            None,
        )
        .unwrap();

    app.execute_contract(
        Addr::unchecked("sender"),
        contract_addr.clone(),
        &ExecuteMsg::Withdraw {},
        &[],
    )
    .unwrap();

    assert_eq!(
        app.wrap()
            .query_all_balances(Addr::unchecked("sender"),)
            .unwrap(),
        coins(20, "sei")
    );
    assert_eq!(
        app.wrap()
            .query_all_balances(Addr::unchecked("alice"),)
            .unwrap(),
        vec![]
    );
    assert_eq!(
        app.wrap().query_all_balances(contract_addr).unwrap(),
        vec![]
    );
}

#[test]
fn test_unauthorized_withdraw() {
    let mut app = get_app();

    let contract_id = app.store_code(store_contract());

    let contract_addr = app
        .instantiate_contract(
            contract_id,
            Addr::unchecked("sender"),
            &ConstructorMsg {
                base_fee: coin(2, "sei"),
            },
            &[],
            "KV Store Contract",
            None,
        )
        .unwrap();

    let err = app
        .execute_contract(
            Addr::unchecked("attacker"),
            contract_addr,
            &ExecuteMsg::Withdraw {},
            &[],
        )
        .unwrap_err();

    assert_eq!(
        ContractError::Unauthorized {
            owner: Addr::unchecked("sender").into()
        },
        err.downcast().unwrap()
    );
}
