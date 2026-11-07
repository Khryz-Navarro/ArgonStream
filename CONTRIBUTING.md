# Contributing to YT Downloader

Thanks for helping. YT Downloader is a small Windows app built with Rust and Tauri 2. It drives
[yt-dlp](https://github.com/yt-dlp/yt-dlp) and a bundled ffmpeg so people can save a YouTube video or its
audio without a command line. [README.md](README.md) explains what the app does. This file covers how to
change it.

- [Before you start](#before-you-start)
- [Set up](#set-up)
- [Where things go](#where-things-go)
- [Rules that keep the app safe](#rules-that-keep-the-app-safe)
- [Common changes](#common-changes)
- [Design and wording](#design-and-wording)
- [Check your change](#check-your-change)
- [Branches, commits and pull requests](#branches-commits-and-pull-requests)
- [Reporting a problem](#reporting-a-problem)

## Before you start

- **Small fixes** (a typo, a clear bug with an obvious fix): open a pull request straight away.
- **Anything bigger** (a new feature, a new setting, a change to how downloads work): open an
  [issue](https://github.com/Khryz-Navarro/youtube-downloader-rust/issues) first and describe it, so
  nobody builds something that won't be merged.
- **License:** the project is under the [MIT License](LICENSE). By contributing, you agree that your
  contribution is released under it too.
- **Electron version:** YT Downloader started as an Electron app (`Khryz-Navarro/youtube-downloader`).
  That version is discontinued; all work happens here.

## Set up

You need Windows 10 or 11, [Rust](https://rustup.rs/) (stable, MSVC toolchain) and the Visual Studio C++
build tools ("Desktop development with C++"). See [Requirements](README.md#requirements).

```powershell
git clone https://github.com/Khryz-Navarro/youtube-downloader-rust.git
cd youtube-downloader-rust
.\scripts\get-ffmpeg.ps1                # once: the bundled ffmpeg, checked by its SHA-256
git config core.hooksPath .githooks      # once: turns on the pre-push check
cargo run                                # the app, with a console for logs
```

If PowerShell refuses to run a script, see [Troubleshooting](README.md#troubleshooting).

## Where things go

| Path | What belongs there |
|---|---|
| `src/ytdlp.rs` | **Pure logic, no Tauri:** yt-dlp arguments, quality and audio presets, output parsing, file naming, leftover matching. It has the unit tests. Put new logic here, with tests. |
| `src/main.rs` | **The backend's plumbing:** Tauri commands, running yt-dlp and ffmpeg, files, settings. Keep it thin; call into `ytdlp.rs` for decisions. |
| `ui/index.html`, `ui/styles.css`, `ui/renderer.js` | **The page:** plain HTML, CSS and JavaScript, with no framework, bundler or build step. `renderer.js` reaches the backend only through `window.api`. |
| `ui/api.js` | **The only Tauri-specific file in the page:** `window.api`, the window buttons, and the guards that turn off WebView2's browser behaviour. |
| `capabilities/default.json` | **What the page may call.** Only touch it when a change needs a new Tauri permission (a plugin or window API). |
| `tauri.conf.json` | **App name, installer and the content security policy.** |

**Adding a backend call takes three edits:**

1. a `#[tauri::command]` function in `src/main.rs`;
2. its name added to `tauri::generate_handler![…]` in `main()`;
3. a method on `window.api` in `ui/api.js` that `invoke`s it.

The content security policy allows no inline scripts or styles and no scripts from elsewhere. Put code in
the `.js` files and styles in `styles.css`.

## Rules that keep the app safe

These hold everywhere in the code. A pull request that breaks one won't be merged.

- **Treat everything from the page as untrusted.** Check it in Rust against a fixed list, the way
  `quality_args`, `audio_target`, `SETTINGS` and `assert_url` do.
- **Links** must be `http(s)` and always go after `--` in yt-dlp's arguments, so a link can never be read
  as an option.
- **Files:** only touch paths the backend learned from yt-dlp for that download, never a path sent by
  the page. Never overwrite a file the app didn't create (see `place()` in `main.rs`). A finished file is
  deleted only to the Recycle Bin, after the user confirms.
- **Never leave yt-dlp or ffmpeg running in the background.** Stopping kills the whole process tree, and
  closing the app stops every download.

## Common changes

| To add… | Change |
|---|---|
| **A video quality preset** | `quality_args` and the `QUALITIES` test list in `src/ytdlp.rs`; a radio in `#video-opts` in `ui/index.html`; its label in `QUALITY_LABEL` in `ui/renderer.js`; the table in [Quality options](README.md#quality-options). |
| **An audio format** | `AUDIO` and `convert_args` in `src/ytdlp.rs` (check that the bundled ffmpeg has the encoder: `ffmpeg -encoders`); a radio in `#audio-opts`; the table in [Audio only](README.md#audio-only). |
| **A setting** | `SETTINGS` and the defaults in `load_settings` in `src/main.rs`; a `.setting` group with a `.seg` radio group in the Settings dialog (`renderer.js` loads and saves any radio there by its `name`); the README's Settings mentions. |
| **A change to yt-dlp's output** | If you change `--progress-template` or `--print` in `download_args`, change `parse_line` and its test in the same commit. |

Update [README.md](README.md) in the same pull request whenever behavior changes. Also update
[PRODUCT.md](PRODUCT.md) when what the app can do changes. Keep the README's legal note.

## Design and wording

- **Read first:** [PRODUCT.md](PRODUCT.md) (who it's for, what it does, its limits) and
  [DESIGN.md](DESIGN.md) (the "Contact Sheet" look: tokens, components, rules) before changing the UI.
- **It's a quiet Windows app,** not a front-end for every yt-dlp option. A new option needs a reason
  most people would care about.
- **Use the colors and sizes defined in `ui/styles.css`.** Check both themes, and check
  **Text and page size** at 175%.
- **Use the UI's existing words** (New download, Downloads History, Quality, Show in folder, Retry, Resume…);
  PRODUCT.md lists them. Write plain, short labels.
- **Keyboard:** every control must be reachable with <kbd>Tab</kbd> and have a clear name for screen
  readers.
- **Claude Code:** the design tools used for this app (Impeccable and other skills) are in `.claude/`, so
  Claude Code sessions in this folder have them.

## Check your change

```powershell
cargo test                  # unit tests in src/ytdlp.rs
cargo clippy --all-targets  # must be clean
cargo run                   # try it for real
```

- **Anything touching downloads:** run a real download. Try Pause/Resume, Cancel, Retry and Delete if
  your change could affect them.
- **UI changes:** look at both themes and at 175%.
- **The installer:** build it with Tauri CLI 2.12.4 (`npx @tauri-apps/cli@2.12.4 build`) when you change packaging,
  `tauri.conf.json` or `installer/installer.nsi`. That file is the CLI's own installer template with changes marked
  `YT Downloader:`; when upgrading the CLI, start from its new template, re-apply them, and update the pinned version in
  `.github/workflows/release.yml` and the README.

## Branches, commits and pull requests

1. **Branch from `main`:** `git switch -c short-name`. The pre-push hook blocks pushes to `main`.
2. **Commit** with a short, imperative summary, like the existing history ("Add audio-only downloads:
   MP3, AAC, WAV, FLAC, WMA", "Fix gaps found in an audit of the Rust app"). In the body, say what
   changed and why.
3. **Open the pull request with `.\scripts\release.ps1`.** Every pull request carries a new version. The
   script:
   - bumps the patch version (`minor` or `major` as an argument for bigger changes);
   - commits `Release vX.Y.Z`;
   - pushes the branch and opens the pull request.

   If your branch builds on another open pull request, bump past that one's version yourself first (see
   [Releasing](README.md#releasing-pull-request--merge--automatic-release)).
4. **Describe the change** in the pull request:
   - what changes for the person using the app;
   - how it works;
   - what you checked (tests, real downloads, screenshots);
   - what you didn't check.
5. **Merging publishes a release.** The Release workflow builds the installer and publishes the GitHub
   Release for the new version. Merge only when the change is ready to ship.

## Reporting a problem

[Open an issue](https://github.com/Khryz-Navarro/youtube-downloader-rust/issues) with:

- the app version (the version button in the title bar) and the yt-dlp version (status bar);
- the link, if the video is public, and the quality you picked;
- what happened: the red error text in the Downloads History tab, or what you expected instead.

Check [Troubleshooting](README.md#troubleshooting) and [Known limitations](README.md#known-limitations)
first: age-restricted and members-only videos, for example, can't be downloaded yet.
