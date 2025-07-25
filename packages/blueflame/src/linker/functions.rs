use crate::env::GameVer;
use crate::game::{CookItem, FixedSafeString40, PouchItem, WeaponModifierInfo, singleton_instance};
use crate::memory::{Ptr, mem};
use crate::processor::{self, Cpu2, reg};

/// Get item with the given value or default value.
pub fn get_item(
    cpu: &mut Cpu2,
    actor: &str,
    value: Option<i32>,
    modifier: Option<WeaponModifierInfo>,
) -> Result<(), processor::Error> {
    match value {
        Some(value) => get_item_with_value(cpu, actor, value, modifier),
        None => get_item_with_default_value(cpu, actor, modifier),
    }
}

/// Get one item with the default life. Calls `doGetItem_0x710073A464`
pub fn get_item_with_default_value(
    cpu: &mut Cpu2,
    actor: &str,
    modifier: Option<WeaponModifierInfo>,
) -> Result<(), processor::Error> {
    cpu.reset_stack();

    let actor_name_ptr = helper::stack_alloc_string40(cpu, actor)?;
    let modifier_ptr = helper::stack_alloc_weapon_modifier(cpu, modifier)?;

    reg! { cpu:
        x[0] = actor_name_ptr,
        x[1] = modifier_ptr,
    };

    if cpu.proc.env().is160() {
        panic!("1.6.0 not implemented yet");
        // cpu.native_jump_to_main_offset(0x0096f3d0)?;
    } else {
        cpu.native_jump_to_main_offset(0x0073a464)?;
    }

    cpu.stack_check::<FixedSafeString40>(actor_name_ptr.to_raw())?;
    cpu.stack_check::<WeaponModifierInfo>(modifier_ptr.to_raw())?;
    Ok(())
}

/// Get a cook item with the cook data. Calls `uking::ui::PauseMenuDataMgr::cookItemGet`
#[allow(clippy::too_many_arguments)]
pub fn get_cook_item(
    cpu: &mut Cpu2,
    actor: &str,
    ingredients: &[impl AsRef<str>],
    life_recover: Option<f32>,
    effect_time: Option<i32>,
    sell_price: Option<i32>,
    effect_id: Option<i32>,
    vitality_boost: Option<f32>, // i.e effect level
) -> Result<(), processor::Error> {
    cpu.reset_stack();

    let this_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;

    let cook_item = cpu.stack_alloc::<CookItem>()?;
    let cook_item = Ptr!(<CookItem>(cook_item));
    let m = cpu.proc.memory_mut();

    cook_item.construct(m)?;
    Ptr!(&cook_item->actor_name).safe_store(actor, m)?;
    for (i, ingredient) in ingredients.iter().take(5).enumerate() {
        let p = cook_item.ith_ingredient(i as u64);
        p.safe_store(ingredient, m)?;
    }
    mem! { m:
        *(&cook_item->life_recover) = life_recover.unwrap_or(0.0);
        *(&cook_item->effect_time) = effect_time.unwrap_or(0);
        *(&cook_item->sell_price) = sell_price.unwrap_or(0);
        *(&cook_item->effect_id) = effect_id.unwrap_or(-1);
        *(&cook_item->vitality_boost) = vitality_boost.unwrap_or(0.0);
        *(&cook_item->is_crit) = false;
    };
    reg! { cpu:
        x[0] = this_ptr,
        x[1] = cook_item,
    };

    if cpu.proc.env().is160() {
        cpu.native_jump_to_main_offset(0x010be740)?;
    } else {
        cpu.native_jump_to_main_offset(0x00970060)?;
    }

    cpu.stack_check::<CookItem>(cook_item.to_raw())?;
    Ok(())
}

pub fn get_item_with_value(
    cpu: &mut Cpu2,
    actor: &str,
    value: i32,
    modifier: Option<WeaponModifierInfo>,
) -> Result<(), processor::Error> {
    cpu.reset_stack();

    let this_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    let name_ptr = helper::stack_alloc_string40(cpu, actor)?;
    let modifier_ptr = helper::stack_alloc_weapon_modifier(cpu, modifier)?;

    reg! { cpu:
        x[0] = this_ptr,
        x[1] = name_ptr,
        w[2] = value,
        x[3] = modifier_ptr,
    };

    if cpu.proc.env().is160() {
        panic!("1.6.0 not implemented yet");
        // cpu.native_jump_to_main_offset(0x0096f3d0)?;
    } else {
        cpu.native_jump_to_main_offset(0x0096efb8)?;
    }
    cpu.stack_check::<FixedSafeString40>(name_ptr.to_raw())?;
    cpu.stack_check::<WeaponModifierInfo>(modifier_ptr.to_raw())?;
    Ok(())
}

