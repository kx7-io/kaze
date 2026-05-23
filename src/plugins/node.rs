use async_trait::async_trait;
use crate::config::KazeConfig;
use crate::detector;
use crate::executor::safe_exec;
use super::CachePlugin;

pub struct NodePlugin {
    config: KazeConfig,
}

impl NodePlugin {
    pub fn new(config: &KazeConfig) -> Self {
        Self { config: config.clone() }
    }

    fn binary_exists(name: &str) -> bool {
        std::process::Command::new("which")
            .arg(name)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
}

#[async_trait]
impl CachePlugin for NodePlugin {
    fn name(&self) -> &str { "node" }

    async fn is_active(&self) -> bool {
        detector::is_binary_running("node").await
            || detector::is_binary_running("npm").await
            || detector::is_binary_running("pnpm").await
            || detector::is_binary_running("yarn").await
    }

    async fn estimate_space(&self) -> u64 {
        match safe_exec("du -sb ~/.npm/_cacache 2>/dev/null | cut -f1", 5).await {
            Ok(output) => output.trim().parse().unwrap_or(0),
            Err(_) => 0,
        }
    }

    async fn aggressive_clean(&self) -> Result<(), String> {
        for manager in &self.config.plugins.node.managers {
            if !Self::binary_exists(manager) {
                continue;
            }
            match manager.as_str() {
                "npm" => { safe_exec("npm cache clean --force", 60).await.map_err(|e| e.to_string())?; }
                "pnpm" => { let _ = safe_exec("pnpm store prune", 60).await; }
                "yarn" => { let _ = safe_exec("yarn cache clean --all", 60).await; }
                _ => {}
            }
        }
        Ok(())
    }

    async fn gentle_clean(&self) -> Result<(), String> {
        if !Self::binary_exists("npm") {
            return Err("npm não está instalado".into());
        }
        safe_exec("npm cache verify", 60).await.map(|_| ()).map_err(|e| e.to_string())
    }

    fn dry_run_commands(&self) -> Vec<String> {
        self.config.plugins.node.managers.iter().map(|m| {
            match m.as_str() {
                "npm" => "npm cache clean --force".into(),
                "pnpm" => "pnpm store prune".into(),
                "yarn" => "yarn cache clean --all".into(),
                _ => format!("{} cache clean", m),
            }
        }).collect()
    }
}
