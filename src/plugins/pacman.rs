use async_trait::async_trait;
use crate::config::KazeConfig;
use crate::detector;
use crate::executor::safe_exec;
use super::CachePlugin;


pub struct PacmanPlugin {
    config: KazeConfig,
}

impl PacmanPlugin {
    pub fn new(config: &KazeConfig) -> Self {
        Self { config: config.clone() }
    }
}

#[async_trait]
impl CachePlugin for PacmanPlugin {
    fn name(&self) -> &str { "pacman" }

    async fn is_active(&self) -> bool {
        detector::is_binary_running("pacman").await
            || detector::is_binary_running("yay").await
            || detector::is_binary_running("paru").await
            || detector::is_pacman_locked()
    }

    async fn estimate_space(&self) -> u64 {
        // paccache -dvk1 para estimar sem remover
        match safe_exec("paccache -dvk1", 10).await {
            Ok(output) => {
                // Extrai total de bytes simplificadamente
                if let Some(line) = output.lines().last() {
                    parse_size_line(line)
                } else { 0 }
            }
            Err(_) => 0,
        }
    }

    async fn aggressive_clean(&self) -> Result<(), String> {
        let keep = self.config.plugins.pacman.keep_versions;
        safe_exec(&format!("paccache -rk{}", keep), 120)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
    }

    async fn gentle_clean(&self) -> Result<(), String> {
        let keep = self.config.plugins.pacman.keep_versions_gentle;
        safe_exec(&format!("paccache -rk{}", keep), 120)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
    }

    fn dry_run_commands(&self) -> Vec<String> {
        let keep = self.config.plugins.pacman.keep_versions;
        vec![format!("paccache -rk{}", keep)]
    }
}

/// Tenta extrair um número em bytes de uma linha do paccache.
fn parse_size_line(line: &str) -> u64 {
    // Exemplo: "total: 123.45 MB"
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 2 {
        if let Ok(val) = parts[parts.len()-2].parse::<f64>() {
            let unit = parts[parts.len()-1].to_lowercase();
            match unit.as_str() {
                "gb" => (val * 1_073_741_824.0) as u64,
                "mb" => (val * 1_048_576.0) as u64,
                "kb" => (val * 1024.0) as u64,
                _ => val as u64,
            }
        } else { 0 }
    } else { 0 }
}