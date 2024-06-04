use cosmwasm_schema::cw_serde;
use cosmwasm_schema::QueryResponses;
use cosmwasm_std::Coin;

#[cw_serde]
pub struct ConstructorMsg {
    #[serde(default)]
    pub base_fee: Coin,
}

#[cw_serde]
pub enum ExecuteMsg {
    SetValue { key: String, value: String },
    UpdateValue { key: String, value: String },
    DeleteValue { key: String },
    Withdraw {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ValueResp)]
    Value { key: String },
}

#[cw_serde]
pub struct ValueResp {
    pub key: String,
    pub value: Option<String>,
}
