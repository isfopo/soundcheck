mod app;
mod audio;
mod config;
mod constants;
mod error;
mod smoothing;
mod state;
mod ui;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = app::App::new()?;
    app.run().await?;
    Ok(())
}


