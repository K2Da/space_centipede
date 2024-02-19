pub use bevy::prelude::*;

pub use constants::*;
pub use util::*;

mod asset;
mod constants;
mod event;
mod gate;
mod head;
mod input;
mod interact;
mod space;
mod tail;
mod ui;
mod util;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins {},
            event::ModPlugin {},
            space::ModPlugin {},
            input::ModPlugin {},
            ui::ModPlugin {},
            head::ModPlugin {},
            tail::ModPlugin {},
            gate::ModPlugin {},
            asset::ModPlugin {},
            interact::ModPlugin {},
        ))
        .add_systems(
            Startup,
            (asset::setup_system, space::setup_system, ui::setup_system),
        )
        .add_systems(PostStartup, head::setup_system)
        .add_systems(First, input::read_input_system)
        .add_systems(
            PreUpdate,
            (
                ui::score_update_system,
                space::zoom_system,
                head::select_movement_system.pipe(void),
                event::game_start_system,
                event::game_over_system.pipe(void),
            ),
        )
        .add_systems(
            Update,
            (
                head::move_head_system.pipe(void),
                tail::move_tail_system
                    .pipe(void)
                    .after(head::move_head_system),
                tail::purged_tail_system.after(tail::move_tail_system),
                space::position_system.after(tail::purged_tail_system),
            ),
        )
        .add_systems(FixedUpdate, ui::fps_update_system)
        // PostUpdateでGlobalTransformに変換されたあと各種チェック
        .add_systems(
            Last,
            (
                interact::head_and_tail_system.pipe(void),
                interact::head_and_gate_system.pipe(void),
                tail::on_game_start,
                tail::on_through_gate.pipe(void),
                tail::on_miss.pipe(void),
                head::on_game_start,
                head::on_game_over,
                gate::spawn_gate_system.pipe(void),
                gate::on_game_start,
                ui::on_game_start,
                ui::on_through_gate,
            ),
        )
        .run();
}
