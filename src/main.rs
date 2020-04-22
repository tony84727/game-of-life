use amethyst::{
    assets::PrefabLoaderSystemDesc,
    core::{Transform, TransformBundle},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{rendy::mesh::PosTex, Camera, RenderToWindow, RenderingBundle},
    utils::application_root_dir,
    window::ScreenDimensions,
    LoggerConfig, Result,
};
use amethyst_rendy::formats::GraphicsPrefab;
use amethyst_rendy::{RenderDebugLines, RenderFlat3D, RenderSkybox};

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

        let mut camera = Camera::standard_3d(500 as f32, 500 as f32);

        let _camera = world.create_entity().with(transform).with(camera).build();
    }
}

fn main() -> Result<()> {
    amethyst::start_logger(LoggerConfig::default());
    let app_dir = application_root_dir()?;
    let asset_dir = app_dir.join("assets");
    let config_dir = app_dir.join("config");
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
            PrefabLoaderSystemDesc::<GraphicsPrefab<Vec<PosTex>>>::default(),
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
                .with_plugin(RenderSkybox::default())
                .with_plugin(RenderFlat3D::default())
                .with_plugin(RenderDebugLines::default()),
        )?
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with_system_desc(cell::CellSystemDesc, "cell", &[])
        .with_system_desc(cell::CellDisplaySystemDesc, "cell_display", &[]);
    let mut game = Application::new(asset_dir, GameState, game_data)?;
    game.run();
    Ok(())
}
