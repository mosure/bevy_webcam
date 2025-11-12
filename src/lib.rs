#[cfg(target_os = "macos")]
use std::sync::mpsc::channel;

use bevy::{
    prelude::*,
    asset::RenderAssetUsages,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    // time::Stopwatch,
};

#[cfg(not(target_arch = "wasm32"))]
use flume::{Receiver, Sender};

#[cfg(not(target_arch = "wasm32"))]
use std::{thread, time::Duration};

#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;

use nokhwa::utils::{CameraIndex, RequestedFormatType};

#[cfg(not(target_arch = "wasm32"))]
use nokhwa::{Camera, pixel_format::RgbFormat, utils::RequestedFormat};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

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

#[cfg(not(target_arch = "wasm32"))]
struct FramePayload {
    pixels: Vec<u8>,
    extent: Extent3d,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Resource)]
struct NativeWebcam {
    receiver: Receiver<FramePayload>,
    _worker: FrameWorker,
    is_srgb: bool,
    resolution: Extent3d,
}

#[cfg(not(target_arch = "wasm32"))]
struct FrameWorker {
    handle: Option<thread::JoinHandle<()>>,
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for FrameWorker {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            if let Err(err) = handle.join() {
                warn!("capture worker exited with error: {:?}", err);
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[derive(Resource)]
struct WasmWebcam {
    is_srgb: bool,
    resolution: Extent3d,
}

#[cfg(target_arch = "wasm32")]
struct WasmFrame {
    pixels: Vec<u8>,
    width: u32,
    height: u32,
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static PENDING_FRAME: RefCell<Option<WasmFrame>> = RefCell::new(None);
}

#[cfg(target_arch = "wasm32")]
fn take_wasm_frame() -> Option<WasmFrame> {
    PENDING_FRAME.with(|cell| cell.borrow_mut().take())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn frame_input(pixel_data: &[u8], width: u32, height: u32) {
    let pixels = pixel_data.to_vec();
    PENDING_FRAME.with(|cell| {
        *cell.borrow_mut() = Some(WasmFrame {
            pixels,
            width,
            height,
        });
    });
}

#[derive(Resource, Clone, Debug, Reflect)]
pub struct WebcamStream {
    pub frame: Handle<Image>,
}

pub struct BevyWebcamPlugin {
    pub camera_index: CameraIndex,
    pub requested_format_type: RequestedFormatType,
    pub is_srgb: bool,
}

impl Default for BevyWebcamPlugin {
    fn default() -> Self {
        Self {
            camera_index: CameraIndex::Index(0),
            requested_format_type: RequestedFormatType::AbsoluteHighestFrameRate,
            is_srgb: true,
        }
    }
}

impl Plugin for BevyWebcamPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(target_os = "macos")]
        nokhwa_initialize_blocking().expect("failed to initialise nokhwa");

        #[cfg(not(target_arch = "wasm32"))]
        {
            let webcam = spawn_native_webcam(self);
            app.insert_resource(webcam);
        }

        #[cfg(target_arch = "wasm32")]
        {
            app.insert_resource(WasmWebcam {
                is_srgb: self.is_srgb,
                resolution: Extent3d {
                    width: 1,
                    height: 1,
                    depth_or_array_layers: 1,
                },
            });
        }

        app.insert_resource(WebcamStream {
            frame: Handle::default(),
        });
        app.register_type::<WebcamStream>();

        #[cfg(not(target_arch = "wasm32"))]
        app.add_systems(PreStartup, initial_frame_setup);

        #[cfg(target_arch = "wasm32")]
        app.add_systems(PreStartup, wasm_initial_frame_setup);

        #[cfg(not(target_arch = "wasm32"))]
        app.add_systems(Update, upload_frame_native);

        #[cfg(target_arch = "wasm32")]
        app.add_systems(Update, upload_frame_wasm);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn initial_frame_setup(
    mut images: ResMut<Assets<Image>>,
    webcam: Res<NativeWebcam>,
    mut stream: ResMut<WebcamStream>,
) {
    let format = frame_texture_format(webcam.is_srgb);
    stream.frame = images.add(Image::new_fill(
        webcam.resolution,
        TextureDimension::D2,
        &[0u8; 4],
        format,
        RenderAssetUsages::default(),
    ));
}

#[cfg(target_arch = "wasm32")]
fn wasm_initial_frame_setup(
    mut images: ResMut<Assets<Image>>,
    mut stream: ResMut<WebcamStream>,
    webcam: Res<WasmWebcam>,
) {
    let format = frame_texture_format(webcam.is_srgb);
    stream.frame = images.add(Image::new_fill(
        webcam.resolution,
        TextureDimension::D2,
        &[0u8; 4],
        format,
        RenderAssetUsages::default(),
    ));
}

#[cfg(not(target_arch = "wasm32"))]
fn upload_frame_native(
    stream: Res<WebcamStream>,
    mut images: ResMut<Assets<Image>>,
    mut webcam: ResMut<NativeWebcam>,
) {
    let mut latest_frame = None;
    while let Ok(frame) = webcam.receiver.try_recv() {
        latest_frame = Some(frame);
    }

    let Some(frame) = latest_frame else {
        return;
    };

    let Some(image) = images.get_mut(&stream.frame) else {
        warn!("webcam texture handle is missing");
        return;
    };

    let format = frame_texture_format(webcam.is_srgb);
    if image.texture_descriptor.size != frame.extent {
        warn!(
            "camera resolution changed from {}x{} to {}x{}",
            image.texture_descriptor.size.width,
            image.texture_descriptor.size.height,
            frame.extent.width,
            frame.extent.height
        );
    }

    write_frame_to_image(image, frame.extent, frame.pixels, format);
    webcam.resolution = frame.extent;
}

#[cfg(target_arch = "wasm32")]
fn upload_frame_wasm(
    stream: Res<WebcamStream>,
    mut images: ResMut<Assets<Image>>,
    mut webcam: ResMut<WasmWebcam>,
) {
    if stream.frame == Handle::default() {
        return;
    }

    let Some(frame) = take_wasm_frame() else {
        return;
    };

    let Some(image) = images.get_mut(&stream.frame) else {
        warn!("webcam texture handle is missing");
        return;
    };

    let extent = Extent3d {
        width: frame.width,
        height: frame.height,
        depth_or_array_layers: 1,
    };
    if image.texture_descriptor.size != extent {
        warn!(
            "camera resolution changed from {}x{} to {}x{}",
            image.texture_descriptor.size.width,
            image.texture_descriptor.size.height,
            extent.width,
            extent.height
        );
    }
    write_frame_to_image(
        image,
        extent,
        frame.pixels,
        frame_texture_format(webcam.is_srgb),
    );
    webcam.resolution = extent;
}

fn frame_texture_format(is_srgb: bool) -> TextureFormat {
    if is_srgb {
        TextureFormat::Rgba8UnormSrgb
    } else {
        TextureFormat::Rgba8Unorm
    }
}

fn write_frame_to_image(
    image: &mut Image,
    extent: Extent3d,
    pixels: Vec<u8>,
    format: TextureFormat,
) {
    if image.texture_descriptor.size != extent {
        image.resize(extent);
    }
    if image.texture_descriptor.format != format {
        image.texture_descriptor.format = format;
    }
    image.data = Some(pixels);
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(not(target_arch = "wasm32"))]
fn rgb_to_rgba(rgb: &[u8]) -> Vec<u8> {
    let mut rgba = Vec::with_capacity(rgb.len() / 3 * 4);
    for chunk in rgb.chunks_exact(3) {
        rgba.extend_from_slice(&[chunk[0], chunk[1], chunk[2], 0xff]);
    }
    rgba
}

#[cfg(not(target_arch = "wasm32"))]
fn spawn_native_webcam(config: &BevyWebcamPlugin) -> NativeWebcam {
    let requested = RequestedFormat::new::<RgbFormat>(config.requested_format_type);
    let mut camera =
        Camera::new(config.camera_index.clone(), requested).expect("failed to create camera");

    camera.open_stream().expect("failed to open camera stream");

    let framerate = camera.frame_rate();
    let resolution = camera.resolution();
    info!("expected camera framerate: {framerate}");
    info!(
        "expected camera resolution: {}x{}",
        resolution.width_x, resolution.height_y
    );

    let extent = Extent3d {
        width: resolution.width_x,
        height: resolution.height_y,
        depth_or_array_layers: 1,
    };

    let (sender, receiver) = flume::bounded(2);
    let worker_camera = camera;
    let handle = thread::Builder::new()
        .name("bevy_webcam_capture".to_string())
        .spawn(move || capture_frames(worker_camera, sender))
        .expect("failed to spawn capture worker thread");

    NativeWebcam {
        receiver,
        _worker: FrameWorker {
            handle: Some(handle),
        },
        is_srgb: config.is_srgb,
        resolution: extent,
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn capture_frames(mut camera: Camera, sender: Sender<FramePayload>) {
    loop {
        match camera.frame() {
            Ok(frame) => match frame.decode_image::<RgbFormat>() {
                Ok(image) => {
                    let (width, height) = image.dimensions();
                    let extent = Extent3d {
                        width,
                        height,
                        depth_or_array_layers: 1,
                    };
                    let rgba_pixels = rgb_to_rgba(&image.into_raw());
                    if sender
                        .send(FramePayload {
                            pixels: rgba_pixels,
                            extent,
                        })
                        .is_err()
                    {
                        break;
                    }
                }
                Err(err) => error!("failed to decode camera frame: {err}"),
            },
            Err(err) => {
                error!("failed to get camera frame: {err}");
                thread::sleep(Duration::from_millis(16));
            }
        }
    }
}
