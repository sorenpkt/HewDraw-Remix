use super::*;

unsafe extern "C" fn game_detach(agent: &mut L2CAgentBase) {
    
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_detach", game_detach);
}