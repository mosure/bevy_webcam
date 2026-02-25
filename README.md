# bevy_webcam 📷

[![GitHub License](https://img.shields.io/github/license/mosure/bevy_webcam)](https://raw.githubusercontent.com/mosure/bevy_webcam/main/LICENSE-MIT)
[![crates.io](https://img.shields.io/crates/v/bevy_webcam.svg)](https://crates.io/crates/bevy_webcam)

bevy camera input, using the nokhwa crate


## usage

```rust
use bevy::prelude::*;
use bevy_webcam::{BevyWebcamPlugin, WebcamStream};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BevyWebcamPlugin::default()))
        .add_systems(Startup, setup_camera)
        .add_systems(Update, update_ui)
        .run();
}

fn setup_camera(mut commands: Commands, stream: Res<WebcamStream>) {
    commands.spawn(Camera2d);

    commands.spawn((
        ImageNode {
            image: stream.frame.clone(),
            ..Default::default()
        },
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..Default::default()
        },
    ));
}

fn update_ui(stream: Res<WebcamStream>, mut query: Query<&mut ImageNode>) {
    *query.iter_mut().next().unwrap() = ImageNode {
        image: stream.frame.clone(),
        ..Default::default()
    };
}
```


## features

- [x] native camera capture via `nokhwa`'s native backends
- [x] threaded frame decoding on native targets, so the Bevy `Update` stage stays responsive
- [x] wasm32 (browser) capture via the DOM `MediaStreamTrackProcessor` feeding pixels into the exported `frame_input` binding

## platform notes

- **Native:** frames are decoded on a dedicated worker thread and sent to the main Bevy world through a channel before being uploaded to the GPU.
- **Wasm:** `www/index.html` acquires the webcam stream with `getUserMedia`, processes frames with `MediaStreamTrackProcessor`, and forwards RGBA pixels into the wasm module via `frame_input`. The Bevy plugin simply consumes those frames each `Update`, so there is no blocking `nokhwa` path on the browser.
- **Camera selection on web:** the browser decides which device backs the stream the user grants; the `CameraIndex` setting currently applies to native builds only.
