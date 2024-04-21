use crate::state::{DonationTx, Project};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListProjectsResp {
    pub projects: Vec<Project>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListDonationsForProjectByPatronResp {
    pub donations: Vec<DonationTx>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreateProject { name: String },
    Donate { project_id: u128 },
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    ListProjects {},
    ListDonationsForProjectByPatron { project_id: u128, patron: String },
}
