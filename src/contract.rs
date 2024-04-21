use cosmwasm_std::{Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response};

use crate::error::ContractResult;
use crate::msg::{ExecuteMsg, QueryMsg};
use crate::state::{AUTHOR, PROJECT_COUNT};

pub const THRESHOLD: u128 = 10_000;

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> ContractResult<Response> {
    let resp = crate::execute::execute(deps, env, info, msg)?;
    Ok(resp)
}

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: Empty,
) -> ContractResult<Response> {
    PROJECT_COUNT.save(deps.storage, &0u128)?;
    AUTHOR.save(deps.storage, &info.sender)?;
    Ok(Response::new())
}

pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> ContractResult<Binary> {
    let resp = crate::query::query(deps, env, msg)?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::{execute, instantiate, query};

    use cosmwasm_std::{coins, Addr, Coin, Empty, StdResult};
    use cw_multi_test::{App, ContractWrapper, Executor};

    use crate::{
        msg::{ExecuteMsg, ListDonationsForProjectByPatronResp, ListProjectsResp, QueryMsg},
        state::{DonationTx, Project},
    };

    #[test]
    fn test_instantiate() {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &Empty {},
                &[],
                "Donations contract",
                None,
            )
            .unwrap();

        let resp: ListProjectsResp = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::ListProjects {})
            .unwrap();

        let ListProjectsResp { projects } = resp;

        assert!(projects.is_empty());
    }

    #[test]
    fn test_create_project() {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &Empty {},
                &[],
                "Donations contract",
                None,
            )
            .unwrap();

        let resp: ListProjectsResp = app
            .wrap()
            .query_wasm_smart(&addr, &QueryMsg::ListProjects {})
            .unwrap();

        let ListProjectsResp { projects } = resp;

        assert!(projects.is_empty());

        app.execute_contract(
            Addr::unchecked("proj_creator"),
            addr.clone(),
            &ExecuteMsg::CreateProject {
                name: "Project".to_string(),
            },
            &[],
        )
        .unwrap();

        let resp: ListProjectsResp = app
            .wrap()
            .query_wasm_smart(&addr, &QueryMsg::ListProjects {})
            .unwrap();

        let ListProjectsResp { projects } = resp;

        assert_eq!(projects.len(), 1);
    }

    #[test]
    fn test_donate_below_threshold_nonround() {
        let mut app = App::default();

        let contract_owner = app.api().addr_make("contract_owner");
        let proj_owner = app.api().addr_make("proj_owner");
        let patron = app.api().addr_make("patron");

        app.init_modules(|router, _, storage| {
            router
                .bank
                .init_balance(storage, &patron, coins(12, "eth"))
                .unwrap();
        });

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let contract = app
            .instantiate_contract(
                code_id,
                contract_owner.clone(),
                &Empty {},
                &[],
                "Donations contract",
                None,
            )
            .unwrap();

        app.execute_contract(
            proj_owner.clone(),
            contract.clone(),
            &ExecuteMsg::CreateProject {
                name: "projectname".to_string(),
            },
            &[],
        )
        .unwrap();

        let ListProjectsResp { projects } = app
            .wrap()
            .query_wasm_smart(contract.clone(), &QueryMsg::ListProjects {})
            .unwrap();

        assert_eq!(projects.len(), 1);

        app.execute_contract(
            patron.clone(),
            contract.clone(),
            &ExecuteMsg::Donate { project_id: 0 },
            &coins(5, "eth"),
        )
        .unwrap();

        let balance = app.wrap().query_balance(contract_owner, "eth").unwrap();
        assert_eq!(balance, Coin::new(1u128, "eth"));
        let balance = app.wrap().query_balance(contract.clone(), "eth").unwrap();
        assert_eq!(balance, Coin::new(0u128, "eth"));

        // the project owner has received the donation from the patron. 1eth was deducted as a fee due to rounding (5 * 9 / 10 = 4).
        let balance = app.wrap().query_balance(proj_owner, "eth").unwrap();
        assert_eq!(balance, Coin::new(4u128, "eth"));
        let balance = app.wrap().query_balance(patron.clone(), "eth").unwrap();
        assert_eq!(balance, Coin::new(7u128, "eth"));

        let ListDonationsForProjectByPatronResp { donations } = app
            .wrap()
            .query_wasm_smart(
                contract,
                &QueryMsg::ListDonationsForProjectByPatron {
                    project_id: 0,
                    patron: patron.to_string(),
                },
            )
            .unwrap();

        assert_eq!(donations.len(), 1);
        assert_eq!(donations[0], DonationTx(coins(5, "eth")));
    }

    #[test]
    fn test_donate_below_threshold_round() {
        let mut app = App::default();

        let contract_owner = app.api().addr_make("contract_owner");
        let proj_owner = app.api().addr_make("proj_owner");
        let patron = app.api().addr_make("patron");

        app.init_modules(|router, _, storage| {
            router
                .bank
                .init_balance(storage, &patron, coins(20, "eth"))
                .unwrap();
        });

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let contract = app
            .instantiate_contract(
                code_id,
                contract_owner.clone(),
                &Empty {},
                &[],
                "Donations contract",
                None,
            )
            .unwrap();

        app.execute_contract(
            proj_owner.clone(),
            contract.clone(),
            &ExecuteMsg::CreateProject {
                name: "projectname".to_string(),
            },
            &[],
        )
        .unwrap();

        let ListProjectsResp { projects } = app
            .wrap()
            .query_wasm_smart(contract.clone(), &QueryMsg::ListProjects {})
            .unwrap();

        assert_eq!(projects.len(), 1);

        app.execute_contract(
            patron.clone(),
            contract.clone(),
            &ExecuteMsg::Donate { project_id: 0 },
            &coins(10, "eth"),
        )
        .unwrap();

        let balance = app.wrap().query_balance(contract_owner, "eth").unwrap();
        assert_eq!(balance, Coin::new(1u128, "eth"));
        let balance = app.wrap().query_balance(contract.clone(), "eth").unwrap();
        assert_eq!(balance, Coin::new(0u128, "eth"));

        // the project owner has received the donation from the patron.
        let balance = app.wrap().query_balance(proj_owner, "eth").unwrap();
        assert_eq!(balance, Coin::new(9u128, "eth"));
        let balance = app.wrap().query_balance(patron.clone(), "eth").unwrap();
        assert_eq!(balance, Coin::new(10u128, "eth"));

        let ListDonationsForProjectByPatronResp { donations } = app
            .wrap()
            .query_wasm_smart(
                contract,
                &QueryMsg::ListDonationsForProjectByPatron {
                    project_id: 0,
                    patron: patron.to_string(),
                },
            )
            .unwrap();

        assert_eq!(donations.len(), 1);
        assert_eq!(donations[0], DonationTx(coins(10, "eth")));
    }

    #[test]
    fn test_donate_above_threshold_nonround() {
        let mut app = App::default();

        let contract_owner = app.api().addr_make("contract_owner");
        let proj_owner = app.api().addr_make("proj_owner");
        let patron = app.api().addr_make("patron");

        app.init_modules(|router, _, storage| {
            router
                .bank
                .init_balance(storage, &patron, coins(10_002, "eth"))
                .unwrap();
        });

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let contract = app
            .instantiate_contract(
                code_id,
                contract_owner.clone(),
                &Empty {},
                &[],
                "Donations contract",
                None,
            )
            .unwrap();

        app.execute_contract(
            proj_owner.clone(),
            contract.clone(),
            &ExecuteMsg::CreateProject {
                name: "projectname".to_string(),
            },
            &[],
        )
        .unwrap();

        let ListProjectsResp { projects } = app
            .wrap()
            .query_wasm_smart(contract.clone(), &QueryMsg::ListProjects {})
            .unwrap();

        assert_eq!(projects.len(), 1);

        app.execute_contract(
            patron.clone(),
            contract.clone(),
            &ExecuteMsg::Donate { project_id: 0 },
            &coins(10_001, "eth"),
        )
        .unwrap();

        let balance = app.wrap().query_balance(contract_owner, "eth").unwrap();
        assert_eq!(balance, Coin::new(501u128, "eth"));
        let balance = app.wrap().query_balance(contract.clone(), "eth").unwrap();
        assert_eq!(balance, Coin::new(0u128, "eth"));

        // the project owner has received the donation from the patron.
        let balance = app.wrap().query_balance(proj_owner, "eth").unwrap();
        assert_eq!(balance, Coin::new(9_500u128, "eth"));
        let balance = app.wrap().query_balance(patron.clone(), "eth").unwrap();
        assert_eq!(balance, Coin::new(1u128, "eth"));

        let ListDonationsForProjectByPatronResp { donations } = app
            .wrap()
            .query_wasm_smart(
                contract,
                &QueryMsg::ListDonationsForProjectByPatron {
                    project_id: 0,
                    patron: patron.to_string(),
                },
            )
            .unwrap();

        assert_eq!(donations.len(), 1);
        assert_eq!(donations[0], DonationTx(coins(10_001, "eth")));
    }

    #[test]
    fn test_donate_above_threshold_round() {
        let mut app = App::default();

        let contract_owner = app.api().addr_make("contract_owner");
        let proj_owner = app.api().addr_make("proj_owner");
        let patron = app.api().addr_make("patron");

        app.init_modules(|router, _, storage| {
            router
                .bank
                .init_balance(storage, &patron, coins(10_021, "eth"))
                .unwrap();
        });

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let contract = app
            .instantiate_contract(
                code_id,
                contract_owner.clone(),
                &Empty {},
                &[],
                "Donations contract",
                None,
            )
            .unwrap();

        app.execute_contract(
            proj_owner.clone(),
            contract.clone(),
            &ExecuteMsg::CreateProject {
                name: "projectname".to_string(),
            },
            &[],
        )
        .unwrap();

        let ListProjectsResp { projects } = app
            .wrap()
            .query_wasm_smart(contract.clone(), &QueryMsg::ListProjects {})
            .unwrap();

        assert_eq!(projects.len(), 1);

        app.execute_contract(
            patron.clone(),
            contract.clone(),
            &ExecuteMsg::Donate { project_id: 0 },
            &coins(10_020, "eth"),
        )
        .unwrap();

        let balance = app.wrap().query_balance(contract_owner, "eth").unwrap();
        assert_eq!(balance, Coin::new(501u128, "eth"));
        let balance = app.wrap().query_balance(contract.clone(), "eth").unwrap();
        assert_eq!(balance, Coin::new(0u128, "eth"));

        // the project owner has received the donation from the patron.
        let balance = app.wrap().query_balance(proj_owner, "eth").unwrap();
        assert_eq!(balance, Coin::new(9_519u128, "eth"));
        let balance = app.wrap().query_balance(patron.clone(), "eth").unwrap();
        assert_eq!(balance, Coin::new(1u128, "eth"));

        let ListDonationsForProjectByPatronResp { donations } = app
            .wrap()
            .query_wasm_smart(
                contract,
                &QueryMsg::ListDonationsForProjectByPatron {
                    project_id: 0,
                    patron: patron.to_string(),
                },
            )
            .unwrap();

        assert_eq!(donations.len(), 1);
        assert_eq!(donations[0], DonationTx(coins(10_020, "eth")));
    }

    #[test]
    fn test_list_projects() {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &Empty {},
                &[],
                "Donations contract",
                None,
            )
            .unwrap();

        let ListProjectsResp { projects } = app
            .wrap()
            .query_wasm_smart(addr.clone(), &QueryMsg::ListProjects {})
            .unwrap();

        assert!(projects.is_empty());

        app.execute_contract(
            Addr::unchecked("proj_creator"),
            addr.clone(),
            &ExecuteMsg::CreateProject {
                name: "Project0".to_string(),
            },
            &[],
        )
        .unwrap();

        let ListProjectsResp { projects } = app
            .wrap()
            .query_wasm_smart(addr.clone(), &QueryMsg::ListProjects {})
            .unwrap();

        assert_eq!(projects.len(), 1);
        let Project { name, creator } = projects.get(0).unwrap();
        assert_eq!(name, "Project0");
        assert_eq!(creator, Addr::unchecked("proj_creator"));

        app.execute_contract(
            Addr::unchecked("proj_creator"),
            addr.clone(),
            &ExecuteMsg::CreateProject {
                name: "Project1".to_string(),
            },
            &[],
        )
        .unwrap();

        let ListProjectsResp { projects } = app
            .wrap()
            .query_wasm_smart(addr.clone(), &QueryMsg::ListProjects {})
            .unwrap();

        assert_eq!(projects.len(), 2);
        let Project { name, creator } = projects.get(1).unwrap();
        assert_eq!(name, "Project1");
        assert_eq!(creator, Addr::unchecked("proj_creator"));
    }

    #[test]
    fn test_list_donations_for_project_by_patrons() {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let contract = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &Empty {},
                &[],
                "Donations contract",
                None,
            )
            .unwrap();

        let random_person = app.api().addr_make("random_person");
        let patron = app.api().addr_make("patron");

        app.init_modules(|router, _, storage| {
            router
                .bank
                .init_balance(storage, &patron, coins(12, "eth"))
                .unwrap();
        });

        let res: StdResult<ListDonationsForProjectByPatronResp> = app.wrap().query_wasm_smart(
            contract.clone(),
            &QueryMsg::ListDonationsForProjectByPatron {
                // We haven't created any projects yet, so this project_id doesn't exist.
                project_id: 0,
                patron: random_person.to_string(),
            },
        );

        assert!(res.is_err());

        app.execute_contract(
            Addr::unchecked("proj_creator"),
            contract.clone(),
            &ExecuteMsg::CreateProject {
                name: "Project0".to_string(),
            },
            &[],
        )
        .unwrap();

        let ListDonationsForProjectByPatronResp { donations } = app
            .wrap()
            .query_wasm_smart(
                contract.clone(),
                &QueryMsg::ListDonationsForProjectByPatron {
                    project_id: 0,
                    patron: random_person.to_string(),
                },
            )
            .unwrap();

        assert!(donations.is_empty());

        app.execute_contract(
            patron.clone(),
            contract.clone(),
            &ExecuteMsg::Donate { project_id: 0 },
            &coins(10, "eth"),
        )
        .unwrap();

        let ListDonationsForProjectByPatronResp { donations } = app
            .wrap()
            .query_wasm_smart(
                contract.clone(),
                &QueryMsg::ListDonationsForProjectByPatron {
                    project_id: 0,
                    patron: patron.to_string(),
                },
            )
            .unwrap();

        assert_eq!(donations.len(), 1);
        assert_eq!(donations[0], DonationTx(coins(10, "eth")));

        let ListDonationsForProjectByPatronResp { donations } = app
            .wrap()
            .query_wasm_smart(
                contract.clone(),
                &QueryMsg::ListDonationsForProjectByPatron {
                    project_id: 0,
                    patron: random_person.to_string(),
                },
            )
            .unwrap();

        assert!(donations.is_empty());
    }
}
