use crate::state::{DonationTx, Project};
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct ListProjectsResp {
    pub projects: Vec<Project>,
}

#[cw_serde]
pub struct ListDonationsForProjectByPatronResp {
    pub donations: Vec<DonationTx>,
}

#[cw_serde]
pub enum ExecuteMsg {
    CreateProject { name: String },
    Donate { project_id: u128 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ListProjectsResp)]
    ListProjects {},
    #[returns(ListDonationsForProjectByPatronResp)]
    ListDonationsForProjectByPatron { project_id: u128, patron: String },
}
