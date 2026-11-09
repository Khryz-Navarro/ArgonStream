# Security policy

## Supported versions

Only the [latest release](https://github.com/Khryz-Navarro/ArgonStream/releases/latest) gets security
fixes. Fixes ship as a new release, so update to it. The Windows installer updates an installed copy in
place.

## Reporting a vulnerability

**Please don't open a public issue for a security problem.** Report it privately through the
repository's [private reporting form](https://github.com/Khryz-Navarro/ArgonStream/security/advisories/new)
(the **Security** tab, then **Report a vulnerability**). Only the maintainer can read it.

Include:

- the ArgonStream version (the version button in the title bar) and your system (Windows or Linux, and
  which version);
- what someone could do with the problem, and who could do it (a web page, a crafted link, another
  program on the computer…);
- the steps to reproduce it, ideally with a proof of concept.

You'll get a reply in the report itself. Once a fix is ready, it ships in a new release, and the report
becomes a published security advisory. You're credited in it unless you'd rather not be.

## What's in scope

ArgonStream's own code and the way it's built and shipped:

- the app: the Rust backend (`src/`), the page (`ui/`) and what the page is allowed to call
  (`capabilities/`, the content security policy in `tauri.conf.json`);
- the installers and packages, and the Release workflow that builds them;
- how the app fetches, updates and runs yt-dlp and ffmpeg.

Problems worth reporting include:

- a link, setting or page value that reaches yt-dlp as an option;
- the app deleting or overwriting a file it didn't create;
- a page script running code it shouldn't;
- the app downloading or running a program other than yt-dlp and ffmpeg;
- cookies, or the path of the cookies file, leaking anywhere.

The safety rules the code follows are in
[CONTRIBUTING.md](CONTRIBUTING.md#rules-that-keep-the-app-safe).

## What's out of scope

Report these to the project that owns them:

- **yt-dlp** itself: [yt-dlp's private reporting form](https://github.com/yt-dlp/yt-dlp/security/advisories/new);
- **ffmpeg**: [ffmpeg's security page](https://ffmpeg.org/security.html);
- **Tauri**, WebView2 or WebKitGTK: [Tauri's security policy](https://github.com/tauri-apps/tauri/security/policy);
- YouTube, or your browser's cookies.

A YouTube account getting restricted after its cookies are used with yt-dlp isn't a vulnerability in
ArgonStream. The README's [Cookies](README.md#cookies) section explains the risk.
