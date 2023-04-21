use cosmwasm_std::{entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, StdError};

use cw2::set_contract_version;

use wasmswap::msg::{Token2ForToken1PriceResponse};

use crate::error::ContractError;
use crate::msg::{EmptyInstantiateMsg, ExecuteMsg, MigrateMsg};
use crate::query::{QueryMsg, SwapDetailsResponse};
use crate::state::{SwapDetails, SWAP_DETAILS, ADMIN, Admin};
use crate::tools;

const CONTRACT_NAME: &str = "crates.io:junomint-prices";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    let ver = cw2::get_contract_version(deps.storage)?;

    // ensure we are migrating from an allowed contract
    if ver.contract != CONTRACT_NAME {
        return Err(StdError::generic_err("Can only upgrade from same type").into());
    }

    // set the new version
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // do any desired state migrations...
    let admin = deps.api.addr_validate(&msg.admin)?;
    ADMIN.save(deps.storage, &Admin{ address: admin }).ok();

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: EmptyInstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    ADMIN.save(deps.storage, &Admin{ address: info.sender }).ok();
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
    }
}
