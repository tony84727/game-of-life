use amethyst::{
    prelude::*,
    renderer::{types::DefaultBackend, RenderFlat2D, RenderToWindow, RenderingBundle},
    utils::application_root_dir,
    LoggerConfig, Result,
};

struct GameState;

impl SimpleState for GameState {}

fn main() -> Result<()> {
    amethyst::start_logger(LoggerConfig::default());
    let app_dir = application_root_dir()?;
    let asset_dir = app_dir.join("assets");
    let config_dir = app_dir.join("config");
    let game_data = GameDataBuilder::default().with_bundle(
        RenderingBundle::<DefaultBackend>::new()
            .with_plugin(
                RenderToWindow::from_config_path(config_dir.join("display.ron"))?
                    .with_clear([0.0, 0.0, 0.0, 0.0]),
            )
            .with_plugin(RenderFlat2D::default()),
    )?;
    let mut game = Application::new(asset_dir, GameState, game_data)?;
    game.run();
    Ok(())
}
