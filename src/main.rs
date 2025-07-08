use bevy::{
    prelude::*,
    app::AppExit,
    color::palettes::css::GOLD,
    diagnostic::{
        DiagnosticsStore,
        FrameTimeDiagnosticsPlugin,
    },
};

use bevy_webcam::{
    BevyWebcamPlugin,
    WebcamStream,
};


fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        BevyWebcamPlugin::default(),
    ));

    app.add_systems(Startup, setup_ui);

    app.add_systems(Update, press_esc_close);

    app.add_plugins(FrameTimeDiagnosticsPlugin::default());
    app.add_systems(Startup, fps_display_setup);
    app.add_systems(Update, fps_update_system);

    app.run();
}


fn setup_ui(
    mut commands: Commands,
    stream: Res<WebcamStream>,
) {
    commands.spawn(Camera2d);

    commands.spawn(
            Node {
                display: Display::Grid,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                grid_template_columns: RepeatedGridTrack::flex(1, 1.0),
                grid_template_rows: RepeatedGridTrack::flex(1, 1.0),
                ..default()
            }
        )
        .with_children(|builder| {
            builder.spawn(
                ImageNode {
                    image: stream.frame.clone(),
                    ..default()
                }
            );
        });
}


pub fn press_esc_close(
    keys: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>
) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}


fn fps_display_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Text("fps: ".to_string()),
        TextFont {
            font: asset_server.load("fonts/Caveat-Bold.ttf"),
            font_size: 60.0,
            ..Default::default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        },
        ZIndex(2),
    )).with_child((
        FpsText,
        TextColor(Color::Srgba(GOLD)),
        TextFont {
            font: asset_server.load("fonts/Caveat-Bold.ttf"),
            font_size: 60.0,
            ..Default::default()
        },
        TextSpan::default(),
    ));
}

#[derive(Component)]
struct FpsText;

fn fps_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut TextSpan, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                **text = format!("{value:.2}");
            }
        }
    }
}
