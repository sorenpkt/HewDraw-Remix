// status imports
use super::*;
use globals::*;

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuard_exitStatus_common)]
unsafe fn sub_ftStatusUniqProcessGuard_exitStatus_common(fighter: &mut L2CFighterCommon) {
    ShieldModule::set_status(
        fighter.module_accessor,
        *FIGHTER_SHIELD_KIND_GUARD,
        app::ShieldStatus(*SHIELD_STATUS_NONE),
        0
    );
    ShieldModule::set_hit_stop_mul(fighter.module_accessor, 1.0);
}

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuard_exitStatus)]
unsafe fn ftStatusUniqProcessGuard_exitStatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    sub_ftStatusUniqProcessGuard_exitStatus_common(fighter);
    L2CValue::I32(0)
}

pub fn install() {
    skyline::install_hooks!(
        ftStatusUniqProcessGuard_exitStatus,
        sub_ftStatusUniqProcessGuard_exitStatus_common
    );
}
