use crate::*;
use bevy::prelude::CoreStage;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum MyStage {
    SendEvent,
    ReceiveEvent,
    PreRender,
}

pub struct ModPlugin;

impl Plugin for ModPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_stage_after(CoreStage::Last, MyStage::SendEvent, SystemStage::parallel())
            .add_stage_after(
                MyStage::SendEvent,
                MyStage::ReceiveEvent,
                SystemStage::parallel(),
            )
            .add_stage_after(
                MyStage::ReceiveEvent,
                MyStage::PreRender,
                SystemStage::parallel(),
            );
    }
}