/// Call `uking::ui::PauseMenuDataMgr::autoEquipLastAddedItem()`
pub fn equip_last_added_item(cpu: &mut Cpu2) -> Result<(), processor::Error> {
    cpu.reset_stack();
    let this_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    reg! { cpu: x[0] = this_ptr, };
    // TODO --160
    cpu.native_jump_to_main_offset(0x00970264)
}

/// `uking::ui::PauseMenuDataMgr::updateEquippedItemArray`
///
/// This is re-implemented since it's inlined in 1.6 (0x1203fec)
pub fn update_equipped_item_array(cpu: &mut Cpu2) -> Result<(), processor::Error> {
    cpu.reset_stack();
    let pmdm = singleton_instance!(pmdm(cpu.proc.memory()))?;
    let nullptrs = [0u64.into(); 4];
    mem! { (cpu.proc.memory_mut()):
        *(&pmdm->mEquippedWeapons) = nullptrs;
    };

    let list1 = Ptr!(&pmdm->mList1);
    let (mut iter, iter_end) = {
        let m = cpu.proc.memory();
        (list1.begin(m)?, list1.end(m)?)
    };
    while iter != iter_end {
        let m = cpu.proc.memory();
        let item_ptr = Ptr!(<PouchItem>(iter.get_tptr()));
        // no null check
        mem! { m:
            let item_type = *(&item_ptr->mType);
        };
        // > Shield
        if item_type > 3 {
            break;
        }
        mem! { m:
            let is_equipped = *(&item_ptr->mEquipped);
        };
        if is_equipped {
            // safe array
            let i = if item_type as u32 > 3 {
                0u64
            } else {
                item_type as u64
            };
            mem! { (cpu.proc.memory_mut()):
                *(pmdm.equipped_weapons().ith(i)) = item_ptr;
            };
        }
        iter.next(cpu.proc.memory())?;
    }

    Ok(())
}

/// `uking::ui::PauseMenuDataMgr::createHoldingItemActors` (0x97AB34 in 1.5)
///
/// This is re-implemented since it's inlined in 1.6
pub fn create_holding_items(cpu: &mut Cpu2) -> Result<(), processor::Error> {
    cpu.reset_stack();
    let pmdm = singleton_instance!(pmdm(cpu.proc.memory()))?;
    let grabbed_items = pmdm.grabbed_items();
    for i in 0..5 {
        let grabbed_item = grabbed_items.ith(i);
        mem! {(cpu.proc.memory()):
            let info = *grabbed_item;
        };
        let item = info.mItem;
        if item.is_nullptr() || info.mIsActorSpawned {
            continue;
        }
        let name_ptr = Ptr!(&item->mName);
        helper::assure_termination(cpu, name_ptr.reinterpret())?;
        let name_str_ptr = name_ptr.cstr(cpu.proc.memory())?;
        reg! { cpu:
            x[0] = name_str_ptr,
            x[1] = 0 // Heap*, we don't actually need this
        };
        match cpu.proc.env().game_ver {
            GameVer::X150 => {
                cpu.native_jump_to_main_offset(0x0073c5b4)?;
            }
            GameVer::X160 => {
                cpu.native_jump_to_main_offset(0x00d23b20)?;
            }
        }
        mem! {(cpu.proc.memory_mut()):
            *(&grabbed_item->mIsActorSpawned) = true;
        };
    }

    Ok(())
}

/// Call `uking::ui::PauseMenuDataMgr::deleteRemovedItems` (0x7100977128 in 1.5).
/// i.e. removes translucent items
pub fn delete_removed_items(cpu: &mut Cpu2) -> Result<(), processor::Error> {
    cpu.reset_stack();
    let this_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    reg! { cpu: x[0] = this_ptr, };
    // TODO --160
    cpu.native_jump_to_main_offset(0x00977128)
}

