/// Get the PouchItemUse enum value for the given actor from static data
pub fn get_pouch_item_use(actor: &str) -> i32 {
    crate::generated::actor::ACTOR_USE_MAP
        .get(actor)
        .copied()
        .unwrap_or(8) // default is Item
}

/// Get the PouchItemType enum value for the given actor from static data
pub fn get_pouch_item_type(actor: &str) -> i32 {
    crate::generated::actor::ACTOR_TYPE_MAP
        .get(actor)
        .copied()
        .unwrap_or(7) // default is Material
}

/// Get the general life if the actor has a profile that starts with "Weapon"
pub fn get_weapon_general_life(actor: &str) -> Option<i32> {
    crate::generated::actor::WEAPON_LIFE_MAP.get(actor).copied()
}

/// Get if the actor has `CanStack` tag from static data
pub fn can_stack(actor: &str) -> bool {
    crate::generated::actor::STACKABLE_ACTORS_SORTED
        .binary_search(&actor)
        .is_ok()
}

/// Get if the actor does not have `CannotSell` tag from static data
pub fn can_sell(actor: &str) -> bool {
    crate::generated::actor::NON_SELLABLE_ACTORS_SORTED
        .binary_search(&actor)
        .is_err()
}

/// Get if the actor has the `CanUse` tag from static data
pub fn can_use(actor: &str) -> bool {
    crate::generated::actor::EATABLE_ACTORS_SORTED
        .binary_search(&actor)
        .is_ok()
}
