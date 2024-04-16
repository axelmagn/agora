//! data models for simulation primitives.
//!
//! Structs in this module are designed for serialization / deserialization with
//! TOML or similar via config files.  Inter-object references are therefore
//! handled through simple string IDs, which may need to be optimized at
//! runtime. Similarly, structs may need to be converted to more efficient
//! representations for simulation.

use std::{collections::BTreeMap, ops::Add, time::Duration};

use serde::{Deserialize, Serialize};

/// A ware that is produced and consumed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WareType {
    pub display_name: String,
}
pub type WareTypeId = String;

/// A type of building.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingType {
    pub display_name: String,
}
pub type BuildingTypeId = String;

/// A type of building.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationClass {
    pub display_name: String,
}
pub type PopulationClassId = String;

/// Recipes are processes that convert input wares into output wares.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    /// The building this recipe is performed in.
    pub building: BuildingTypeId,
    /// The input and output ware amounts, with inputs represented by negative
    /// amounts.
    pub conversion: Balance<WareTypeId>,
    /// The time it takes to perform this recipe.
    pub cycle: Duration,
    /// The labor balance of this recipe, with negative amounts representing
    /// labor costs. Labor surplus is not anticipated, but may be possible.
    pub labor: Balance<PopulationClassId>,
}
pub type RecipeId = String;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Balance<K> {
    amounts: BTreeMap<K, i64>,
}

impl<K> Balance<K> {
    pub fn new() -> Self {
        Self {
            amounts: BTreeMap::new(),
        }
    }
}

impl<K> Add for &Balance<K>
where
    K: Clone + Ord,
{
    type Output = Balance<K>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut out = self.clone();
        for (key, rhs_value) in rhs.amounts.iter() {
            let lhs_value = out.amounts.entry(key.clone()).or_insert(0);
            *lhs_value += rhs_value;
        }
        out
    }
}
