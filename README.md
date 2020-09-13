# bevy_plugin_billboard

![](assets/screenshot.png?raw=true)

Adds a billboard render pipeline to bevy. Billboards can be coloured, textured, and either drawn with correct z-ordering, or drawn on top of other in world meshes.

## Usage

Add `bevy_plugin_billboard` as a dependency to `Cargo.toml`

```
[dependencies]
bevy = "0.1.3"
bevy_plugin_billboard = { git = "https://github.com/atbentley/bevy_plugin_billboard" }
```

Then register the `BillboardPlugin` plugin

```rust
use bevy::prelude::*;
use bevy_plugin_billboard::{BillboardComponents, BillboardPlugin, BillboardMaterial};

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
    mut billboards: ResMut<Assets<BillboardMaterial>>,
) {
    let billboard = billboard.add(BillboardMaterial {
        albedo: Color::rgb(0.0, 0.8, 0.0),
        ..Default::default()
    });
    commands
        .spawn(BillboardComponents {
            billboard,
            ..Default::default()
        });
}
```

See `examples` for a more complete examples.

## Next

- [ ] Support PBR lights (currently all 4 bind groups are being used: camera, window aspect, transform and billboard material)
- [ ] Support cylindrical billboards
- [ ] Use correct scaling (currently billboards are ~2.5x smaller than they should be)
