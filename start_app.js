import { exec } from 'child_process';
const tauriProcess = exec('npm run tauri dev');
tauriProcess.stdout.on('data', (data) => console.log(data));
tauriProcess.stderr.on('data', (data) => console.error(data));
