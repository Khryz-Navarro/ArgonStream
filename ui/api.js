// Everything Tauri-specific in the page. renderer.js talks to the Rust backend only through `window.api`,
// defined here; this file also runs the window buttons and keeps WebView2's browser behaviour out of the app.
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

// WebView2 has a browser right-click menu (Back, Reload, Print…); keep it for text fields only.
document.addEventListener('contextmenu', e => {
  if (!e.target.closest('input')) e.preventDefault();
});

// WebView2 also keeps browser shortcuts (Tauri can't switch them off). A reload would make the
// page forget every running download while yt-dlp keeps going; print, find and history navigation mean nothing here.
const BROWSER_KEYS = new Set(['F3', 'F5', 'BrowserBack', 'BrowserForward', 'BrowserRefresh']);
document.addEventListener('keydown', e => {
  const ctrl = (e.ctrlKey || e.metaKey) && ['r', 'p', 'f', 'g'].includes(e.key.toLowerCase());
  const history = e.altKey && (e.key === 'ArrowLeft' || e.key === 'ArrowRight');
  if (BROWSER_KEYS.has(e.key) || ctrl || history) e.preventDefault();
});

// main.rs turns off Tauri's own drop handler so a link dragged from the browser can be dropped into the link box.
// Anything dropped elsewhere, and any file, would otherwise replace the app page: refuse those.
for (const type of ['dragover', 'drop']) {
  document.addEventListener(type, e => {
    if (e.dataTransfer?.types.includes('Files') || !e.target.closest('input')) e.preventDefault();
  });
}
