#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use msg::{ConstructorMsg, ExecuteMsg, QueryMsg};
use thiserror::Error;

mod contract;
pub mod msg;
mod state;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,       // utiltiy for communicating with external/contract state
    _env: Env, // representss blockchain state like chainId, blockHeight, blockTime, msg.sender
    _info: MessageInfo, // information about the message triggerign the execution
    _msg: ConstructorMsg, // constructor params
) -> StdResult<Response> {
    contract::constructor(_deps, _info, _msg.base_fee)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use contract::execute;
    use msg::ExecuteMsg::*;

    match _msg {
        SetValue { key, value } => execute::insert_value(_deps, _info, key, value),
        UpdateValue { key, value } => {
            execute::update_value(_deps, _info, key, value).map_err(ContractError::Std)
        }
        DeleteValue { key } => execute::delete_value(_deps, _info, key).map_err(ContractError::Std),
        Withdraw {} => execute::withdraw(_deps, _env, _info),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    use contract::query;
    use msg::QueryMsg::*;

    match _msg {
        Value { key } => to_json_binary(&query::get_value(_deps, key)?),
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized - only {owner} can call it")]
    Unauthorized { owner: String },

    #[error("Storage fee too low - expected {amount} minimum")]
    StorageFeeTooLow { amount: u128 },
}
