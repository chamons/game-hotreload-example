use anyhow::Result;
use macroquad::prelude::*;

mod render;
use render::*;

mod input;
use input::*;

mod screen;
pub use screen::GameScreen;

mod texture_cache;

#[cfg(feature = "hotreload")]
mod hotreload;

#[cfg(feature = "hotreload")]
use crate::hotreload::binding::{
    example::host::types::{KeyboardInfo, MouseInfo},
    WebAssemblyContext, WebAssemblyInstance,
};

#[cfg(not(feature = "hotreload"))]
pub use game::{
    exports::example::host::game_api::{KeyboardInfo, MouseInfo},
    Instance,
};

use texture_cache::TextureCache;

pub trait RunnableGameInstance {
    fn run_frame(&self, mouse: MouseInfo, key: KeyboardInfo, screen: GameScreen);
}

#[cfg(not(feature = "hotreload"))]
impl RunnableGameInstance for Instance {
    fn run_frame(&self, mouse: MouseInfo, key: KeyboardInfo, screen: GameScreen) {
        Instance::run_frame(self, mouse, key, &screen)
    }
}

async fn run_frame<R: RunnableGameInstance>(instance: &R, screen: GameScreen) {
    let mouse = get_mouse_state();
    let key = get_key_info();
    instance.run_frame(mouse, key, screen);

    next_frame().await
}

#[cfg(not(feature = "hotreload"))]
async fn run(font: Font, mut texture_cache: TextureCache) -> Result<()> {
    let instance = Instance::new();
    let screen = GameScreen::default();
    loop {
        run_frame(&instance, screen.clone()).await;
    }
}

#[cfg(feature = "hotreload")]
async fn run(font: Font, mut texture_cache: TextureCache) -> Result<()> {
    let context = WebAssemblyContext::load()?;
    let mut assembly = WebAssemblyInstance::load(context)?;
    let mut instance = assembly.create_game_instance()?;

    let file_watcher = crate::hotreload::watcher::FileWatcher::new(crate::hotreload::wasm_path()?)?;

    loop {
        if file_watcher.changed() {
            let save_data = instance.save();
            let context = WebAssemblyContext::load()?;
            assembly = WebAssemblyInstance::load(context)?;
            instance = assembly.create_game_instance()?;
            if let Ok(save_data) = save_data {
                let _ = instance.load(save_data);
            }
        }

        let screen = GameScreen::default();

        run_frame(&instance, screen.clone()).await;
    }
}

#[macroquad::main("BasicShapes")]
async fn main() -> Result<()> {
    let font = load_ttf_font_from_bytes(include_bytes!("../../resources/Kreon-Regular.ttf"))
        .expect("Unable to load font");
    let texture_cache = TextureCache::default();

    run(font, texture_cache).await
}
