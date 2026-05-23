//! Carrega a configuração do Kaze a partir de ~/.config/kaze/config.toml.
//! Valores ausentes herdam dos defaults definidos aqui.

use serde::Deserialize;
use std::{fs, path::PathBuf};

#[derive(Deserialize, Clone)]
pub struct KazeConfig {
    #[serde(default)]
    pub limits: LimitsConfig,
    #[serde(default)]
    pub plugins: PluginsConfig,
    #[serde(default)]
    pub schedule: ScheduleConfig,
}

#[derive(Deserialize, Clone)]
pub struct LimitsConfig {
    #[serde(default = "default_max_cache_age_days")]
    pub max_cache_age_days: u32,
    #[serde(default = "default_min_free_disk_gb")]
    pub min_free_disk_gb: u32,
    #[serde(default = "default_max_ram_percent")]
    pub max_ram_percent: u8,
}

#[derive(Deserialize, Clone)]
pub struct PluginsConfig {
    #[serde(default)]
    pub pacman: PacmanPluginConfig,
    #[serde(default)]
    pub node: NodePluginConfig,
    #[serde(default)]
    pub docker: DockerPluginConfig,
}

#[derive(Deserialize, Clone)]
pub struct PacmanPluginConfig {
    #[serde(default = "default_keep_versions")]
    pub keep_versions: u32,
    #[serde(default = "default_keep_versions_gentle")]
    pub keep_versions_gentle: u32,
}

#[derive(Deserialize, Clone)]
pub struct NodePluginConfig {
    #[serde(default = "default_node_managers")]
    pub managers: Vec<String>,
    #[serde(default = "default_true")]
    pub enabled: bool,
}

#[derive(Deserialize, Clone)]
pub struct DockerPluginConfig {
    #[serde(default)]
    pub prune_volumes: bool,
    #[serde(default = "default_true")]
    pub enabled: bool,
}

#[derive(Deserialize, Clone)]
pub struct ScheduleConfig {
    #[serde(default = "default_interval_minutes")]
    pub interval_minutes: u64,
}

// Defaults
fn default_max_cache_age_days() -> u32 { 7 }
fn default_min_free_disk_gb() -> u32 { 5 }
fn default_max_ram_percent() -> u8 { 85 }
fn default_keep_versions() -> u32 { 2 }
fn default_keep_versions_gentle() -> u32 { 3 }
fn default_node_managers() -> Vec<String> {
    vec!["npm".into(), "pnpm".into(), "yarn".into()]
}
fn default_interval_minutes() -> u64 { 30 }
fn default_true() -> bool { true }

impl Default for KazeConfig {
    fn default() -> Self {
        Self {
            limits: LimitsConfig::default(),
            plugins: PluginsConfig::default(),
            schedule: ScheduleConfig::default(),
        }
    }
}

impl Default for LimitsConfig {
    fn default() -> Self {
        Self {
            max_cache_age_days: default_max_cache_age_days(),
            min_free_disk_gb: default_min_free_disk_gb(),
            max_ram_percent: default_max_ram_percent(),
        }
    }
}

impl Default for PluginsConfig {
    fn default() -> Self {
        Self {
            pacman: PacmanPluginConfig::default(),
            node: NodePluginConfig::default(),
            docker: DockerPluginConfig::default(),
        }
    }
}

impl Default for PacmanPluginConfig {
    fn default() -> Self {
        Self {
            keep_versions: default_keep_versions(),
            keep_versions_gentle: default_keep_versions_gentle(),
        }
    }
}

impl Default for NodePluginConfig {
    fn default() -> Self {
        Self {
            managers: default_node_managers(),
            enabled: true,
        }
    }
}

impl Default for DockerPluginConfig {
    fn default() -> Self {
        Self {
            prune_volumes: false,
            enabled: true,
        }
    }
}

impl Default for ScheduleConfig {
    fn default() -> Self {
        Self {
            interval_minutes: default_interval_minutes(),
        }
    }
}

/// Carrega config do arquivo, mesclando com defaults.
pub fn load_config() -> KazeConfig {
    let config_path = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("kaze")
        .join("config.toml");

    match fs::read_to_string(&config_path) {
        Ok(content) => toml::from_str(&content).unwrap_or_default(),
        Err(_) => KazeConfig::default(),
    }
}