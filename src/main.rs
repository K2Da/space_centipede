pub use bevy::prelude::*;

use crate::event::*;
pub use constants::*;
pub use stage::MyStage;
pub use util::*;

mod constants;
mod event;
mod gate;
mod head;
mod input;
mod interaction;
mod space;
mod stage;
mod tail;
mod ui;
mod util;

fn main() {
    let mut app = App::build();

    app.insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.add_plugin(event::ModPlugin {})
        .add_plugin(stage::ModPlugin {})
        .add_plugin(space::ModPlugin {})
        .add_plugin(input::ModPlugin {})
        .add_plugin(ui::ModPlugin {})
        .add_plugin(head::ModPlugin {})
        .add_plugin(tail::ModPlugin {})
        .add_plugin(gate::ModPlugin {})
        .add_plugin(interaction::ModPlugin {});

    app.run();
}