/// Call `uking::ui::PauseMenuDataMgr::removeGrabbedItems`
pub fn remove_held_items(cpu: &mut Cpu2) -> Result<(), processor::Error> {
    cpu.reset_stack();
    let this_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    reg! { cpu: x[0] = this_ptr, };
    // TODO --160
    cpu.native_jump_to_main_offset(0x00971b00)
}

/// Call `uking::ui::PauseMenuDataMgr::canGrabAnotherItem`
///
/// This is re-implemented since it's so simple and not worth to find
/// the right function in versions other than 1.5
pub fn can_hold_another_item(cpu: &mut Cpu2) -> Result<bool, processor::Error> {
    let this_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    let grabbed_items = this_ptr.grabbed_items();
    for i in 0..grabbed_items.len() {
        let grabbed_item = grabbed_items.ith(i as u64);
        mem! {(cpu.proc.memory()):
            let grabbed_item = *(&grabbed_item->mItem);
        };
        if grabbed_item.is_nullptr() {
            return Ok(true);
        }
    }

    Ok(false)
}

/// Call `uking::ui::PauseMenuDataMgr::cannotGetItem`
///
/// Count should be 1 for unstackable, and value for stackable
pub fn cannot_get_item(cpu: &mut Cpu2, name: &str, count: i32) -> Result<bool, processor::Error> {
    cpu.reset_stack();
    let this_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    let name_ptr = helper::stack_alloc_string40(cpu, name)?;
    reg! { cpu:
        x[0] = this_ptr,
        x[1] = name_ptr,
        w[2] = count,
    };
    // TODO --160
    cpu.native_jump_to_main_offset(0x0096d72c)?;

    reg! { cpu: x[0] => let ret_val: bool };

    cpu.stack_check::<FixedSafeString40>(name_ptr.to_raw())?;
    Ok(ret_val)
}

/// Call `uking::ui::PauseMenuDataMgr::trashItem` (holds or drops item)
pub fn trash_item(cpu: &mut Cpu2, tab_index: i32, slot_index: i32) -> Result<(), processor::Error> {
    cpu.reset_stack();
    let this_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    reg! { cpu:
        x[0] = this_ptr,
        w[1] = tab_index,
        w[2] = slot_index,
    };
    // TODO --160
    cpu.native_jump_to_main_offset(0x009766d8)
}

/// Call `uking::ui::PauseMenuDataMgr::equipFromTabSlot`
pub fn equip_from_tab_slot(
    cpu: &mut Cpu2,
    tab_index: i32,
    slot_index: i32,
) -> Result<(), processor::Error> {
    cpu.reset_stack();
    let this_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    reg! { cpu:
        x[0] = this_ptr,
        w[1] = tab_index,
        w[2] = slot_index,
    };
    // TODO --160
    cpu.native_jump_to_main_offset(0x009762a8)
}

/// Call `uking::ui::PauseMenuDataMgr::unequipFromTabSlot`
pub fn unequip_from_tab_slot(
    cpu: &mut Cpu2,
    tab_index: i32,
    slot_index: i32,
) -> Result<(), processor::Error> {
    cpu.reset_stack();
    let this_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    reg! { cpu:
        x[0] = this_ptr,
        w[1] = tab_index,
        w[2] = slot_index,
    };
    // TODO --160
    cpu.native_jump_to_main_offset(0x009764a0)
}

/// Call `uking::ui::PauseMenuDataMgr::unholdGrabbedItems` (0x710097ADFC)
pub fn unhold_items(cpu: &mut Cpu2) -> Result<(), processor::Error> {
    cpu.reset_stack();
    let this_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    reg! { cpu:
        x[0] = this_ptr,
    };
    // TODO --160
    cpu.native_jump_to_main_offset(0x0097ADFC)
}

/// Call `uking::ui::PauseMenuDataMgr::sellItem`
pub fn sell_item(
    cpu: &mut Cpu2,
    item: Ptr![PouchItem],
    count: i32,
) -> Result<(), processor::Error> {
    cpu.reset_stack();
    let this_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    reg! { cpu:
        x[0] = this_ptr,
        x[1] = item,
        w[2] = count
    };
    // TODO --160 (probably inlined)
    cpu.native_jump_to_main_offset(0x0097D250)
}

