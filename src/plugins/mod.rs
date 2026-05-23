//! Define a trait CachePlugin e a engine que orquestra a execução.

use async_trait::async_trait;
use tracing::{info, error};

mod pacman;
mod node;
mod docker;

pub use pacman::PacmanPlugin;
pub use node::NodePlugin;
pub use docker::DockerPlugin;

#[async_trait]
pub trait CachePlugin: Send + Sync {
    /// Nome do plugin (ex: "pacman").
    fn name(&self) -> &str;

    /// Verifica se o aplicativo associado está rodando.
    async fn is_active(&self) -> bool;

    /// Estima espaço em bytes que pode ser liberado.
    async fn estimate_space(&self) -> u64;

    /// Limpeza agressiva (app inativo).
    async fn aggressive_clean(&self) -> Result<(), String>;

    /// Limpeza suave (app ativo).
    async fn gentle_clean(&self) -> Result<(), String>;

    /// Comandos que seriam executados (dry-run).
    fn dry_run_commands(&self) -> Vec<String>;
}

/// Orquestra a execução de todos os plugins.
pub struct PluginEngine {
    plugins: Vec<Box<dyn CachePlugin>>,
}

impl PluginEngine {
    pub fn new(plugins: Vec<Box<dyn CachePlugin>>) -> Self {
        Self { plugins }
    }

    /// Executa a limpeza em todos os plugins.
    pub async fn run(&self, dry_run: bool) {
        for plugin in &self.plugins {
            let name = plugin.name();
            let active = plugin.is_active().await;
            info!("Plugin {}: {}", name, if active { "ativo" } else { "inativo" });

            if dry_run {
                for cmd in plugin.dry_run_commands() {
                    println!("[DRY-RUN {}] {}", name, cmd);
                }
                continue;
            }

            let result = if active {
                plugin.gentle_clean().await
            } else {
                plugin.aggressive_clean().await
            };

            match result {
                Ok(()) => info!("{}: limpeza concluída", name),
                Err(e) => {
                    error!("{}: erro - {}", name, e);
                    crate::notifier::notify(
                        &format!("Kaze - Erro no {}", name),
                        &e,
                        "critical",
                    );
                }
            }
        }
    }

    /// Exibe status de todos os plugins.
    pub async fn status(&self) {
        for plugin in &self.plugins {
            let name = plugin.name();
            let active = plugin.is_active().await;
            let space = plugin.estimate_space().await;
            let mb = space as f64 / 1_048_576.0;
            println!("{}: {} | Cache: {:.2} MB", name, if active { "ATIVO" } else { "inativo" }, mb);
        }
    }
}