//! Main application logic and orchestration

use crate::audio;
use crate::config::Config;
use cpal::traits::StreamTrait;
use crate::error::{AppError, AppResult};
use crate::state::{AppState, SharedState};
use crate::ui;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::Duration;

/// Main application struct
pub struct App {
    config: Config,
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
}

impl App {
    /// Initialize the application with configuration
    pub fn new() -> AppResult<Self> {
        let config = Config::from_args().map_err(|e| AppError::Config(e.to_string()))?;

        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(App { config, terminal })
    }

    /// Run the main application loop
    pub async fn run(mut self) -> AppResult<()> {
        // Setup audio
        let (device, audio_config) = audio::setup_audio_device(self.config.device_name.clone())?;
        let device_name = audio_config.device_name;

        // Create shared state
        let shared_state = SharedState::new();
        let (current_db, smoothed_db, display_db, threshold_reached) = shared_state.audio_refs();

        // Create app state
        let mut app_state = AppState::new(device_name, self.config.threshold_db);

        // Build audio stream
        let audio_callback = audio::create_audio_callback(
            current_db,
            smoothed_db,
            display_db,
            threshold_reached,
            self.config.linear_threshold(),
        );

        let config = cpal::StreamConfig {
            channels: audio_config.channels,
            sample_rate: cpal::SampleRate(audio_config.sample_rate),
            buffer_size: crate::constants::audio::BUFFER_SIZE,
        };

        let stream = audio::build_audio_stream(&device, &config, audio_callback)?;
        stream.play()?;

        // Main UI loop
        let mut interval = tokio::time::interval(Duration::from_millis(crate::constants::ui::UPDATE_INTERVAL_MS));
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    // Update state from shared values
                    app_state.update_from_audio(&shared_state.current_db, &shared_state.smoothed_db, &shared_state.display_db, &shared_state.threshold_reached);

                    // Render UI
                    self.terminal.draw(|f| {
                        let ui_state = ui::UiState {
                            device_name: app_state.device_name.clone(),
                            current_db: app_state.current_db,
                            display_db: app_state.display_db,
                            threshold_db: app_state.threshold_db,
                            status: app_state.status.clone(),
                        };
                        ui::render_ui(f, &ui_state);
                    })?;

                    // Check if threshold reached
                    if app_state.threshold_reached {
                        break;
                    }
                }
                _ = tokio::signal::ctrl_c() => {
                    break;
                }
            }
        }

        // Cleanup
        drop(stream);
        self.cleanup()?;

        Ok(())
    }

    /// Clean up terminal state
    fn cleanup(mut self) -> AppResult<()> {
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}