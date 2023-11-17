use contract::query::query_verify_cosmos;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use msg::ExecMsg;

use crate::error::ContractError;
use crate::msg::{InstantiateMsg, QueryMsg};

mod contract;
pub mod msg;

mod error;
#[cfg(any(test, feature = "tests"))]
pub mod multitest;
mod state;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    contract::instantiate(deps, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::VerifyCosmosSignature {
            message,
            signature,
            public_key,
        } => to_binary(&query_verify_cosmos(
            deps,
            message.as_slice(),
            signature.as_slice(),
            public_key.as_slice(),
        )?),
    }
}
