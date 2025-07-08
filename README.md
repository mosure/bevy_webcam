# bevy_webcam 📷

[![GitHub License](https://img.shields.io/github/license/mosure/bevy_webcam)](https://raw.githubusercontent.com/mosure/bevy_webcam/main/LICENSE-MIT)
[![crates.io](https://img.shields.io/crates/v/bevy_webcam.svg)](https://crates.io/crates/bevy_webcam)

bevy camera input, using the nokhwa crate


## usage

```rust
app.add_plugins((
    DefaultPlugins,
    BevyWebcamPlugin::default(),
));
app.add_systems(
    Update,
    setup_ui,
);

// ...

fn setup_ui(
    mut commands: Commands,
    stream: Res<WebcamStream>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        ImageNode {
            image: stream.frame.clone(),
            ..default()
        },
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
    ));
}
```


## features

- [x] native camera
- [ ] threaded camera
- [ ] wasm camera
