use amethyst::{
    assets::{PrefabData, PrefabLoaderSystemDesc},
    core::Transform,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        camera::{Orthographic, Projection},
        types::DefaultBackend,
        Camera, RenderFlat2D, RenderToWindow, RenderingBundle,
    },
    utils::application_root_dir,
    window::ScreenDimensions,
    LoggerConfig, Result,
};

mod cell;

struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        GameState::initialize_camera(&mut data.world);
    }
}

impl GameState {
    fn initialize_camera(world: &mut World) {
        let (width, height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };

        // Translate the camera to Z coordinate 10.0, and it looks back toward
        // the origin with depth 20.0
        let mut transform = Transform::default();
        transform.set_translation_xyz(0., height, 10.);
        let mut camera = Camera::standard_3d(width, height);
        camera.set_projection(Projection::Orthographic(Orthographic::new(
            0.0, width, 0.0, height, 0.0, 20.,
        )));

        let camera = world.create_entity().with(transform).with(camera).build();
    }
}

fn main() -> Result<()> {
    amethyst::start_logger(LoggerConfig::default());
    let app_dir = application_root_dir()?;
    let asset_dir = app_dir.join("assets");
    let config_dir = app_dir.join("config");
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(config_dir.join("display.ron"))?
                        .with_clear([0.0, 0.0, 0.0, 0.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with(cell::CellSystem::new(25, 25), "cell", &[]);
    let mut game = Application::new(asset_dir, GameState, game_data)?;
    game.run();
    Ok(())
}
