<h1 align="center">🌬️ Kaze</h1>

<p align="center">
  <img alt="Rust" src="https://img.shields.io/badge/-Rust-000000?style=flat&logo=rust&logoColor=white">
  <img alt="Arch Linux" src="https://img.shields.io/badge/-Arch_Linux-1793D1?style=flat&logo=archlinux&logoColor=white">
  <img alt="Hyprland" src="https://img.shields.io/badge/-Hyprland-58E1FF?style=flat&logo=hyprland&logoColor=black">
  <br>
  <strong>Limpeza inteligente de sistema para Arch Linux + Hyprland.</strong><br>
  Detecta o que você está usando e limpa o resto — sem pressa, sem quebrar nada.
</p>

---

## 🧠 O que é?

Kaze (風, "vento" em japonês) é um daemon que monitora processos ativos e realiza limpeza de caches de forma adaptativa:

- **Apps inativos** → limpeza agressiva (ex: `paccache -rk2`)
- **Apps ativos** → limpeza suave (ex: `npm cache verify`)
- **Nunca remove arquivos manualmente** — apenas comandos nativos de cada gerenciador
- **Totalmente automático** via timer systemd

---

## ⚙️ Stack

- **Rust** (binário nativo, ~3 MB em release, sem runtime)
- **Systemd** (timer de usuário para execução periódica)
- **Hyprland** (integração via `exec-once`)

---

## 🚀 Uso

```bash
# Status dos plugins
kaze status

# Simulação (dry-run)
kaze clean --dry-run

# Limpeza real
kaze clean
