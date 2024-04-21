use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response};
use error::ContractResult;

mod contract;
pub mod error;
pub mod execute;
pub mod msg;
pub mod query;
pub mod state;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: Empty,
) -> ContractResult<Response> {
    let resp = contract::instantiate(deps, env, info, msg)?;
    Ok(resp)
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: msg::QueryMsg) -> ContractResult<Binary> {
    let resp = contract::query(deps, env, msg)?;
    Ok(resp)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: msg::ExecuteMsg,
) -> ContractResult<Response> {
    let resp = contract::execute(deps, env, info, msg)?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::instantiate;

    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::Empty;

    #[test]
    fn proper_instantiation() {
        let mut deps = mock_dependencies();

        let msg = Empty {};
        let info = mock_info("creator", &[]);
        let env = mock_env();

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }
}
