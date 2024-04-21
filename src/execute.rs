use cosmwasm_std::{Addr, Coin, DepsMut, Env, MessageInfo, Response, Uint128};

use crate::contract::THRESHOLD;
use crate::error::ContractResult;
use crate::msg::ExecuteMsg;
use crate::state::{DonationTx, Project, AUTHOR, DONATIONS, PROJECTS, PROJECT_COUNT};

pub fn create_project(deps: &mut DepsMut, name: String, creator: Addr) -> ContractResult<()> {
    // TODO: implement the .push() for the structure representing the pair (PROJECTS, PROJECT_COUNT)
    let project_count = PROJECT_COUNT.load(deps.storage)?;
    PROJECT_COUNT.save(deps.storage, &(project_count + 1))?;

    PROJECTS.save(deps.storage, project_count, &Project::new(name, creator))?;

    Ok(())
}

fn split_by_recipient(mut funds: Vec<Coin>) -> split_by_recipient::Output {
    let for_project_creator: Vec<Coin> = funds
        .iter()
        .map(|coin| {
            let amount = match coin.amount.u128() {
                // The payment is 90% of the donation
                0..=THRESHOLD => coin.amount * Uint128::new(9) / Uint128::new(10u128),
                // The payment is 95% of the donation
                _ => coin.amount * Uint128::new(19) / Uint128::new(20),
            };
            let denom = coin.denom.clone();
            Coin { denom, amount }
        })
        .collect();

    // modify the original funds to deduct the amount for the project creator, leaving the remainder for the contract author
    for (coin_for_contract_author, coin_for_project_creator) in
        funds.iter_mut().zip(for_project_creator.iter())
    {
        coin_for_contract_author.amount -= coin_for_project_creator.amount;
    }

    split_by_recipient::Output {
        for_project_creator,
        for_contract_author: funds,
    }
}

mod split_by_recipient {
    use cosmwasm_std::{Addr, BankMsg, Coin};

    pub(super) struct Output {
        pub(super) for_project_creator: Vec<Coin>,
        pub(super) for_contract_author: Vec<Coin>,
    }

    impl Output {
        fn into_bank_message_iter(
            self,
            project_creator: Addr,
            contract_author: Addr,
        ) -> impl Iterator<Item = cosmwasm_std::BankMsg> {
            let project_creator_msg = cosmwasm_std::BankMsg::Send {
                to_address: project_creator.to_string(),
                amount: self.for_project_creator,
            };

            let contract_author_msg = cosmwasm_std::BankMsg::Send {
                to_address: contract_author.to_string(),
                amount: self.for_contract_author,
            };

            [project_creator_msg, contract_author_msg]
                .into_iter()
                .filter(|msg| matches!(msg, BankMsg::Send { amount, .. } if !amount.is_empty()))
        }

        pub(super) fn into_response(
            self,
            project_creator: Addr,
            contract_author: Addr,
        ) -> cosmwasm_std::Response {
            let mut resp = cosmwasm_std::Response::new();
            for bank_msg in self.into_bank_message_iter(project_creator, contract_author) {
                resp = resp.add_message(bank_msg);
            }
            resp
        }
    }
}

fn record_donation(deps: &mut DepsMut, info: &MessageInfo, project_id: u128) -> ContractResult<()> {
    let mut donations = DONATIONS
        .may_load(deps.storage, (project_id, info.sender.clone()))?
        .unwrap_or_default();

    donations.push(DonationTx(info.funds.clone()));
    DONATIONS.save(deps.storage, (project_id, info.sender.clone()), &donations)?;

    Ok(())
}

pub fn donate(deps: &mut DepsMut, info: MessageInfo, project_id: u128) -> ContractResult<Response> {
    let project = PROJECTS.load(deps.storage, project_id)?;

    record_donation(deps, &info, project_id)?;

    let contract_author = AUTHOR.load(deps.storage)?;

    let resp =
        split_by_recipient(info.funds).into_response(project.creator.clone(), contract_author);

    Ok(resp)
}

pub fn execute(
    mut deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> ContractResult<Response> {
    let resp = match msg {
        ExecuteMsg::CreateProject { name } => {
            let creator = info.sender;
            create_project(&mut deps, name, creator)?;
            Response::new()
        }
        ExecuteMsg::Donate { project_id } => donate(&mut deps, info, project_id)?,
    };
    Ok(resp)
}
