//! Testes básicos para o módulo detector.
//! Executados com `cargo test`.

#[cfg(test)]
mod tests {
    use kaze::detector;

    #[tokio::test]
    async fn test_list_processes_includes_current() {
        let procs = detector::list_processes().await;
        // Deve pelo menos conter o próprio processo de teste
        assert!(!procs.is_empty());
    }

    #[tokio::test]
    async fn test_systemd_running() {
        // systemd (PID 1) geralmente está rodando em sistemas Linux
        let procs = detector::list_processes().await;
        let systemd = procs.iter().find(|p| p.pid == 1);
        assert!(systemd.is_some());
    }
}