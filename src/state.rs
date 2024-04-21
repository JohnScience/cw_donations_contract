use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct DonationTx(pub Vec<Coin>);

#[cw_serde]
pub struct Project {
    pub name: String,
    pub(crate) creator: Addr,
}

impl Project {
    pub fn new(name: String, creator: Addr) -> Self {
        Project { name, creator }
    }
}

// TODO: change the type to Map<u128, Project>
pub const PROJECTS: Item<Vec<Project>> = Item::new("projects");
// The map from a pair (project_id, patron) to a list of donations. The recorded donations are before the fees are deducted.
// We don't use a newtype around u128 because it'd require implementing cw_storage_plus::PrimaryKey trait, which gets a bit verbose.
pub const DONATIONS: Map<(u128, Addr), Vec<DonationTx>> = Map::new("donations");
pub const AUTHOR: Item<Addr> = Item::new("author");
