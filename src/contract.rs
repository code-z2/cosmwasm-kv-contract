use crate::state::{OWNER, STORAGE_FEE};
use cosmwasm_std::{Coin, DepsMut, MessageInfo, Response, StdResult};

pub fn constructor(deps: DepsMut, info: MessageInfo, base_fee: Coin) -> StdResult<Response> {
    STORAGE_FEE.save(deps.storage, &base_fee)?;
    OWNER.save(deps.storage, &info.sender)?;
    Ok(Response::new())
}

pub mod query {
    use crate::msg::ValueResp;
    use crate::state::STORE;
    use cosmwasm_std::{Deps, StdResult};

    pub fn get_value(deps: Deps, key: String) -> StdResult<ValueResp> {
        let result = STORE.may_load(deps.storage, key.to_owned())?;

        Ok(ValueResp {
            key: key.to_string(),
            value: result,
        })
    }
}

pub mod execute {
    use crate::{
        state::{OWNER, STORAGE_FEE, STORE},
        ContractError,
    };
    use cosmwasm_std::{BankMsg, DepsMut, Env, MessageInfo, Response, StdResult};

    fn get_response(action: &str, sender: &str, value: String) -> Response {
        Response::new()
            .add_attribute("action", action)
            .add_attribute("sender", sender)
            .add_attribute("value", value)
    }

    pub fn insert_value(
        deps: DepsMut,
        info: MessageInfo,
        key: String,
        value: String,
    ) -> Result<Response, ContractError> {
        let fee = STORAGE_FEE.load(deps.storage)?;

        if info
            .funds
            .iter()
            .any(|coin| coin.denom == fee.denom && coin.amount >= fee.amount)
        {
            STORE.save(deps.storage, key, &value)?;
            let response = get_response("insert_value", info.sender.as_str(), value);
            Ok(response)
        } else {
            Err(ContractError::StorageFeeTooLow {
                amount: fee.amount.u128(),
            })
        }
    }

    pub fn update_value(
        deps: DepsMut,
        info: MessageInfo,
        key: String,
        value: String,
    ) -> StdResult<Response> {
        let result = STORE.update(deps.storage, key, |_| -> StdResult<_> {
            Ok(value.to_owned())
        })?;

        let response = get_response("update_value", info.sender.as_str(), result);
        Ok(response)
    }

    pub fn delete_value(deps: DepsMut, info: MessageInfo, key: String) -> StdResult<Response> {
        STORE.remove(deps.storage, key.to_owned());

        let response = get_response("delete_value", info.sender.as_str(), key);
        Ok(response)
    }

    pub fn withdraw(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
        let owner = OWNER.load(deps.storage)?;
        if info.sender != owner {
            return Err(ContractError::Unauthorized {
                owner: owner.to_string(),
            });
        }

        let balance = deps.querier.query_all_balances(&env.contract.address)?;
        let bank_msg = BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: balance,
        };

        let response = get_response("withdraw", info.sender.as_str(), "all balances".to_string())
            .add_message(bank_msg);

        Ok(response)
    }
}
