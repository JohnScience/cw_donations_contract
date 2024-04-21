use crate::error::ContractResult;
use crate::msg::{ListDonationsForProjectByPatronResp, ListProjectsResp, QueryMsg};
use crate::state::{DONATIONS, PROJECTS};
use cosmwasm_std::{to_json_binary, Addr, Binary, Deps, Env};

fn list_projects(deps: &Deps) -> ContractResult<ListProjectsResp> {
    let projects = PROJECTS.load(deps.storage)?;
    let resp = ListProjectsResp { projects };
    Ok(resp)
}

fn list_donations_for_project_by_patron(
    deps: &Deps,
    project_id: u128,
    patron: String,
) -> ContractResult<ListDonationsForProjectByPatronResp> {
    let patron: Addr = deps.api.addr_validate(&patron)?;
    let donations = DONATIONS
        .may_load(deps.storage, (project_id, patron))?
        .unwrap_or_default();
    let resp = ListDonationsForProjectByPatronResp { donations };
    Ok(resp)
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> ContractResult<Binary> {
    use QueryMsg::*;

    let res: Binary = match msg {
        ListProjects {} => to_json_binary(&list_projects(&deps)?)?,
        ListDonationsForProjectByPatron { project_id, patron } => to_json_binary(
            &list_donations_for_project_by_patron(&deps, project_id, patron)?,
        )?,
    };

    Ok(res)
}
