use crate::error::{ContractResult, NonexistentProjectIdError};
use crate::msg::{ListDonationsForProjectByPatronResp, ListProjectsResp, QueryMsg};
use crate::state::{DONATIONS, PROJECTS, PROJECT_COUNT};
use cosmwasm_std::{to_json_binary, Addr, Binary, Deps, Env};

fn list_projects(deps: &Deps) -> ContractResult<ListProjectsResp> {
    let project_count = PROJECT_COUNT.load(deps.storage)?;
    let mut projects = vec![];
    for i in 0..project_count {
        let project = PROJECTS.load(deps.storage, i)?;
        projects.push(project);
    }
    let resp = ListProjectsResp { projects };
    Ok(resp)
}

fn list_donations_for_project_by_patron(
    deps: &Deps,
    project_id: u128,
    patron: String,
) -> ContractResult<ListDonationsForProjectByPatronResp> {
    let project_count = PROJECT_COUNT.load(deps.storage)?;
    if project_id >= project_count {
        return Err(NonexistentProjectIdError(project_id).into());
    }
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
