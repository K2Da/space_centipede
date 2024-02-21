use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_ehttp::prelude::*;

pub struct ModPlugin {}

impl Plugin for ModPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HttpPlugin)
            .add_systems(Update, handle_response)
            .add_systems(
                Update,
                send_request.run_if(on_timer(std::time::Duration::from_secs(1))),
            );
    }
}

fn send_request(mut commands: Commands) {
    let req = Request::get("https://api.ipify.org?format=json");
    commands.spawn(HttpRequest(req));
}

fn handle_response(mut requests: EventReader<RequestCompleted>) {
    for request in &mut requests.read() {
        match &**request {
            Ok(response) => info!("response: {:?}", response.text()),
            Err(e) => info!("response error: {:?}", e),
        }
    }
}
