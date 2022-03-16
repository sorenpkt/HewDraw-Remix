// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    palutena_teleport_cancel(boma, id, status_kind, situation_kind, cat[0]);
}

pub unsafe fn palutena_teleport_cancel(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, cat1: i32) {
    if status_kind == *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_2 {
        if compare_mask(ControlModule::get_pad_flag(boma), *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_3, false);
        }
    }
    if [*FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_2,
        *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_3].contains(&status_kind) {
        if situation_kind == *SITUATION_KIND_AIR {
            if  !VarModule::is_flag(boma.object(), vars::common::SPECIAL_WALL_JUMP) {
                let touch_right = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32);
                let touch_left = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32);
                let is_turn_dash = compare_mask(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH);
                let is_jump = compare_mask(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP);
                if (touch_right || touch_left) && (is_turn_dash || is_jump) {
                    VarModule::on_flag(boma.object(), vars::common::SPECIAL_WALL_JUMP);
                    VarModule::on_flag(boma.object(), vars::common::UP_SPECIAL_CANCEL);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WALL_JUMP, true);
                }
            }
        }
    }

    // Wall Ride momentum fixes
    let mut wall_ride = Vector3f{x: 1.0, y: 1.0, z: 1.0};
    let touch_right = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32);
    let touch_left = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32);
    let warp_speed = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_hi_wrap_speed_multi"));

    if status_kind == *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_2 {
        if touch_right || touch_left || VarModule::is_flag(boma.object(), vars::common::IS_TELEPORT_WALL_RIDE) {
            VarModule::on_flag(boma.object(), vars::common::IS_TELEPORT_WALL_RIDE);
            if (touch_right && KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) < 0.0) || (touch_left && KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > 0.0) {
                let rise_speed = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                if rise_speed > 0.0 {
                    wall_ride = Vector3f{x: 0.0, y: (warp_speed / rise_speed), z: 1.0};
                }
                else {
                    wall_ride = Vector3f{x: 0.0, y: 1.0, z: 1.0};
                }
                KineticModule::mul_speed(boma, &wall_ride, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
        }
    }
    else if status_kind == *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_3 {
        if touch_right || touch_left {
            if (touch_right && KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) < 0.0) || (touch_left && KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > 0.0) {
                wall_ride = Vector3f{x: 0.0, y: 1.0, z: 1.0};
                KineticModule::mul_speed(boma, &wall_ride, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
        }
    }
    else {
        VarModule::off_flag(boma.object(), vars::common::IS_TELEPORT_WALL_RIDE);
    }
}

#[utils::macros::opff(FIGHTER_KIND_PALUTENA )]
pub fn palutena_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		palutena_frame(fighter)
    }
}

pub unsafe fn palutena_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

#[smashline::weapon_frame_callback]
pub fn reflection_board_callback(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    unsafe { 
        if weapon.kind() != WEAPON_KIND_PALUTENA_REFLECTIONBOARD {
            return
        }
        if weapon.is_status(*WEAPON_PALUTENA_REFLECTIONBOARD_STATUS_KIND_SHOOT) {
            let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
            let palutena = utils::util::get_battle_object_from_id(owner_id);
            let palutena_boma = &mut *(*palutena).module_accessor;
            if AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_ATTACK){
                StatusModule::change_status_request_from_script(weapon.module_accessor, *WEAPON_PALUTENA_REFLECTIONBOARD_STATUS_KIND_BREAK, false);
            }
        }
    }
}