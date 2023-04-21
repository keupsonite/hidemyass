use cosmwasm_std::{BankMsg, Coin, CosmosMsg, Deps, MessageInfo, QueryRequest, Response, StdError, StdResult, SubMsg, to_binary, Uint128, WasmQuery};
use crate::{query};
use crate::error::PaymentError;

pub fn coin_amount(info: &MessageInfo, amount: Uint128) -> Result<Coin, PaymentError> {
    match info.funds.len() {
        0 => Err(PaymentError::NoFunds {}),
        1 => {
            let coin = &info.funds[0];
            if coin.amount.is_zero() {
                Err(PaymentError::NoFunds {})
            } else if coin.amount < amount {
                Err(PaymentError::NoEnoughFunds {})
            } else {
                Ok(coin.clone())
            }
        }
        _ => Err(PaymentError::MultipleDenoms {}),
    }
}

pub fn send_payment(sender: String, receiver: String, coin: Vec<Coin>) -> Response<> {
    let send_msg = SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
        to_address: receiver.clone(),
        amount: coin
    }));
    Response::new()
        .add_submessages(vec![send_msg])
        .add_attribute("action", "execute")
        .add_attribute("owner", sender)
}