/// Call `uking::ui::PauseMenuDataMgr::removeWeaponIfEquipped`
pub fn remove_weapon_if_equipped(cpu: &mut Cpu2, name: &str) -> Result<(), processor::Error> {
    cpu.reset_stack();
    let this_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    let name_ptr = helper::stack_alloc_string40(cpu, name)?;
    reg! { cpu:
        x[0] = this_ptr,
        x[1] = name_ptr
    };
    // TODO --160
    cpu.native_jump_to_main_offset(0x00970a04)?;
    cpu.stack_check::<FixedSafeString40>(name_ptr.to_raw())?;
    Ok(())
}

/// Call `uking::ui::PauseMenuDataMgr::equipWeapon`, which equips an item by pointer
pub fn equip_weapon(cpu: &mut Cpu2, item: Ptr![PouchItem]) -> Result<(), processor::Error> {
    cpu.reset_stack();
    let this_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    reg! { cpu:
        x[0] = this_ptr,
        x[1] = item,
    };
    // TODO --160 (probably inlined)
    cpu.native_jump_to_main_offset(0x0097a944)
}

/// Unequip weapon by pointer. This is effectively the same as
/// `uking::ui::PauseMenuDataMgr::unequip`
pub fn unequip(cpu: &mut Cpu2, item: Ptr![PouchItem]) -> Result<(), processor::Error> {
    if item.is_nullptr() {
        return Ok(());
    }
    mem! { (cpu.proc.memory_mut()): *(&item->mEquipped) = false; }
    save_to_game_data(cpu)
}

/// Wrapper that calls `uking::ui::PauseMenuDataMgr::getWeaponsForDpad`
///
/// The returned Vec has at most 20 elements, and is guaranteed to not have nullptrs
pub fn get_weapons_for_dpad(
    cpu: &mut Cpu2,
    target_type: i32,
) -> Result<Vec<Ptr![PouchItem]>, processor::Error> {
    cpu.reset_stack();
    let out_safe_array = cpu.stack_alloc::<[Ptr![PouchItem]; 20]>()?;
    let out_safe_array = Ptr!(<[Ptr![PouchItem] ;20]>(out_safe_array));

    let this_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    reg! { cpu:
        x[0] = this_ptr,
        x[1] = out_safe_array,
        w[2] = target_type
    };
    // TODO --160 (probably inlined)
    cpu.native_jump_to_main_offset(0x0097a7cc)?;
    reg! { cpu: w[0] => let count: i32 };
    if count < 0 || count > 20 {
        log::warn!("get_weapons_for_dpad returned invalid count: {count}");
        return Ok(vec![]);
    }

    let len = count as usize;
    let mut out_vec = Vec::with_capacity(len);
    mem! { (cpu.proc.memory()): let items = *out_safe_array; }
    for item in items.into_iter().take(len) {
        if item.is_nullptr() {
            log::warn!("get_weapons_for_dpad has nullptr in return result");
            continue;
        }
        out_vec.push(item);
    }

    cpu.stack_check::<[Ptr![PouchItem]; 20]>(out_safe_array.to_raw())?;

    Ok(out_vec)
}

/// Call `uking::ui::PauseMenuDataMgr::setEquippedWeaponItemValue`
pub fn set_equipped_weapon_value(
    cpu: &mut Cpu2,
    value: i32,
    item_type: i32,
) -> Result<(), processor::Error> {
    cpu.reset_stack();
    let this_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    reg! { cpu:
        x[0] = this_ptr,
        w[1] = value,
        w[2] = item_type
    };
    // TODO --160
    cpu.native_jump_to_main_offset(0x00971438)
}

/// Call `uking::ui::PauseMenuDataMgr::updateInventoryInfo`
pub fn update_inventory_info(cpu: &mut Cpu2) -> Result<(), processor::Error> {
    cpu.reset_stack();
    let this_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    let items_list_ptr = Ptr!(&this_ptr->mList1);
    reg! { cpu:
        x[0] = this_ptr,
        x[1] = items_list_ptr,
    };
    // TODO --160
    cpu.native_jump_to_main_offset(0x0096c954)
}

