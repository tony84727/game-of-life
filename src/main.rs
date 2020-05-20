use amethyst::{
    assets::PrefabLoaderSystemDesc,
    controls::{FlyControlBundle, FlyControlTag},
    core::{Transform, TransformBundle},
    input::{InputBundle, StringBindings},
    LoggerConfig,
    prelude::*,
    renderer::{Camera, RenderFlat3D, RenderingBundle, RenderToWindow, rendy::mesh::PosTex},
    Result,
    utils::application_root_dir, window::ScreenDimensions,
};

use crate::camera::ZoomCamera;
use crate::cell::CellPrefabData;

mod camera;
mod cell;
mod debug;

struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        GameState::initialize_camera(&mut data.world);
        data.world.insert(cell::AutomataControl::default());
    }
}

impl GameState {
    fn initialize_camera(world: &mut World) {
        let (width, height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };
        // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
        let mut transform = Transform::default();

        let mut camera = Camera::standard_2d(width, height);

        let _camera = world
            .create_entity()
            .with(transform)
            .with(camera)
            .with(ZoomCamera {
                dimension: (width, height),
            })
            .build();
    }
}

fn main() -> Result<()> {
    amethyst::start_logger(LoggerConfig::default());
    let app_dir = application_root_dir()?;
    let asset_dir = app_dir.join("assets");
    let config_dir = app_dir.join("config");
    let input_binding = config_dir.join("input.ron");
    #[cfg(target_os = "macos")]
    let rendering_bundle = {
        use amethyst::renderer::rendy::metal::Backend;
        RenderingBundle::<Backend>::new()
    };
    #[cfg(not(target_os = "macos"))]
    let rendering_bundle = {
        use amethyst::renderer::rendy::vulkan::Backend;
        RenderingBundle::<Backend>::new()
    };
    let game_data = GameDataBuilder::default()
        .with_system_desc(
            PrefabLoaderSystemDesc::<CellPrefabData>::default(),
            "prefab_loader",
            &[],
        )
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            rendering_bundle
                .with_plugin(
                    RenderToWindow::from_config_path(config_dir.join("display.ron"))?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat3D::default()),
        )?
        .with_bundle(InputBundle::<StringBindings>::new().with_bindings_from_file(input_binding)?)?
        .with_system_desc(cell::CellSystemDesc, "cell", &[])
        .with_system_desc(
            cell::CellDisplaySystemDesc::load(config_dir.join("cell_display.ron"))?,
            "cell_display",
            &[],
        )
        .with(camera::CameraZoomSystem::new(), "camera_zoom", &[]);
    // .with_bundle(
    //     FlyControlBundle::<StringBindings>::new(
    //         Some(String::from("horizontal")),
    //         Some(String::from("vertical")),
    //         Some(String::from("forward")),
    //     )
    //     .with_speed(20.0),
    // )?;
    let mut game = Application::new(asset_dir, GameState, game_data)?;
    game.run();
    Ok(())
}
