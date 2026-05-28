import blessed from 'blessed';
import { getKazeStatus, isKazeRunning } from './kaze-bridge';

const screen = blessed.screen({
  smartCSR: true,
  title: '🌬️ Kaze Dashboard'
});

// ── Status do Kaze (canto superior direito) ──
const statusBox = blessed.box({
  top: 0,
  right: 0,
  width: 'shrink',
  height: 1,
  padding: { left: 1, right: 1 },
  style: {
    fg: 'black',
    bg: 'red'
  }
});

// ── Plugins (esquerda) ──
const pluginsBox = blessed.box({
  top: 2,
  left: 0,
  width: '50%',
  height: '70%-2',
  label: ' 🔌 Plugins ',
  border: 'line',
  style: {
    border: { fg: 'cyan' }
  }
});

// ── Última limpeza (direita) ──
const logBox = blessed.box({
  top: 2,
  right: 0,
  width: '50%',
  height: '70%-2',
  label: ' 📋 Últimas Limpezas ',
  border: 'line',
  style: {
    border: { fg: 'green' }
  },
  scrollable: true,
  alwaysScroll: true
});

// ── Sistema (inferior) ──
const sysBox = blessed.box({
  bottom: 0,
  left: 0,
  width: '100%',
  height: '30%',
  label: ' 💻 Sistema ',
  border: 'line',
  style: {
    border: { fg: 'yellow' }
  }
});

screen.append(statusBox);
screen.append(pluginsBox);
screen.append(logBox);
screen.append(sysBox);

async function updateDashboard() {
  // Atualiza o STATUS
  const running = isKazeRunning();
  if (running) {
    statusBox.setContent(' KAZE: ON ');
    statusBox.style.bg = 'green';
  } else {
    statusBox.setContent(' KAZE: OFF ');
    statusBox.style.bg = 'red';
  }

  // Atualiza os plugins
  const status = getKazeStatus();
  if (status.length > 0) {
    const content = status.map(s =>
      `${s.active ? '🟢' : '⚫'} ${s.name.padEnd(15)} ${s.cache}`
    ).join('\n');
    pluginsBox.setContent(content);
  } else {
    pluginsBox.setContent('❌ Kaze não encontrado.');
  }

  // Atualiza o log
  try {
    const { execSync } = require('child_process');
    const logOutput = execSync(
      'journalctl --user -u kaze-cleaner.service --no-pager -n 6 2>/dev/null | grep -E "INFO|Finished|Starting" || echo "Nenhum log."',
      { encoding: 'utf-8' }
    );
    logBox.setContent(logOutput);
  } catch {
    logBox.setContent('Nenhum log.');
  }

  // Atualiza o sistema
  const { execSync } = require('child_process');
  const ram = execSync('free -h | grep Mem | awk \'{print $3 "/" $2}\'').toString().trim();
  const disk = execSync('df -h / | awk \'NR==2 {print $3 "/" $2 " (" $5 ")"}\'').toString().trim();
  const uptime = execSync('uptime -p').toString().trim();
  sysBox.setContent(
    `🧠 RAM: ${ram}\n💾 Disco: ${disk}\n⏱️ Uptime: ${uptime}`
  );

  screen.render();
}

updateDashboard();
setInterval(updateDashboard, 3000);
screen.key(['q', 'C-c'], () => process.exit(0));
screen.render();
