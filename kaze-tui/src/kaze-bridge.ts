import { execSync } from 'child_process';

export interface KazeStatus {
  name: string;
  active: boolean;
  cache: string;
}

export function getKazeStatus(): KazeStatus[] {
  try {
    const output = execSync('/home/nyx7/Documentos/kaze/target/release/kaze status', { encoding: 'utf-8' }).toString();
    const lines = output.trim().split('\n').filter(line => line.includes(':'));
    return lines.map(line => {
      const parts = line.split(':').map(s => s.trim());
      return {
        name: parts[0],
        active: parts[1] === 'ATIVO',
        cache: parts[2] || '---'
      };
    });
  } catch (error) {
    return [];
  }
}

export function isKazeRunning(): boolean {
  try {
    const output = execSync('systemctl --user is-active kaze-cleaner.timer', { encoding: 'utf-8' }).toString().trim();
    return output === 'active';
  } catch (error) {
    return false;
  }
}
