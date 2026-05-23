//! Ponto de entrada do Kaze.
//! Processa argumentos da CLI e dispara o modo de limpeza ou o daemon.

mod config;
mod daemon;
mod detector;
mod executor;
mod logger;
mod notifier;
mod plugins;

use clap::{Parser, Subcommand};
use config::load_config;
use plugins::{DockerPlugin, NodePlugin, PacmanPlugin, PluginEngine};

#[derive(Parser)]
#[command(name = "kaze", about = "🧹 Limpeza inteligente de sistema", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Executa a limpeza (agressiva para apps inativos, suave para ativos)
    Clean {
        /// Apenas mostra o que seria feito, sem executar
        #[arg(long)]
        dry_run: bool,
    },
    /// Exibe o status dos caches e processos ativos
    Status,
    /// Inicia o daemon que executa limpeza periodicamente
    Daemon,
}

#[tokio::main]
async fn main() {
    logger::init();
    let cli = Cli::parse();

    let config = load_config();

    // Registra os plugins na ordem desejada
    let plugins: Vec<Box<dyn plugins::CachePlugin>> = vec![
        Box::new(PacmanPlugin::new(&config)),
        Box::new(NodePlugin::new(&config)),
        Box::new(DockerPlugin::new(&config)),
    ];

    let engine = PluginEngine::new(plugins);

    match cli.command {
        Commands::Clean { dry_run } => {
            engine.run(dry_run).await;
        }
        Commands::Status => {
            engine.status().await;
        }
        Commands::Daemon => {
            daemon::run(config).await;
        }
    }
}