// Everything Tauri-specific in the page. renderer.js is the Electron app's renderer (changed only in
// the About box and to await confirm(), which the dialog plugin makes async) and talks to the Rust
// backend only through the same `window.api` Electron's preload.js gave it.
const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

window.api = {
  status: () => invoke('status'),
  about: () => invoke('about'),
  settings: () => invoke('settings'),
  setSetting: (key, value) => invoke('set_setting', { key, value }),
  defaultFolder: () => invoke('default_folder'),
  paste: () => invoke('paste'),
  pickFolder: current => invoke('pick_folder', { current }),
  showFile: file => invoke('show_file', { file }),
  info: url => invoke('info', { url }),
  download: ({ id, url, quality, folder }) => invoke('download', { id, url, quality, folder }),
  cancel: id => invoke('cancel', { id }),
  pause: id => invoke('pause', { id }),
  deleteFiles: id => invoke('delete_files', { id }),
  onStatus: fn => listen('status', e => fn(e.payload)),
  onProgress: fn => listen('progress', e => fn(e.payload)),
};

/* ---------- window buttons (the window is frameless, so the page draws them) ---------- */

const appWindow = window.__TAURI__.window.getCurrentWindow();
const maxBtn = document.getElementById('win-max');
document.getElementById('win-min').onclick = () => appWindow.minimize();
document.getElementById('win-close').onclick = () => appWindow.close();
maxBtn.onclick = () => appWindow.toggleMaximize();
async function syncMaximized() {
  const maximized = await appWindow.isMaximized();
  maxBtn.classList.toggle('maximized', maximized);
  maxBtn.title = maximized ? 'Restore' : 'Maximize';
  maxBtn.setAttribute('aria-label', maxBtn.title);
}
appWindow.onResized(syncMaximized);
syncMaximized();

// WebView2 has a browser right-click menu (Back, Reload, Print…) that Electron didn't; keep it for text fields only.
document.addEventListener('contextmenu', e => {
  if (!e.target.closest('input')) e.preventDefault();
});