/// Call `uking::ui::PauseMenuDataMgr::updateListHeads`, which is re-implemented
/// because it's inlined in most places
pub fn update_list_heads(cpu: &mut Cpu2) -> Result<(), processor::Error> {
    let pmdm = singleton_instance!(pmdm(cpu.proc.memory()))?;
    let mut list_heads: [Ptr![Ptr![PouchItem]]; 7] = [0u64.into(); 7];
    mem! {(cpu.proc.memory()):
        let tab_types = *(&pmdm->mTabsType);
    }

    // consistent with C++ implementation
    #[allow(clippy::needless_range_loop)]
    for i in 0..50 {
        let category_idx = match tab_types[i] {
            // Sword
            0 => 0,
            // Bow/Arrow
            1 | 2 => 1,
            // Shield
            3 => 2,
            // Armor
            4..=6 => 3,
            // Material
            7 => 4,
            // Food
            8 => 5,
            // Key Item
            9 => 6,
            _ => continue,
        };
        if list_heads[category_idx].is_nullptr() {
            list_heads[category_idx] = pmdm.tabs().ith(i as u64);
        }
    }

    mem! {(cpu.proc.memory_mut()):
        *(&pmdm->mListHeads) = list_heads;
    }

    Ok(())
}

/// Call `uking::ui::PauseMenuDataMgr::saveToGameData`
pub fn save_to_game_data(cpu: &mut Cpu2) -> Result<(), processor::Error> {
    cpu.reset_stack();
    let this_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    let items_list_ptr = Ptr!(&this_ptr->mList1);
    reg! { cpu:
        x[0] = this_ptr,
        x[1] = items_list_ptr,
    };
    // TODO --160
    cpu.native_jump_to_main_offset(0x0096f9bc)
}

/// Call `uking::ui::PauseMenuDataMgr::loadFromGameData`
pub fn load_from_game_data(cpu: &mut Cpu2) -> Result<(), processor::Error> {
    cpu.reset_stack();
    let this_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    reg! { cpu:
        x[0] = this_ptr,
    };
    // TODO --160
    cpu.native_jump_to_main_offset(0x0096be24)
}

/// Call `uking::ui::PauseMenuDataMgr::createPlayerEquipment`
pub fn create_player_equipment(cpu: &mut Cpu2) -> Result<(), processor::Error> {
    cpu.reset_stack();
    let this_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    reg! { cpu:
        x[0] = this_ptr,
    };
    // TODO --160
    cpu.native_jump_to_main_offset(0x00971504)
}

/// Call `uking::ui::PauseMenuDataMgr::useItem`
pub fn use_item(
    cpu: &mut Cpu2,
    tab_index: i32,
    slot_index: i32,
    quantity: i32,
) -> Result<(), processor::Error> {
    cpu.reset_stack();
    let this_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    reg! { cpu:
        x[0] = this_ptr,
        w[1] = tab_index,
        w[2] = slot_index,
        w[3] = quantity,
    };
    // TODO --160 (probably inlined)
    cpu.native_jump_to_main_offset(0x00976cc4)
}

/// Call `uking::ui::PauseMenuDataMgr::getEquippedItem`
pub fn get_equipped_item(
    cpu: &mut Cpu2,
    item_type: i32,
) -> Result<Ptr![PouchItem], processor::Error> {
    cpu.reset_stack();
    let this_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    reg! { cpu:
        x[0] = this_ptr,
        w[1] = item_type,
    };
    // TODO --160
    cpu.native_jump_to_main_offset(0x009792f4)?;
    reg! { cpu: x[0] => let item: Ptr![PouchItem] };
    Ok(item)
}

/// Call `uking::ui::PauseMenuDataMgr::breakMasterSword`
pub fn break_master_sword(cpu: &mut Cpu2) -> Result<(), processor::Error> {
    cpu.reset_stack();
    let this_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    reg! { cpu: x[0] = this_ptr, };
    // TODO --160  (probably inlined)
    cpu.native_jump_to_main_offset(0x00972608)
}

/// Call `uking::ui::PauseMenuDataMgr::removeArrow`
pub fn remove_arrow(cpu: &mut Cpu2, arrow_name: &str, count: i32) -> Result<(), processor::Error> {
    cpu.reset_stack();
    let arrow_name_ptr = helper::stack_alloc_string40(cpu, arrow_name)?;
    let this_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    reg! { cpu:
        x[0] = this_ptr,
        x[1] = arrow_name_ptr,
        x[2] = count,
    };
    // TODO --160  (probably inlined)
    cpu.native_jump_to_main_offset(0x00970d84)?;
    cpu.stack_check::<FixedSafeString40>(arrow_name_ptr.to_raw())?;
    Ok(())
}

