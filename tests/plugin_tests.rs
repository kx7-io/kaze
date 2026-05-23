//! Testes para verificar que os plugins retornam nome e não paniquem.

#[cfg(test)]
mod tests {
    use kaze::config::KazeConfig;
    use kaze::plugins::{CachePlugin, PacmanPlugin};

    #[tokio::test]
    async fn test_pacman_plugin_name() {
        let config = KazeConfig::default();
        let plugin = PacmanPlugin::new(&config);
        assert_eq!(plugin.name(), "pacman");
    }

    #[tokio::test]
    async fn test_pacman_plugin_dry_run() {
        let config = KazeConfig::default();
        let plugin = PacmanPlugin::new(&config);
        let commands = plugin.dry_run_commands();
        assert!(!commands.is_empty());
    }
}