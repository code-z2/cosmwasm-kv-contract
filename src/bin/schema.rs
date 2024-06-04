use cosmwasm_schema::write_api;
use kv_store_contract::msg::{ConstructorMsg, ExecuteMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: ConstructorMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}
