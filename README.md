# YT Downloader (Rust)

The Rust version of [YT Downloader](https://github.com/Khryz-Navarro/youtube-downloader), a small
Windows app for downloading YouTube videos on top of **[yt-dlp](https://github.com/yt-dlp/yt-dlp)**.
It does the same things and looks the same, but runs on **[Tauri 2](https://tauri.app/)**: a Rust
backend replaces Electron's main process, and the page is drawn by Windows' built-in WebView2 instead
of a bundled Chromium. The installer is about 23 MB, most of it ffmpeg (the Electron one is about 140 MB).

Paste a link, pick a quality preset (or your own resolution and file type), and download. yt-dlp
installs itself on first launch and keeps itself up to date.

> **Status:** early versions (0.x) for Windows 10/11. Download the installer from
> [Releases](https://github.com/Khryz-Navarro/youtube-downloader-rust/releases), or run from source.
> The UI follows the [Figma design](https://www.figma.com/design/aj74kwrRhnwNIhxQSibcxE) and is
> shared with the Electron version.

## Contents

- [Features](#features)
- [Quality options](#quality-options)
- [Requirements](#requirements)
- [Install](#install)
- [Run from source](#run-from-source)
- [Using the app](#using-the-app)
- [Where things are stored](#where-things-are-stored)
- [How it's built](#how-its-built)
- [Development](#development)
- [Troubleshooting](#troubleshooting)
- [Known limitations](#known-limitations)
- [Credits and legal](#credits-and-legal)

## Features

- **Five quality presets:** 4K Best (MKV), 4K, 1080p and 720p MP4 (H.264 preferred), and Default MP4.
- **Custom quality:** any maximum resolution from 8K down to 144p, as MP4, MKV or WebM. Every option includes the best available audio.
- **Video preview** as soon as you paste a link: thumbnail, title, channel, length and the highest resolution available.
- **Two tabs:** *New download* (link box, preview, save folder and Download on the left, quality on the right) and *Downloads* (the full-width list). The Downloads tab shows a count that turns orange while something is downloading.
- **The look, "Contact Sheet":** the app as a darkroom contact sheet. The preview sits in a film frame with its edge code (video ID, resolution, length), and each download's progress is a row of sprocket holes that develop orange, ending green when done or red where it failed. The light theme is the light table.
- **Download list** with live progress, size, speed and time left. Each item can be **paused and resumed**, canceled, retried, shown in File Explorer, or **deleted** with its files.
- **Several downloads at once**, each in its own yt-dlp process.
- **yt-dlp is managed for you:** downloaded on first launch and updated with `yt-dlp -U` every time the app starts. The status bar shows its version and update state, with "Developed by Khryz" in the middle.
- **ffmpeg is bundled**, so the separate video and audio streams are merged automatically.
- **Settings** (the cog, top right): **Theme** (Light, Dark, or Desktop, which follows Windows) and **Text and page size** (100%, 125%, 150% or 175%). Both are remembered.
- **About:** the `v0.1.0` button shows the app version and the yt-dlp, ffmpeg and Tauri versions.
- **Remembers** your last quality choice and download folder.
- **Keyboard friendly:** <kbd>Enter</kbd> in the link box starts the download; every control is reachable with <kbd>Tab</kbd>.

## Quality options

YouTube stores the high-quality video and the audio as separate streams. Each preset tells yt-dlp
which video stream to pick, adds the best audio stream, and merges them with ffmpeg. The arguments are
exactly the Electron version's.

| Preset | What you get | File | yt-dlp options |
|---|---|---|---|
| **4K Best** | Highest quality up to 2160p, in whatever codec YouTube has (often AV1 or VP9) | `.mkv` | `-f "bv*+ba/b" -S res:2160 --merge-output-format mkv` |
| **4K** | Best MP4 up to 2160p | `.mp4` | `-f "bv*[ext=mp4]+ba[ext=m4a]/b[ext=mp4]/bv*+ba/b" --merge-output-format mp4 --remux-video mp4 -S res:2160,vcodec:h264` |
| **1080p** | Best MP4 up to 1080p, H.264 when available | `.mp4` | same as 4K with `-S res:1080,vcodec:h264` |
| **720p** | Best MP4 up to 720p, H.264 when available | `.mp4` | same as 4K with `-S res:720,vcodec:h264` |
| **Default** | yt-dlp's own MP4 preset: H.264 + AAC first | `.mp4` | `-t mp4` |
| **Custom** · MP4 / MKV / WebM | The same rules up to the resolution you pick | `.mp4` / `.mkv` / `.webm` | `-S res:<N>` (+ `vcodec:h264` for MP4); WebM uses `-f "bv*[ext=webm]+ba[ext=webm]/b[ext=webm]/bv*+ba/b" --merge-output-format webm/mkv` |

Files are named `Title [videoId] 1080p.ext` (yt-dlp template `%(title)s [%(id)s]%(height& {}p|)s.%(ext)s`),
so different qualities of one video never collide. If a video doesn't have the exact resolution, the
preset picks the best one below the cap. WebM can't hold H.264, so a video with no WebM streams is
saved as `.mkv`.

## Requirements

- **Windows 10 or 11** with the WebView2 runtime (built into Windows 11 and current Windows 10).
- **Internet access** on first launch, to download yt-dlp.
- To build: **[Rust](https://rustup.rs/)** (stable, MSVC toolchain) and the **Visual Studio C++ build tools**
  ("Desktop development with C++" workload, which includes the Windows SDK).

You don't need to install yt-dlp, ffmpeg or Python yourself.

## Install

1. Open [Releases](https://github.com/Khryz-Navarro/youtube-downloader-rust/releases) and download
   `YT-Downloader-Setup-<version>.exe` from the newest release.
2. Run it. The installer isn't code-signed yet, so Windows may show **"Windows protected your PC"**:
   click **More info → Run anyway**.

To update, install the newer release over the old one. Uninstall from *Settings → Apps*.

## Run from source

```powershell
git clone https://github.com/Khryz-Navarro/youtube-downloader-rust.git
cd youtube-downloader-rust
.\scripts\get-ffmpeg.ps1     # once: downloads the bundled ffmpeg into binaries\
cargo run
```

On the first start the status bar shows **Downloading yt-dlp…** for a few seconds, then
**yt-dlp &lt;version&gt; · up to date**. The Download button unlocks once yt-dlp is ready.
The first `cargo run` compiles Tauri and takes a few minutes; later runs start in seconds.

To build the installer (`target\release\bundle\nsis\YT Downloader_<version>_x64-setup.exe`), use the
Tauri CLI, either `cargo install tauri-cli --version "^2" --locked` once and then `cargo tauri build`,
or, with Node.js installed, `npx @tauri-apps/cli@2 build`.

## Using the app

1. **Paste a link** with **Paste** (or <kbd>Ctrl</kbd>+<kbd>V</kbd> in the link box). The preview shows the thumbnail, title, channel and highest resolution.
2. **Pick a quality,** or use the **Custom** row's dropdowns.
3. **Choose where to save** with **Change** (default: your Downloads folder).
4. **Click Download** (or press <kbd>Enter</kbd>). The link box clears for the next link.
5. **Open the Downloads tab** to watch progress.

| State | What it means |
|---|---|
| Starting… | yt-dlp is launching. If yt-dlp is still updating, it waits. |
| Video · 225 MB of 921 MB · 11 MB/s · 1m left | Downloading the video stream, then the audio stream. |
| Merging video and audio… | ffmpeg is combining the two streams. |
| ✓ Done · 727 KB | Finished. **Show in folder** opens File Explorer with the file selected. The 🗑 button (after a confirmation) moves the video to the Recycle Bin. |
| Paused · 48.9 MB of 921.4 MB | **Pause** (‖) stopped yt-dlp and kept the partial file; **Resume** continues from there. Not available while merging. |
| Canceled | **Retry** resumes from where it stopped; 🗑 deletes the leftover partial files instead. |
| Error message in red | yt-dlp's reason, e.g. "This video is unavailable". **Retry** tries again. |

The 🗑 button only touches files yt-dlp reported writing for *that* download: its `.part` pieces,
fragments, separate video/audio streams and ffmpeg's `.temp` merge file. Leftovers are deleted
permanently; a finished video goes to the Recycle Bin. An earlier download of the same video in another
quality is never touched.

## Where things are stored

| What | Where |
|---|---|
| Your downloads | The folder shown in the app (default: your Downloads folder) |
| yt-dlp program | `%APPDATA%\com.khryz.ytdownloader\bin\yt-dlp.exe` |
| Settings (theme, text and page size) | `%APPDATA%\com.khryz.ytdownloader\settings.json` |
| Remembered quality and folder | WebView2 storage in `%LOCALAPPDATA%\com.khryz.ytdownloader` |
| ffmpeg | `ffmpeg.exe` next to the app's exe (`binaries\` in the source tree) |

To force a clean reinstall of yt-dlp, close the app and delete `%APPDATA%\com.khryz.ytdownloader\bin`.

## How it's built

```
youtube-downloader-rust/
├─ src/
│  ├─ main.rs          Backend: window, yt-dlp install/update, downloads, pause/cancel, Delete, settings
│  └─ ytdlp.rs         Quality presets, yt-dlp arguments, output parsing, leftover matching (unit-tested)
├─ ui/
│  ├─ index.html       The window's markup (from the Electron version, plus window buttons)
│  ├─ styles.css       Dark and light themes
│  ├─ renderer.js      UI logic, the Electron version's renderer
│  └─ api.js           Tauri bridge: `window.api` for renderer.js, window buttons
├─ capabilities/       What the page may call (Tauri permissions)
├─ icons/              App icon (icon.svg is the source)
├─ scripts/
│  ├─ get-ffmpeg.ps1   Downloads the bundled ffmpeg
│  └─ release.ps1      Bump the version, push the branch, open the pull request
├─ .githooks/pre-push  Blocks direct pushes to main and branch pushes without a newer version
├─ .github/workflows/
│  └─ release.yml      After a merge to main: builds the installer and publishes the GitHub Release
├─ tauri.conf.json     App name, content security policy, installer settings
├─ PRODUCT.md          Who the app is for, what it does, its constraints and principles
├─ DESIGN.md           The visual system: tokens, components and rules ("Contact Sheet")
└─ Cargo.toml          Dependencies and the app version
```

- **Same page, new backend.** `renderer.js` calls the same `window.api` functions it did under
  Electron; `api.js` implements them with Tauri `invoke` calls and events. Each Electron IPC handler
  in `src/main.js` is a `#[tauri::command]` in `src/main.rs`.
- **Downloads** run `yt-dlp.exe` with a machine-readable progress template; each output line becomes a
  `progress` event for the page. **Pause** and **Cancel** end the whole process tree with
  `taskkill /T /F` (yt-dlp starts its own child and ffmpeg); a paused download keeps its `.part` file,
  so Resume continues it.
- **Security:** links must be `http(s)` and are passed after `--`, so they can't be read as yt-dlp
  options. Quality and settings values from the page are checked against fixed lists. Delete only
  removes files yt-dlp reported for that download, never a path sent by the page. The page has a
  strict content security policy and can only call the app's own commands and the window buttons.
- **Window:** frameless, like the Electron version; the page draws the title bar, including the
  minimize, maximize and close buttons.

## Development

```powershell
cargo test    # presets, argument building, output parsing, leftover matching
cargo run     # run the app (debug build, with a console for logs)
```

The UI files in `ui/` started as the Electron version's `src/renderer/`. The Contact Sheet redesign
landed here first; the Electron version gets the same `index.html`, `styles.css` and `renderer.js` next.
Its copy keeps Electron's own differences: no `api.js` (its preload script provides `window.api`), native
window buttons, and "Electron" in the About box. This version's `renderer.js` awaits `confirm(...)` in the
Delete handler because Tauri's `confirm()` is async; that's harmless under Electron.

Before designing anything new, read [PRODUCT.md](PRODUCT.md) and [DESIGN.md](DESIGN.md). They are
shared with the Electron version, so update them in both repositories.

### Releasing: pull request → merge → automatic release

Changes reach `main` only through a **pull request**, and every pull request carries a new version.

1. Work on a branch: `git switch -c my-change`, then commit your changes.
2. Run, from the repository folder:

   ```powershell
   .\scripts\release.ps1
   ```

   It bumps the patch version in `Cargo.toml` and `Cargo.lock` (0.1.0 → 0.1.1) and commits
   `Release v0.1.1` (only once per branch: running it again just pushes), pushes the branch, and opens
   a pull request into `main` with the [GitHub CLI](https://cli.github.com/) (`gh`). It refuses to run
   on `main` or with uncommitted changes, and turns on the pre-push hook.
3. Review and **merge the pull request** on GitHub.
4. **GitHub publishes the release by itself.** The merge starts the
   [Release workflow](.github/workflows/release.yml): if `v0.1.1` has no release yet, it runs the tests,
   builds the installer, and creates the GitHub Release *YT Downloader v0.1.1* (and its `v0.1.1` tag)
   with `YT-Downloader-Setup-0.1.1.exe` attached. Follow it on the repo's **Actions** tab.

The **pre-push hook** (`.githooks/pre-push`) blocks pushes straight to `main`, and blocks a branch push
whose version isn't newer than `main`'s. `release.ps1` turns it on; in a fresh clone you can also run
`git config core.hooksPath .githooks` once.

| To… | Run |
|---|---|
| Release a fix or small change (0.1.0 → 0.1.1) | `.\scripts\release.ps1` |
| Release new features (0.1.x → 0.2.0) | `.\scripts\release.ps1 minor` |
| Release a big change (0.x → 1.0.0) | `.\scripts\release.ps1 major` |
| Push without the checks (emergency only) | `git push --no-verify` |

If PowerShell refuses to run the script ("running scripts is disabled"), use
`powershell -ExecutionPolicy Bypass -File .\scripts\release.ps1`.

## Troubleshooting

| Problem | Fix |
|---|---|
| `cargo run` fails with *resource path `binaries\ffmpeg-x86_64-pc-windows-msvc.exe` doesn't exist* | Run `.\scripts\get-ffmpeg.ps1` once. |
| `cargo` fails with *link.exe not found* | Install the "Desktop development with C++" workload in the Visual Studio Installer. |
| Status bar: *Couldn't set up yt-dlp* | First launch needs internet to download yt-dlp. Connect and restart the app. |
| Status bar: *couldn't check for updates* | You're offline or GitHub is unreachable. The app keeps using the installed version. |
| A download fails with "Sign in to confirm your age" or a members-only error | These videos need your YouTube cookies, which this version doesn't support yet. |
| A 4K MP4 or MKV won't play | It's AV1 or VP9. Use VLC, install the Microsoft Store video extensions, or download 1080p/Default instead. |

## Known limitations

- The installer isn't code-signed, so Windows SmartScreen warns the first time it runs.
- The app doesn't update itself. (yt-dlp does, at every start.)
- Playlists are ignored: a link with `&list=` downloads only that one video.
- No cookie or login support (age-restricted and members-only videos fail).
- Tested only on Windows 11.

## Credits and legal

- [yt-dlp](https://github.com/yt-dlp/yt-dlp) does the downloading (Unlicense).
- [ffmpeg](https://ffmpeg.org/), from the [ffmpeg-static](https://github.com/eugeneware/ffmpeg-static) builds, merges the streams (GPL; its license ships with the app as `ffmpeg.LICENSE`).
- [Tauri](https://tauri.app/) provides the window and the bridge to Rust.
- Icons are from [Lucide](https://lucide.dev/) (ISC).

Only download videos you have the right to save. Downloading may be against YouTube's Terms of
Service and copyright law where you live; you are responsible for how you use this tool.

No license has been chosen for this project yet.
