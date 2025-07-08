#[cfg(target_os = "macos")]
use std::sync::mpsc::channel;

// use std::time::Duration;

use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{
            Extent3d,
            TextureDimension,
            TextureFormat,
        },
    },
    // time::Stopwatch,
};

// TODO: wasm support
use nokhwa::{
    pixel_format::RgbFormat,
    utils::{
        CameraIndex,
        RequestedFormat,
        RequestedFormatType,
    },
    Camera,
};



#[cfg(target_os = "macos")]
fn nokhwa_initialize_blocking() -> Result<(), &'static str> {
    let (tx, rx) = channel();

    nokhwa::nokhwa_initialize(move |success| {
        let _ = tx.send(success);
    });

    match rx.recv() {
        Ok(true) => Ok(()),
        Ok(false) => Err("user denied camera permission"),
        Err(_) => Err("initialization channel closed unexpectedly"),
    }
}


struct NokhwaResource {
    camera: Camera,
    resolution: Extent3d,
}


#[derive(Resource, Clone, Debug, Reflect)]
pub struct WebcamStream {
    pub frame: Handle<Image>,
}


pub struct BevyWebcamPlugin {
    pub camera_index: CameraIndex,
    pub requested_format_type: RequestedFormatType,
}

impl Default for BevyWebcamPlugin {
    fn default() -> Self {
        Self {
            camera_index: CameraIndex::Index(0),
            requested_format_type: RequestedFormatType::AbsoluteHighestFrameRate,
        }
    }
}

impl Plugin for BevyWebcamPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(target_os = "macos")]
        nokhwa_initialize_blocking().expect("failed to initialise nokhwa");

        let requested = RequestedFormat::new::<RgbFormat>(
            self.requested_format_type
        );
        let mut camera = Camera::new(
                self.camera_index.clone(),
                requested,
            )
            .expect("failed to create camera");

        camera
            .open_stream()
            .expect("failed to open camera stream");

        let framerate = camera.frame_rate();
        let resolution = camera.resolution();

        info!("expected camera framerate: {framerate}");
        info!("expected camera resolution: {}x{}", resolution.width_x, resolution.height_y);

        let nokhwa_resource = NokhwaResource {
            camera,
            resolution: Extent3d {
                width: resolution.width_x,
                height: resolution.height_y,
                depth_or_array_layers: 1,
            },
        };
        app.insert_non_send_resource(nokhwa_resource);

        app.insert_resource(WebcamStream {
            frame: Handle::default(),

        });
        app.register_type::<WebcamStream>();

        app.add_systems(PreStartup, initial_frame_setup);
        app.add_systems(Update, upload_frame);
    }
}


fn initial_frame_setup(
    mut images: ResMut<Assets<Image>>,
    nokhwa_resource: NonSend<NokhwaResource>,
    mut nokhwa_stream: ResMut<WebcamStream>,
) {
    nokhwa_stream.frame = images.add(Image::new_fill(
        nokhwa_resource.resolution,
        TextureDimension::D2,
        &[0u8; 4],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    ));
}


fn rgb_to_rgba(rgb: &[u8]) -> Vec<u8> {
    let mut rgba = Vec::with_capacity(rgb.len() / 3 * 4);
    for chunk in rgb.chunks_exact(3) {
        rgba.extend_from_slice(&[chunk[0], chunk[1], chunk[2], 0xff]);
    }
    rgba
}

// TODO: move frame decoding to a separate thread
fn upload_frame(
    nokhwa_stream: Res<WebcamStream>,
    mut images: ResMut<Assets<Image>>,
    mut nokhwa_resource: NonSendMut<NokhwaResource>,
    // time: Res<Time>,
    // mut last_frame: Local<Stopwatch>,
) {
    // last_frame.tick(time.delta());

    // let capture_period = Duration::from_secs_f32(1.0 / nokhwa_resource.camera.frame_rate() as f32);
    // if last_frame.elapsed() < capture_period {
    //     info!("waiting until next frame, capture period: {:?}", capture_period);
    //     // return;
    // }

    match nokhwa_resource.camera.frame() {
        Ok(frame) => {
            let rgba = frame.decode_image::<RgbFormat>()
                .expect("failed to decode camera frame");

            let (w, h) = rgba.dimensions();
            if nokhwa_resource.resolution.width != w || nokhwa_resource.resolution.height != h {
                warn!("camera resolution changed from {}x{} to {}x{}",
                    nokhwa_resource.resolution.width,
                    nokhwa_resource.resolution.height,
                    w, h
                );
                nokhwa_resource.resolution = Extent3d {
                    width: w,
                    height: h,
                    depth_or_array_layers: 1,
                };
            }

            let pixels = rgba.into_raw();
            let rgba_pixels = rgb_to_rgba(&pixels);

            images.get_mut(&nokhwa_stream.frame)
                .expect("failed to get image handle")
                .data = rgba_pixels.into();

            // last_frame.reset();
        }
        Err(e) => {
            error!("failed to get camera frame: {}", e);
        }
    }
}
