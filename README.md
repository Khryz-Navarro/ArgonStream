# ArgonStream

A small Windows and Linux app for downloading YouTube videos and audio on top of **[yt-dlp](https://github.com/yt-dlp/yt-dlp)**,
built with **Rust** and **[Tauri 2](https://tauri.app/)**: a Rust backend runs yt-dlp and ffmpeg, and the
window is drawn by the system's web view (WebView2 on Windows, WebKitGTK on Linux). The Windows installer
is about 23 MB, most of it ffmpeg.

Paste a link, pick a quality preset (or your own resolution and file type), and download. yt-dlp
installs itself on first launch and keeps itself up to date.

> **Status:** early versions (0.x) for Windows 10/11 and 64-bit Linux. Download the installer, `.deb` or AppImage from
> [Releases](https://github.com/Khryz-Navarro/ArgonStream/releases), or run from source.
> The look, "Contact Sheet", is recorded in [DESIGN.md](DESIGN.md). ArgonStream was called **YT Downloader** up to
> v0.1.10; its installer replaces an installed YT Downloader and keeps its settings.

## Contents

- [What it does](#what-it-does)
- [Features](#features)
- [Quality options](#quality-options)
- [Cookies](#cookies)
- [Requirements](#requirements)
- [Install](#install)
- [Run from source](#run-from-source)
- [Using the app](#using-the-app)
- [Where things are stored](#where-things-are-stored)
- [How it's built](#how-its-built)
- [Development](#development)
- [Troubleshooting](#troubleshooting)
- [Known limitations](#known-limitations)
- [Roadmap](#roadmap)
- [Credits and legal](#credits-and-legal)

## What it does

ArgonStream turns a YouTube link into a video or audio file on your PC, without a command line. Two
free tools do the heavy lifting: **yt-dlp** finds and downloads the streams, and **ffmpeg** joins or
converts them. The app installs and updates them, turns your choices into the right options, shows what's
happening, and cleans up after itself.

### From link to file

1. **It gets its tools ready.** Every time it starts, the app checks that yt-dlp is there (downloading it
   the first time) and lets it update itself, because YouTube changes often and old versions stop
   working. The status bar shows the yt-dlp version and whether it's up to date. Downloads wait until this
   check is done, so yt-dlp is never replaced in the middle of one.
2. **It looks the link up.** When you paste a link, the app asks yt-dlp about the video without
   downloading it and shows the thumbnail, title, channel, length and the highest resolution available.
   Only web (`http`/`https`) links are accepted, one video per link: a link with `&list=` gets just the
   video it points to.
3. **It turns your choice into yt-dlp options.** Pick a preset (4K Best, 4K, 1080p, 720p or Default), a
   Custom resolution and file type, or switch to Audio and pick a format. The app turns that into
   yt-dlp's format rules (see [Quality options](#quality-options)). If the video doesn't reach the
   resolution you picked, you get the best one below it.
4. **It downloads and joins the pieces.** YouTube serves high-quality video and its sound as separate
   streams. yt-dlp downloads both, then the bundled ffmpeg merges them into one playable file. For audio,
   ffmpeg converts the sound into the format you chose instead. The **Downloads History** tab shows each stage
   (video, audio, merging or converting) with size, speed and time left. Several downloads can run at
   once, each in its own yt-dlp process.
5. **It names and saves the file.** The finished file goes to the folder you chose, named in your
   [File names](#file-names) style (Classic by default: `Video Title (1080p).mp4`). It never overwrites a
   file that's already there.
6. **It lets you manage downloads.**
   - **Pause** keeps the partial file, and **Resume** continues from it.
   - **Cancel** stops a download. **Retry** starts it again, picking up any partial file.
   - **Show in folder** opens File Explorer (your file manager on Linux) with the file selected.
   - **Delete** removes what that download left behind: unfinished pieces are deleted for good, and a
     finished file goes to the Recycle Bin (the Trash on Linux) after you confirm. It only touches files yt-dlp reported
     writing for that download.
   - **Clear finished** tidies the list without touching any files.

It also remembers your last quality, your folder and your Settings (theme, text and page size, file names,
speed limit, cookies).

### What it doesn't do

- It doesn't sign in to YouTube itself. For age-restricted and members-only videos it can use your
  browser's signed-in session instead (see [Cookies](#cookies)).
- It downloads one video per link; whole playlists and channels aren't supported.
- It doesn't install its own updates: when a newer release is out, a banner says so and **Update** opens
  the release. (yt-dlp does update itself, at every start.)
- It's made and tested for YouTube. yt-dlp supports many other sites and the app accepts any
  web link, but other sites aren't tested.

## Features

The window has two tabs: **New download** (link box, preview, save folder and Download on the left,
Quality on the right) and **Downloads History** (the full-width list).

### Video and audio quality

- **Five video presets:**
  - **4K Best:** the highest quality up to 2160p, in whatever codec YouTube has (often AV1 or VP9), saved as `.mkv`.
  - **4K**, **1080p** and **720p:** the best MP4 up to that height, preferring H.264 so it plays everywhere.
  - **Default:** yt-dlp's own MP4 preset (H.264 + AAC first), for the widest compatibility.
- **Custom quality:** any maximum resolution (8K, 4K, 1440p, 1080p, 720p, 480p, 360p, 240p or 144p) as
  MP4, MKV or WebM. Choosing from either dropdown selects Custom.
- **Best audio included:** every video option adds the best audio stream.
- **Audio only:** the **Video / Audio** switch in Quality saves just the sound, as:
  - **MP3** (best VBR, about 245 kbps)
  - **AAC** (`.m4a`; YouTube's own AAC is kept as it is)
  - **WAV** (uncompressed)
  - **FLAC** (lossless)
  - **WMA**
- **Remembered choices:** the switch, the last video pick, the last audio pick and the Custom dropdowns
  are all kept for next time.

The exact yt-dlp options are in [Quality options](#quality-options).

### Adding a download

- **Paste:** the **Paste** button reads the clipboard. Once there's a link, it becomes a clear (×)
  button. <kbd>Ctrl</kbd>+<kbd>V</kbd> works too, and so does dragging a link from your browser into the
  link box.
- **Preview:** a moment after the link changes, the app shows the thumbnail, title, channel and length.
  Along the film frame is an edge code: video ID, best resolution (with the frame rate when it's above
  30 fps, e.g. `2160p60`) and length. If yt-dlp can't read a link, the preview says why.
- **Save folder:** your Downloads folder by default. **Change** picks another, which is remembered.
- **Start:** **Download** or <kbd>Enter</kbd> starts it. The link box then clears for the next link.

### Watching and managing downloads

- **Downloads History tab:** lists everything you've started since the app opened. Its count turns orange while
  something is downloading.
- **Each row** shows:
  - the thumbnail, title and quality (e.g. *1080p · MP4*, *Audio · MP3*);
  - a progress strip;
  - one big number: the percent while it runs, the file size once it's done;
  - what's happening now: *Video · 225 MB of 921 MB · 11 MB/s · 1m left*, then the audio stream, then
    *Merging video and audio…* or *Converting to MP3…*, or yt-dlp's reason if it failed.
- **Several at once:** each download runs in its own yt-dlp process.
- **Speed limit:** **Settings → Speed limit** caps how fast each download may go, so the app doesn't take
  your whole connection.
- **Pause and Resume:** Pause keeps the partial file and Resume continues from it. Not available while
  merging.
- **Cancel and Retry:** Retry starts a canceled or failed download again, picking up any partial file.
- **Show in folder:** opens File Explorer (your file manager on Linux) with the finished file selected.
- **Delete (🗑):**
  - for a finished download, it moves the file to the Recycle Bin (the Trash on Linux) after a confirm;
  - for a canceled or failed one, it deletes the leftover partial files.
  - It only touches files yt-dlp reported writing for that download. An earlier download of the same
    video is never touched.
- **Clear finished:** removes done, failed and canceled rows from the list, not the files.
- **Closing the app** stops its downloads, so yt-dlp is never left running in the background.

### Finished files

- **One playable file:** ffmpeg merges the video and audio streams, or converts the audio, into a single file.
- **File names:** **Settings → File names** picks the style, shown with an example:
  - **Classic:** `Video Title (1080p).mp4`
  - **Basic:** `Video Title.mp4`
  - **Pretty:** `Video Title - Author (1080p, h264).mp4`
  - **Nerdy:** `Video_Title_1080p_h264_aac.mp4`

  See [File names](#file-names).
- **Never overwrites:** if a name is taken, the new file gets ` (2)`, ` (3)` and so on.

### Tools managed for you

- **yt-dlp:** downloaded on first launch and updated with `yt-dlp -U` every time the app starts. A
  download started during the check shows *Waiting for yt-dlp update…* and begins right after.
- **Status bar:**
  - a dot: green when ready, amber while checking or offline, red if yt-dlp couldn't be set up;
  - the yt-dlp version: *up to date*, *updated just now* or *couldn't check for updates*;
  - "Developed by Khryz" in the middle;
  - on the right, the ffmpeg version and when yt-dlp was last checked.
- **ffmpeg:** bundled with the app, pinned to a build checked by its SHA-256, so there's nothing else to install.
- **ArgonStream itself:** at each start, after yt-dlp's check, the app asks GitHub once for its latest
  release. If that's newer, a banner under the title bar says so: **Update** opens the release page, and
  **×** hides the banner until the next start. Offline, nothing shows. The request carries nothing about
  you beyond what any web request does.

### Window and settings

- **The look, "Contact Sheet":** the app as a darkroom contact sheet. The preview sits in a film frame
  with its edge code, and each download's progress is a row of sprocket holes. They develop orange, end
  green when done, or stop red where it failed. The light theme is the light table.
- **Its own title bar:** Windows-style minimize, maximize and close buttons. The version button opens
  **About**, which shows the app, yt-dlp, ffmpeg and Tauri versions.
- **Settings** (the cog, top right), all saved for next time:
  - **Theme:** Light, Dark, or Desktop (follows the system's light or dark setting, live on Windows).
  - **Text and page size:** 100%, 125%, 150% or 175%. It zooms the whole window, for large or high-DPI
    screens.
  - **File names:** Classic, Basic, Pretty or Nerdy.
  - **Speed limit:** a slider with eight steps: 500 KB/s, 1, 2, 5, 10, 25 or 50 MB/s, or **Unlimited**
    (the default).
    - It caps each download on its own (yt-dlp's `--limit-rate`), so two downloads at 5 MB/s can use
      10 MB/s together.
    - It uses the same units as the download rows.
    - It takes effect on downloads you start or resume after changing it.
  - **Cookies:** Off, From Firefox, or From a file (a `cookies.txt`), for age-restricted and members-only
    videos. See [Cookies](#cookies).
- **Keyboard:**
  - every control is reachable with <kbd>Tab</kbd>;
  - arrow keys move between the tabs and between options;
  - <kbd>Enter</kbd> downloads;
  - <kbd>Esc</kbd> closes Settings and About.
- **Small:** the Windows installer is about 23 MB, and the window uses the system's web view: the WebView2
  built into Windows, or WebKitGTK on Linux.
- **Safe by default:**
  - only web links reach yt-dlp, and never as an option;
  - values from the page are checked against fixed lists;
  - Delete only removes files it was told about by yt-dlp;
  - browser shortcuts like <kbd>F5</kbd> are turned off, so a stray key can't wipe the download list.

  More in [How it's built](#how-its-built).

## Quality options

YouTube stores the high-quality video and the audio as separate streams. Each preset tells yt-dlp
which video stream to pick, adds the best audio stream, and merges them with ffmpeg. The arguments live
in `src/ytdlp.rs`.

| Preset | What you get | File | yt-dlp options |
|---|---|---|---|
| **4K Best** | Highest quality up to 2160p, in whatever codec YouTube has (often AV1 or VP9) | `.mkv` | `-f "bv*+ba/b" -S res:2160 --merge-output-format mkv` |
| **4K** | Best MP4 up to 2160p | `.mp4` | `-f "bv*[ext=mp4]+ba[ext=m4a]/b[ext=mp4]/bv*+ba/b" --merge-output-format mp4 --remux-video mp4 -S res:2160,vcodec:h264` |
| **1080p** | Best MP4 up to 1080p, H.264 when available | `.mp4` | same as 4K with `-S res:1080,vcodec:h264` |
| **720p** | Best MP4 up to 720p, H.264 when available | `.mp4` | same as 4K with `-S res:720,vcodec:h264` |
| **Default** | yt-dlp's own MP4 preset: H.264 + AAC first | `.mp4` | `-t mp4` |
| **Custom** · MP4 / MKV / WebM | The same rules up to the resolution you pick | `.mp4` / `.mkv` / `.webm` | `-S res:<N>` (+ `vcodec:h264` for MP4); WebM uses `-f "bv*[ext=webm]+ba[ext=webm]/b[ext=webm]/bv*+ba/b" --merge-output-format webm/mkv` |

While downloading, files are named `Title [videoId] 1080p.ext` (yt-dlp template
`%(title)s [%(id)s]%(height& {}p|)s.%(ext)s`), so different qualities of one video never collide and
Pause or Retry can pick up where they stopped. When a download finishes, the app renames the file in the
[File names](#file-names) style chosen in Settings. If a video doesn't have the exact resolution, the
preset picks the best one below the cap. WebM can't hold H.264, so a video with no WebM streams is
saved as `.mkv`.

### Audio only

Switch Quality to **Audio** to save only the sound. yt-dlp downloads the best audio stream as it is,
then the app converts it with the bundled ffmpeg. yt-dlp's own `-x` can't make WMA and wants ffprobe,
which isn't bundled. Only formats the bundled ffmpeg can encode are offered.

| Format | File | How it's made |
|---|---|---|
| **MP3** | `.mp3` | `-c:a libmp3lame -q:a 0`: best VBR, about 245 kbps |
| **AAC** | `.m4a` | YouTube's own AAC (`-f ba[ext=m4a]`), kept as it is; other sites' audio is encoded at `-c:a aac -b:a 192k` |
| **WAV** | `.wav` | `-c:a pcm_s16le`: uncompressed 16-bit |
| **FLAC** | `.flac` | `-c:a flac`: lossless, about half the size of WAV |
| **WMA** | `.wma` | `-c:a wmav2 -b:a 192k -ac 2 -ar 44100`: Windows Media Audio, stereo |

Audio files get the [File names](#file-names) style too (`Video Title.mp3` with Classic). While working,
the download is called `Title [videoId].f251-mp3.webm`: the format ID plus the target. That name can't
be mistaken for a finished file, and two formats of one video can download at the same time. It's
deleted once the conversion is done.

### File names

**Settings → File names** picks how finished files are named:

| Style | Video | Audio |
|---|---|---|
| **Classic** (default) | `Video Title (1080p).mp4` | `Video Title.mp3` |
| **Basic** | `Video Title.mp4` | `Video Title.mp3` |
| **Pretty** | `Video Title - Author (1080p, h264).mp4` | `Video Title - Author.mp3` |
| **Nerdy** | `Video_Title_1080p_h264_aac.mp4` | `Video_Title.mp3` |

- *Author* is the channel name. Codecs use their short names: `h264`, `h265`, `vp9`, `av1`, `aac`, `opus`.
  Audio files leave out the resolution and codecs, since the extension already says what they are.
- yt-dlp makes the title and channel safe for a file name (`%(title)S`), so characters like `/ : * ? "`
  are dropped.
- The app never replaces a file: if the name is taken, it adds ` (2)`, ` (3)` and so on. If the rename
  fails, the file keeps its working name.
- A download uses the style that was set when it started (or was resumed or retried).

## Cookies

Age-restricted and members-only videos, and YouTube's occasional "confirm you're not a bot" check, need a
signed-in YouTube session. **Settings → Cookies** lets yt-dlp use yours:

| Choice | What yt-dlp uses | Notes |
|---|---|---|
| **Off** (default) | Nothing | Those videos fail, with a note pointing to this setting. |
| **From Firefox** | Your signed-in Firefox's cookies (`--cookies-from-browser firefox`) | Sign in to YouTube in Firefox once. Firefox can stay open. |
| **From a file** | A `cookies.txt` you exported (`--cookies <file>`) | For Chrome, Edge and other browsers. Picking it asks for the file. |

Chrome and Edge can't be read directly: on Windows they lock their cookies to themselves (app-bound
encryption, since Chrome 127), so yt-dlp can't decrypt them. Export a `cookies.txt` from them instead:

1. Add a browser extension that exports cookies in the Netscape `cookies.txt` format. yt-dlp's
   [FAQ](https://github.com/yt-dlp/yt-dlp/wiki/FAQ#how-do-i-pass-cookies-to-yt-dlp) names ones it trusts.
2. Open a private window and sign in to YouTube. In that same tab, go to `youtube.com/robots.txt`, so no
   YouTube page stays open.
3. Export the cookies for youtube.com, then close the private window. YouTube rotates the cookies of open
   YouTube tabs, which would make the export stop working.
4. In ArgonStream: **Settings → Cookies → From a file**, then choose the file.

Good to know:

- **Cookies are like a password** for your YouTube account. ArgonStream saves only where the file is (in
  `settings.json`), never the cookies; yt-dlp reads them from the file, and writes refreshed ones back to it.
- **YouTube may restrict accounts** used this way. A spare account is safer.
- **The preview and the download** both use the cookies. Closing Settings after changing them looks the
  current link up again.
- **Still asked to sign in?** The cookies may be signed out or expired: sign in again and export them again.

## Requirements

- **Windows 10 or 11** with the WebView2 runtime (built into Windows 11 and current Windows 10), or
- **64-bit Linux** (x86-64) as new as Ubuntu 24.04, Debian 13 or Fedora 40 (glibc 2.39 or newer), with `curl`.
  The `.deb` installs what it needs (WebKitGTK 4.1, GTK 3, curl); the AppImage carries WebKitGTK itself.
- **Internet access** on first launch, to download yt-dlp.
- To build on Windows: **[Rust](https://rustup.rs/)** (stable, MSVC toolchain) and the **Visual Studio C++ build
  tools** ("Desktop development with C++" workload, which includes the Windows SDK).
- To build on Linux: **[Rust](https://rustup.rs/)** (stable) and the packages Tauri needs (see
  [Run from source](#run-from-source)).

You don't need to install yt-dlp, ffmpeg or Python yourself.

## Install

### Windows

1. Open [Releases](https://github.com/Khryz-Navarro/ArgonStream/releases) and download
   `ArgonStream-Setup-<version>.exe` from the newest release.
2. Run it. The installer isn't code-signed yet, so Windows may show **"Windows protected your PC"**:
   click **More info → Run anyway**.

**To update,** run the newer installer; there's no need to uninstall first. (When one is out, the app's banner
says so, and its **Update** button opens the release.) It sees the installed version, its
first page says **Update ArgonStream** (from which version to which), and **Next** installs the new version
over the old one in the same folder. Your settings, yt-dlp and downloaded videos are kept. Close the app first:
downloads still running in it stop.

An installed **YT Downloader** (the app's old name) is replaced the same way: the first page says **Update to
ArgonStream**, and the old app, its shortcuts and its uninstall entry are removed.

To go back to an older version, uninstall first. Uninstall from *Settings → Apps*.

### Linux

From the newest release on [Releases](https://github.com/Khryz-Navarro/ArgonStream/releases), take either:

- **`ArgonStream-<version>-amd64.deb`** for Debian, Ubuntu, Linux Mint and the like. Install it with
  `sudo apt install ./ArgonStream-<version>-amd64.deb`, then start **ArgonStream** from the app menu (under
  Video) or with `argonstream`. To update, install the newer `.deb` the same way. To uninstall:
  `sudo apt remove argon-stream`.
- **`ArgonStream-<version>-x86_64.AppImage`** for any other distribution. Make it executable and run it:
  `chmod +x ArgonStream-<version>-x86_64.AppImage`, then `./ArgonStream-<version>-x86_64.AppImage`. To
  update, replace it with the newer AppImage.

Either way your settings and yt-dlp are kept across updates, in `~/.local/share/com.khryz.ytdownloader`.

## Run from source

```powershell
git clone https://github.com/Khryz-Navarro/ArgonStream.git
cd ArgonStream
.\scripts\get-ffmpeg.ps1     # once: downloads the bundled ffmpeg into binaries\
cargo run
```

On Linux, install the packages Tauri builds against first (this is Debian and Ubuntu; Tauri's
[prerequisites](https://v2.tauri.app/start/prerequisites/#linux) list them for other distributions):

```bash
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev \
  libayatana-appindicator3-dev librsvg2-dev
git clone https://github.com/Khryz-Navarro/ArgonStream.git
cd ArgonStream
./scripts/get-ffmpeg.sh       # once: downloads the bundled ffmpeg into binaries/
cargo run
```

On the first start the status bar shows **Downloading yt-dlp…** for a few seconds, then
**yt-dlp &lt;version&gt; · up to date**. The Download button unlocks once yt-dlp is ready.
The first `cargo run` compiles Tauri and takes a few minutes; later runs start in seconds.

To build the installer (`target\release\bundle\nsis\ArgonStream_<version>_x64-setup.exe`), use the
Tauri CLI 2.12.4, either `cargo install tauri-cli --version "=2.12.4" --locked` once and then `cargo tauri build`,
or, with Node.js installed, `npx @tauri-apps/cli@2.12.4 build`. Use that exact version: the installer is built from
`installer/installer.nsi`, which is that version's template with a few changes (updates install in place, see
[Install](#install)). On Linux the same command builds the `.deb` and the AppImage instead, into
`target/release/bundle/deb/` and `target/release/bundle/appimage/` (`tauri.linux.conf.json` picks them).

## Using the app

1. **Paste a link** with **Paste** (or <kbd>Ctrl</kbd>+<kbd>V</kbd> in the link box). The preview shows the thumbnail, title, channel and highest resolution.
2. **Pick a quality,** or use the **Custom** row's dropdowns. For sound only, switch Quality to **Audio** and pick MP3, AAC, WAV, FLAC or WMA.
3. **Choose where to save** with **Change** (default: your Downloads folder).
4. **Click Download** (or press <kbd>Enter</kbd>). The link box clears for the next link.
5. **Open the Downloads History tab** to watch progress.

| State | What it means |
|---|---|
| Starting… | yt-dlp is launching. If yt-dlp is still updating, it waits. |
| Video · 225 MB of 921 MB · 11 MB/s · 1m left | Downloading the video stream, then the audio stream. |
| Merging video and audio… | ffmpeg is combining the two streams. |
| Converting to MP3… | Audio only: ffmpeg is turning the downloaded stream into the format you picked. Cancel stops it. |
| ✓ Done | Finished; the file size shows as the row's big number. **Show in folder** opens File Explorer (your file manager on Linux) with the file selected. The 🗑 button (after a confirmation) moves the file to the Recycle Bin (the Trash on Linux). |
| Paused · 48.9 MB of 921.4 MB | **Pause** (‖) stopped yt-dlp and kept the partial file; **Resume** continues from there. Not available while merging. |
| Canceled | **Retry** resumes from where it stopped; 🗑 deletes the leftover partial files instead. |
| Error message in red | yt-dlp's reason, e.g. "This video is unavailable". **Retry** tries again. |

The 🗑 button only touches files yt-dlp reported writing for *that* download: its `.part` pieces,
fragments, separate video/audio streams and ffmpeg's `.temp` merge file. Leftovers are deleted
permanently; a finished video goes to the Recycle Bin (the Trash on Linux). An earlier download of the same video in another
quality is never touched.

## Where things are stored

| What | Windows | Linux |
|---|---|---|
| Your downloads | The folder shown in the app (default: your Downloads folder) | The same |
| yt-dlp program | `%APPDATA%\com.khryz.ytdownloader\bin\yt-dlp.exe` | `~/.local/share/com.khryz.ytdownloader/bin/yt-dlp` |
| Settings (theme, text and page size, file names, speed limit, cookies and the cookies.txt location) | `%APPDATA%\com.khryz.ytdownloader\settings.json` | `~/.local/share/com.khryz.ytdownloader/settings.json` |
| Remembered quality and folder | WebView2 storage in `%LOCALAPPDATA%\com.khryz.ytdownloader` | WebKitGTK storage in `~/.local/share/com.khryz.ytdownloader` |
| ffmpeg | `ffmpeg.exe` next to the app's exe (`binaries\` in the source tree) | `argonstream-ffmpeg` next to the app (`/usr/bin` for the `.deb`) |

To force a clean reinstall of yt-dlp, close the app and delete that `bin` folder.

## How it's built

```
ArgonStream/
├─ src/
│  ├─ main.rs          Backend: window, yt-dlp install/update, downloads, pause/cancel, Delete, settings
│  └─ ytdlp.rs         Quality presets, yt-dlp arguments, output parsing, leftover matching (unit-tested)
├─ ui/
│  ├─ index.html       The window's markup, title bar and window buttons included
│  ├─ styles.css       Dark and light themes
│  ├─ renderer.js      UI logic: tabs, link box, preview, download list, Settings, status bar
│  └─ api.js           Bridge to the backend (`window.api`), window buttons, WebView2 guards
├─ capabilities/       What the page may call (Tauri permissions)
├─ icons/              App icon (icon.svg is the source)
├─ scripts/
│  ├─ get-ffmpeg.ps1   Downloads the bundled ffmpeg (Windows)
│  ├─ get-ffmpeg.sh    Downloads the bundled ffmpeg (Linux)
│  └─ release.ps1      Bump the version, push the branch, open the pull request
├─ .githooks/pre-push  Blocks direct pushes to main, and app changes without a newer version
├─ .github/workflows/
│  └─ release.yml      After a merge to main: builds the installer and Linux packages, publishes the Release
├─ tauri.conf.json     App name, content security policy, installer settings
├─ tauri.linux.conf.json  Linux only: the .deb and AppImage, and ffmpeg's name there
├─ PRODUCT.md          Who the app is for, what it does, its constraints and principles
├─ DESIGN.md           The visual system: tokens, components and rules ("Contact Sheet")
└─ Cargo.toml          Dependencies and the app version
```

- **Page and backend.** `renderer.js` only talks to the backend through `window.api`; `api.js`
  implements it with Tauri `invoke` calls and events, one `#[tauri::command]` in `src/main.rs` per call.
- **Downloads** run yt-dlp with a machine-readable progress template; each output line becomes a
  `progress` event for the page. **Pause** and **Cancel** end the whole process tree (yt-dlp starts its
  own child and ffmpeg): `taskkill /T /F` on Windows; on Linux every command starts in its own process
  group, which gets `SIGKILL`. A paused download keeps its `.part` file, so Resume continues it.
- **Security:** links must be `http(s)` and are passed after `--`, so they can't be read as yt-dlp
  options. Quality and settings values from the page are checked against fixed lists. Delete only
  removes files yt-dlp reported for that download, never a path sent by the page. The cookies.txt path
  comes from the system's file dialog, opened and saved by the backend. The page has a
  strict content security policy and can only call the app's own commands and the window buttons.
- **Window:** frameless; the page draws the title bar, including the minimize, maximize and close
  buttons. `api.js` turns off WebView2's browser shortcuts (F5 reload, print, find) and refuses drops
  that would replace the page.

## Development

```powershell
cargo test    # presets, argument building, output parsing, leftover matching
cargo run     # run the app (debug build, with a console for logs)
```

Want to help? [CONTRIBUTING.md](CONTRIBUTING.md) covers setup, where code goes, the safety rules, common
changes and how pull requests work.

Before designing anything new, read [PRODUCT.md](PRODUCT.md) and [DESIGN.md](DESIGN.md). The design
tools used to make them (Impeccable and other skills, listed in `skills-lock.json`) live in `.claude/`, so
Claude Code sessions in this folder have them.

ArgonStream started as YT Downloader, an Electron app (`Khryz-Navarro/youtube-downloader`). That version is
discontinued; all development happens here.

### Releasing: pull request → merge → automatic release

Changes reach `main` only through a **pull request**, and every pull request that changes the app carries a
new version. One that changes only docs or repo tooling (Markdown files, `LICENSE`, `.github/`, `.githooks/`,
`.claude/`, `release.ps1`) keeps the version, and merging it publishes nothing.

1. Work on a branch named `feat/…` for a feature or `bug/…` for a bug fix (`git switch -c feat/my-change`),
   then commit your changes.
2. Run, from the repository folder:

   ```powershell
   .\scripts\release.ps1
   ```

   It bumps the patch version in `Cargo.toml` and `Cargo.lock` (0.1.0 → 0.1.1) and commits
   `Release v0.1.1` (only once per branch: running it again just pushes; skipped when no app files
   changed), pushes the branch, and opens
   a pull request into `main` with the [GitHub CLI](https://cli.github.com/) (`gh`). It refuses to run
   on `main` or with uncommitted changes, and turns on the pre-push hook.
3. Review and **merge the pull request** on GitHub.
4. **GitHub publishes the release by itself.** The merge starts the
   [Release workflow](.github/workflows/release.yml): if `v0.1.1` has no release yet, it runs the tests and
   builds the Windows installer and the Linux packages side by side, then creates the GitHub Release
   *ArgonStream v0.1.1* (and its `v0.1.1` tag) with `ArgonStream-Setup-0.1.1.exe`,
   `ArgonStream-0.1.1-amd64.deb` and `ArgonStream-0.1.1-x86_64.AppImage` attached. If either build fails,
   nothing is published. Follow it on the repo's **Actions** tab.

The **pre-push hook** (`.githooks/pre-push`) blocks pushes straight to `main`, and blocks a branch push
that changes app files when its version isn't newer than `main`'s. `release.ps1` turns it on; in a fresh
clone you can also run
`git config core.hooksPath .githooks` once.

| To… | Run |
|---|---|
| Release a fix or small change (0.1.0 → 0.1.1) | `.\scripts\release.ps1` |
| Release new features (0.1.x → 0.2.0) | `.\scripts\release.ps1 minor` |
| Release a big change (0.x → 1.0.0) | `.\scripts\release.ps1 major` |
| Push without the checks (emergency only) | `git push --no-verify` |

The script compares your branch only with `main`. If your branch builds on another pull request that isn't
merged yet, bump past that pull request's version yourself (in `Cargo.toml`, then `cargo update --workspace`)
before running it; otherwise both would ship the same version.

## Troubleshooting

| Problem | Fix |
|---|---|
| `cargo run` fails with *resource path `binaries\ffmpeg-x86_64-pc-windows-msvc.exe` doesn't exist* | Run `.\scripts\get-ffmpeg.ps1` once. |
| `cargo` fails with *link.exe not found* | Install the "Desktop development with C++" workload in the Visual Studio Installer. |
| PowerShell says *running scripts is disabled* for a script in `scripts\` | Run it as `powershell -ExecutionPolicy Bypass -File .\scripts\<name>.ps1`. |
| `get-ffmpeg.ps1` or `get-ffmpeg.sh` says *checksum mismatch* | The download isn't the expected ffmpeg build; the script deleted it. Try again later, and don't build with a different file. |
| Linux: `cargo run` fails with *resource path `binaries/argonstream-ffmpeg-x86_64-unknown-linux-gnu` doesn't exist* | Run `./scripts/get-ffmpeg.sh` once. |
| Linux: `cargo` fails because `webkit2gtk-4.1` (or `gtk+-3.0`, `libsoup-3.0`) isn't found | Install the packages listed in [Run from source](#run-from-source). |
| Linux: status bar says *Couldn't set up yt-dlp: couldn't run curl* | Install curl (`sudo apt install curl`); the `.deb` does this for you, the AppImage can't. |
| Linux: the AppImage doesn't start and mentions FUSE | Install your distribution's FUSE package (`fuse3` or `fuse`), or run it with `--appimage-extract-and-run`. |
| Status bar: *Couldn't set up yt-dlp* | First launch needs internet to download yt-dlp. Connect and restart the app. |
| Status bar: *couldn't check for updates* | You're offline or GitHub is unreachable. The app keeps using the installed version. |
| A download fails with "Sign in to confirm your age" or a members-only error | Turn on **Settings → Cookies** (see [Cookies](#cookies)). If it's already on, the cookies may be signed out or expired: export them again. |
| A 4K MP4 or MKV won't play | It's AV1 or VP9. Use VLC, install the Microsoft Store video extensions, or download 1080p/Default instead. |

## Known limitations

- The installer isn't code-signed, so Windows SmartScreen warns the first time it runs.
- The app doesn't install its own updates: its banner opens the new release, and you run the installer.
  (yt-dlp does update itself, at every start.)
- Playlists are ignored: a link with `&list=` downloads only that one video. Whole playlists are
  [planned](https://github.com/Khryz-Navarro/ArgonStream/wiki/Playlists).
- Cookies come only from Firefox or a `cookies.txt` file: Chrome and Edge lock theirs on Windows.
- Tested on Windows 11, and on Linux only on Ubuntu 26.04 under WSL. The title bar keeps its Windows-style
  buttons on Linux too.
- Linux builds are x86-64 only.

## Roadmap

What's planned next is on the wiki's [Roadmap](https://github.com/Khryz-Navarro/ArgonStream/wiki/Roadmap):

1. YouTube clips, whole playlists and GIFs from X;
2. converting, compressing for Discord, and trimming and cropping files on your computer;
3. image galleries (gallery-dl) and transcripts and subtitles;
4. a mobile app and a Discord bot.

None of it is in the app yet. Each feature has a plan page on the wiki and an issue labeled
[`roadmap`](https://github.com/Khryz-Navarro/ArgonStream/issues?q=label%3Aroadmap), where you can say what
you'd use it for.

## Credits and legal

- [yt-dlp](https://github.com/yt-dlp/yt-dlp) does the downloading (Unlicense).
- [ffmpeg](https://ffmpeg.org/), from the [ffmpeg-static](https://github.com/eugeneware/ffmpeg-static) builds, merges the streams (GPL; its license ships with the app as `ffmpeg.LICENSE`).
- [Tauri](https://tauri.app/) provides the window and the bridge to Rust.
- Icons are from [Lucide](https://lucide.dev/) (ISC).

Only download videos you have the right to save. Downloading may be against YouTube's Terms of
Service and copyright law where you live; you are responsible for how you use this tool.

ArgonStream's own code is released under the [MIT License](LICENSE). yt-dlp and ffmpeg are separate
programs under their own licenses (above); the app runs them but doesn't change them.
