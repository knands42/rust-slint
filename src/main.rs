use sample_project::{
    app::App,
    backend::{run_migrations},
    config::Config,
};

#[tokio::main]
async fn main() {
    env_logger::init();
    log_panics::init();

    let config = Config::new().expect("Failed to initialize config");
    log::info!("Config loaded from {}", config.config_file.display());

    // Run migrations before anything else touches the DB
    if let Err(e) = run_migrations(&config.db_path.to_string_lossy().as_ref()).await {
        eprintln!("Database migration failed: {e}");
        std::process::exit(1);
    }

    #[cfg(debug_assertions)]
    {
        use sample_project::backend::run_seeds;

        log::info!("Seeding");

        match run_seeds(&config.db_path.to_string_lossy().as_ref()).await {
            Ok(ids) => log::info!("Seeded {} samples", ids.len()),
            Err(err) => {
                eprintln!("Could not seed demo world: {err}");
                std::process::exit(1);
            }
        }

        log::info!("Demo mode: debug build — exiting after seed.");
    }

    let app = App::new();
    log::info!("Starting Sample Project");

    if let Err(err) = app.run() {
        eprintln!("Application error: {err}");
        std::process::exit(1);
    }
}
