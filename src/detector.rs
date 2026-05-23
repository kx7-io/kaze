//! Lê informações de processos via /proc para determinar quais aplicativos estão ativos.
//! Ignora processos zumbis (state Z) e processos em estado D com timeout.

use std::time::Duration;
use tokio::time::timeout;

/// Verifica se um binário específico está em execução.
pub async fn is_binary_running(binary_name: &str) -> bool {
    let procs = list_processes().await;
    procs.iter().any(|p| {
        p.exe.as_ref().map_or(false, |exe| exe.contains(binary_name))
            || p.cmdline.contains(binary_name)
    })
}

/// Informações simplificadas de um processo.
pub struct ProcInfo {
    pub pid: i32,
    pub exe: Option<String>,
    pub cmdline: String,
    pub state: char,
}

/// Lista todos os processos do sistema, ignorando zumbis.
pub async fn list_processes() -> Vec<ProcInfo> {
    let mut procs = Vec::new();
    // all_processes() retorna Result<Vec<Result<Process, ProcError>>, ProcError>
    if let Ok(all) = procfs::process::all_processes() {
        // `all` é Vec<Result<Process, ProcError>>
        for process_result in all {
            // Desempacota o Result interno
            if let Ok(process) = process_result {
                let pid = process.pid;
                // Timeout para evitar travar em processos em estado D
                let info = timeout(Duration::from_secs(2), async {
                    let stat = process.stat().ok()?;
                    if stat.state == 'Z' {
                        return None; // ignora zumbis
                    }
                    let exe = process.exe().ok().map(|p| p.display().to_string());
                    let cmdline = process.cmdline().ok().map(|c| c.join(" ")).unwrap_or_default();
                    Some(ProcInfo {
                        pid,
                        exe,
                        cmdline,
                        state: stat.state,
                    })
                })
                .await
                .ok()
                .flatten();

                if let Some(info) = info {
                    procs.push(info);
                }
            }
        }
    }
    procs
}

/// Verifica se o lock do pacman está ativo (evita limpeza durante updates).
pub fn is_pacman_locked() -> bool {
    std::path::Path::new("/var/lib/pacman/db.lck").exists()
}
