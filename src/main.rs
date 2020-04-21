use amethyst::{
    prelude::*,
    renderer::{types::DefaultBackend, RenderFlat2D, RenderToWindow, RenderingBundle},
    utils::application_root_dir,
    window::DisplayConfig,
    LoggerConfig, Result,
};

struct GameState;

impl SimpleState for GameState {}

fn main() -> Result<()> {
    amethyst::start_logger(LoggerConfig::default());
    let app_dir = application_root_dir()?;
    let asset_dir = app_dir.join("assets");
    let game_data = GameDataBuilder::default().with_bundle(
        RenderingBundle::<DefaultBackend>::new()
            .with_plugin(RenderToWindow::from_config(DisplayConfig {
                title: String::from("Game of Life"),
                fullscreen: None,
                dimensions: Some((500, 500)),
                min_dimensions: None,
                max_dimensions: None,
                visibility: true,
                icon: None,
                loaded_icon: None,
                always_on_top: false,
                decorations: true,
                maximized: false,
                multitouch: false,
                resizable: true,
                transparent: false,
            }))
            .with_plugin(RenderFlat2D::default()),
    )?;
    let mut game = Application::new(asset_dir, GameState, game_data)?;
    game.run();
    Ok(())
}
