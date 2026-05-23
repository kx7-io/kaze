//! Executa comandos de sistema de forma segura:
//! - Sempre com ionice -c 3 (prioridade de I/O ociosa)
//! - Timeout por comando
//! - Captura stdout/stderr e retorna Result

use std::time::Duration;
use tokio::process::Command as AsyncCommand;
use tokio::time::timeout;

#[derive(thiserror::Error, Debug)]
pub enum ExecError {
    #[error("Comando excedeu timeout")]
    Timeout,
    #[error("Falha ao executar: {0}")]
    Io(#[from] std::io::Error),
    #[error("Comando falhou com status: {0}")]
    Status(i32),
}

/// Executa um comando com ionice e timeout.
pub async fn safe_exec(command: &str, timeout_secs: u64) -> Result<String, ExecError> {
    let full_cmd = format!("ionice -c 3 {}", command);
    let parts: Vec<&str> = full_cmd.split_whitespace().collect();

    let result = timeout(
        Duration::from_secs(timeout_secs),
        async {
            if parts.is_empty() {
                return Err(ExecError::Io(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "comando vazio",
                )));
            }

            let child = AsyncCommand::new(parts[0])
                .args(&parts[1..])
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()?;

            let output = child.wait_with_output().await?;
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                let _stderr = String::from_utf8_lossy(&output.stderr);
                Err(ExecError::Status(output.status.code().unwrap_or(1)))
            }
        }
    )
    .await;

    match result {
        Ok(inner) => inner,
        Err(_) => Err(ExecError::Timeout),
    }
}