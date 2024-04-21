use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct DonationTx(pub Vec<Coin>);

#[cw_serde]
pub struct Project {
    pub name: String,
    pub creator: Addr,
}

impl Project {
    pub fn new(name: String, creator: Addr) -> Self {
        Project { name, creator }
    }
}

// TODO: consider coupling PROJECT_COUNT and PROJECTS into a single struct
// We don't store an Item<Vec> because it is inefficient to load and store the entire list of projects every time we want to add a new project.
// Source: https://book.cosmwasm.com/cross-contract/map-storage.html
pub const PROJECT_COUNT: Item<u128> = Item::new("project_count");
pub const PROJECTS: Map<u128, Project> = Map::new("projects");

// The map from a pair (project_id, patron) to a list of donations. The recorded donations are before the fees are deducted.
// We don't use a newtype around u128 because it'd require implementing cw_storage_plus::PrimaryKey trait, which gets a bit verbose.
pub const DONATIONS: Map<(u128, Addr), Vec<DonationTx>> = Map::new("donations");

// Q: should we use a constant instead?
pub const AUTHOR: Item<Addr> = Item::new("author");
