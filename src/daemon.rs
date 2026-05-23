//! Modo daemon: executa a limpeza periodicamente conforme config.
//! Pode ser usado como alternativa ao timer systemd.

use crate::config::KazeConfig;
use crate::plugins::{DockerPlugin, NodePlugin, PacmanPlugin, PluginEngine};
use std::time::Duration;
use tokio::time;
use tracing::info;

pub async fn run(config: KazeConfig) {
    info!("Kaze daemon iniciado. Intervalo: {} min", config.schedule.interval_minutes);

    let mut interval = time::interval(Duration::from_secs(config.schedule.interval_minutes * 60));

    // Primeira execução imediata
    tick(&config).await;

    loop {
        interval.tick().await;
        tick(&config).await;
    }
}

async fn tick(config: &KazeConfig) {
    let plugins: Vec<Box<dyn crate::plugins::CachePlugin>> = vec![
        Box::new(PacmanPlugin::new(config)),
        Box::new(NodePlugin::new(config)),
        Box::new(DockerPlugin::new(config)),
    ];
    let engine = PluginEngine::new(plugins);
    engine.run(false).await;
}