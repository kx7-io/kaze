use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::EnvFilter;
use std::path::PathBuf;

pub fn init() {
    // Tenta criar diretório de logs, mas não falha se não conseguir
    let log_dir = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("kaze")
        .join("logs");
    std::fs::create_dir_all(&log_dir).ok();

    // Configura o subscriber para escrever tanto no arquivo quanto no console
    let file_appender = tracing_appender::rolling::daily(&log_dir, "kaze.log");

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_writer(std::io::stderr) // Garante saída no terminal
        .with_span_events(FmtSpan::CLOSE)
        .init();
}
