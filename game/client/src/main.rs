use bevy::prelude::*;
use ui::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_plugins(GameUIPlugin);

    app.run();
}
