use async_trait::async_trait;
use crate::config::KazeConfig;
use crate::detector;
use crate::executor::safe_exec;
use super::CachePlugin;

pub struct DockerPlugin {
    config: KazeConfig,
    runtime: String, // "docker" ou "podman"
}

impl DockerPlugin {
    pub fn new(config: &KazeConfig) -> Self {
        let runtime = if Self::binary_exists("podman") {
            "podman".into()
        } else if Self::binary_exists("docker") {
            "docker".into()
        } else {
            String::new()
        };
        Self {
            config: config.clone(),
            runtime,
        }
    }

    fn binary_exists(name: &str) -> bool {
        std::process::Command::new("which")
            .arg(name)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    fn installed(&self) -> bool {
        !self.runtime.is_empty()
    }
}

#[async_trait]
impl CachePlugin for DockerPlugin {
    fn name(&self) -> &str { "container" }

    async fn is_active(&self) -> bool {
        detector::is_binary_running("docker").await
            || detector::is_binary_running("containerd").await
            || detector::is_binary_running("podman").await
    }

    async fn estimate_space(&self) -> u64 {
        0
    }

    async fn aggressive_clean(&self) -> Result<(), String> {
        if !self.installed() {
            return Err("Nenhum runtime de container instalado (docker/podman)".into());
        }
        let args = if self.config.plugins.docker.prune_volumes {
            "--volumes"
        } else {
            ""
        };
        safe_exec(&format!("{} system prune -af {}", self.runtime, args), 120)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
    }

    async fn gentle_clean(&self) -> Result<(), String> {
        if !self.installed() {
            return Err("Nenhum runtime de container instalado (docker/podman)".into());
        }
        safe_exec(&format!("{} image prune -f", self.runtime), 60)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
    }

    fn dry_run_commands(&self) -> Vec<String> {
        if !self.installed() {
            return vec!["Nenhum runtime de container instalado".into()];
        }
        vec![format!("{} system prune -af", self.runtime)]
    }
}