/// Call `uking::ui::PauseMenuDataMgr::removeItem`. Only removes 1.
///
/// In game, this is only used to remove fairy
pub fn remove_item_by_name(cpu: &mut Cpu2, name: &str) -> Result<(), processor::Error> {
    cpu.reset_stack();
    let name_ptr = helper::stack_alloc_string40(cpu, name)?;
    let this_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    reg! { cpu:
        x[0] = this_ptr,
        x[1] = name_ptr,
    };
    // TODO --160  (probably inlined)
    cpu.native_jump_to_main_offset(0x009704bc)?;
    cpu.stack_check::<FixedSafeString40>(name_ptr.to_raw())?;
    Ok(())
}

pub fn is_weapon_profile(cpu: &mut Cpu2, actor: &str) -> Result<bool, processor::Error> {
    let profile = get_actor_profile(cpu, actor)?;
    Ok(profile.starts_with("Weapon"))
}

/// Call ksys::act::InfoData::getActorProfile
pub fn get_actor_profile(cpu: &mut Cpu2, actor: &str) -> Result<String, processor::Error> {
    cpu.reset_stack();
    let this_ptr = singleton_instance!(info_data(cpu.proc.memory()))?;

    // x1 - char** profile
    let out_profile = Ptr!(<Ptr![u8]>(cpu.stack_alloc::<Ptr![u8]>()?));

    // x2 - char* actor name ptr
    // FixedSafeString40*
    let name_ptr = cpu.stack_alloc::<FixedSafeString40>()?;
    let name_ptr = Ptr!(<FixedSafeString40>(name_ptr));
    name_ptr.construct(cpu.proc.memory_mut())?;
    name_ptr.safe_store(actor, cpu.proc.memory_mut())?;
    // char*
    let name_ptr_cstr = name_ptr.cstr(cpu.proc.memory())?;

    reg! { cpu:
        x[0] = this_ptr,
        x[1] = out_profile,
        x[2] = name_ptr_cstr,
    };

    if cpu.proc.env().is160() {
        cpu.native_jump_to_main_offset(0x01542270)?;
    } else {
        cpu.native_jump_to_main_offset(0x00d301fc)?;
    }
    cpu.stack_check::<FixedSafeString40>(name_ptr.to_raw())?;
    cpu.stack_check::<Ptr![u8]>(out_profile.to_raw())?;

    let profile = out_profile.load(cpu.proc.memory())?;
    let profile = profile.load_utf8_lossy(cpu.proc.memory())?;

    Ok(profile)
}

mod helper {
    use crate::game::SafeString;

    use super::*;

    /// Allocate a FixedSafeString40 on the stack and store the value in it
    pub fn stack_alloc_string40(
        cpu: &mut Cpu2,
        value: &str,
    ) -> Result<Ptr![FixedSafeString40], processor::Error> {
        let ptr = cpu.stack_alloc::<FixedSafeString40>()?;
        let ptr = Ptr!(<FixedSafeString40>(ptr));
        ptr.construct(cpu.proc.memory_mut())?;
        ptr.safe_store(value, cpu.proc.memory_mut())?;
        Ok(ptr)
    }

    /// Allocate a WeaponModifierInfo on the stack and store the value in it
    pub fn stack_alloc_weapon_modifier(
        cpu: &mut Cpu2,
        value: Option<WeaponModifierInfo>,
    ) -> Result<Ptr![WeaponModifierInfo], processor::Error> {
        if let Some(modifier) = value {
            let ptr = cpu.stack_alloc::<WeaponModifierInfo>()?;
            mem! { (cpu.proc.memory_mut()):
                *(<WeaponModifierInfo>(ptr)) = modifier;
            };
            Ok(ptr.into())
        } else {
            Ok(Ptr!(<WeaponModifierInfo>(0)))
        }
    }

    /// Call the `assureTerminationImpl_` virtual function. will trash registers
    /// like a normal call
    pub fn assure_termination(
        cpu: &mut Cpu2,
        ptr: Ptr![SafeString],
    ) -> Result<(), processor::Error> {
        let vtable = Ptr!(&ptr->vtable).load(cpu.proc.memory())?;
        let func_addr = Ptr!(<u64>(vtable + 0x18)).load(cpu.proc.memory())?;
        reg! { cpu: x[0] = ptr };
        cpu.native_jump(func_addr)
    }
}
