use cosmwasm_schema::write_api;
use cosmwasm_std::Empty;
use donations_contract::msg::{ExecuteMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: Empty,
        execute: ExecuteMsg,
        query: QueryMsg
    }
}
