# bevy_billboard

![](assets/screenshot.png?raw=true)

Adds a billboard render pipeline to bevy.

Currently only supports in-world spherical billboards

## Usage

Add `bevy_plugin_billboard` as a dependency to `Cargo.toml`

```
[dependencies]
bevy = "0.1.3"
bevy_billboard = { git = "https://github.com/atbentley/bevy_plugin_billboard" }
```

Then register the `BillboardPlugin` plugin

```rust
use bevy::prelude::*;
use bevy_billboard::{BillboardComponents, BillboardPlugin};

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(BillboardPlugin)
        .run();
}
```

And finally spawn in a `BillboardComponents`:

```rust
fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(BillboardComponents {
          material: materials.add(Color::rgb(0.0, 0.8, 0.0).into()),
          ..Default::default()
        });
}
```

Run `examples` for a more complete example

## Next

- [ ] Support PBR lights
- [ ] Option to draw billboards over other in world meshes meshes
- [ ] Support spherical billboards
- [ ] Use correct scaling (currently billboards are ~2.5x smaller than they should be)
