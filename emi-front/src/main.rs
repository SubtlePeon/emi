use std::env;
use tracing::info;

mod app;
mod game_go;
mod menu;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> color_eyre::eyre::Result<()> {
    // Set up tracing_subscriber
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // Set up environmental variables for anyhow
    if env::var("RUST_BACKTRACE").is_err() {
        env::set_var("RUST_BACKTRACE", "1");
    }

    color_eyre::install()?;

    info!("Running app");

    let options = eframe::NativeOptions::default();
    eframe::run_native("Test", options, Box::new(|_cc| Box::new(app::State::new())))
        .expect("Unexpected error");

    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn main() {
    info!("Running app");

    let web_options = eframe::WebOptions::default();

    // Getting some `WebRunner` does not exist errors...
    // wasm_bindgen_futures::spawn_local(async {
    //     eframe::WebRunner::new()
    //         .start(
    //             "canvas_id",
    //             web_options,
    //             Box::new(|_| Box::new(app::State::new())),
    //         )
    //         .await
    //         .expect("Failed to start eframe");
    // });

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "the_canvas_id",
            web_options,
            Box::new(|_| Box::new(app::State::new())),
        )
        .await
        .expect("Failed to start eframe");
    })
}
