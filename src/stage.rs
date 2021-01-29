use crate::*;

pub const FIRST: &str = bevy::prelude::stage::FIRST;
pub const PRE_UPDATE: &str = bevy::prelude::stage::PRE_UPDATE;
pub const UPDATE: &str = bevy::prelude::stage::UPDATE;
pub const POST_UPDATE: &str = bevy::prelude::stage::POST_UPDATE;
pub const LAST: &str = bevy::prelude::stage::LAST;
pub const SEND_EVENT: &str = "SEND_EVENT";
pub const RECEIVE_EVENT: &str = "RECEIVE_EVENT";
pub const PRE_RENDER: &str = "PRE_RENDER";

pub struct ModPlugin;

impl Plugin for ModPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_stage_after(stage::LAST, SEND_EVENT, SystemStage::serial())
            .add_stage_after(SEND_EVENT, RECEIVE_EVENT, SystemStage::serial())
            .add_stage_after(RECEIVE_EVENT, PRE_RENDER, SystemStage::serial());
    }
}
