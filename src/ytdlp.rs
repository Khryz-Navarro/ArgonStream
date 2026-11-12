// Pure helpers for driving yt-dlp: argument building and output parsing.
// No Tauri here, so `cargo test` covers it.

use serde_json::{Value, json};

pub const HEIGHTS: [u32; 9] = [4320, 2160, 1440, 1080, 720, 480, 360, 240, 144];
pub const OUTPUT_TEMPLATE: &str = "%(title)s [%(id)s]%(height& {}p|)s.%(ext)s"; // "Title [id] 1080p.mp4"
#[cfg(test)]
const QUALITIES: [&str; 5] = ["best4k", "mp4_4k", "mp4_1080", "mp4_720", "mp4_default"];

/// Settings → Download speed, slowest first: each download's cap as yt-dlp's --limit-rate takes it (K and M are
/// KiB and MiB, the same units as the app's "MB/s"). "unlimited" adds no cap. ui/renderer.js keeps the same list.
pub const SPEED_LIMITS: [&str; 8] = ["500K", "1M", "2M", "5M", "10M", "25M", "50M", "unlimited"];

pub const BINARY_NAME: &str = if cfg!(windows) { "yt-dlp.exe" } else { "yt-dlp" };
pub const BINARY_URL: &str = if cfg!(windows) {
    "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe"
} else if cfg!(target_os = "macos") {
    "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_macos"
} else {
    "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_linux"
};

fn strings(args: &[&str]) -> Vec<String> {
    args.iter().map(|s| s.to_string()).collect()
}

// `res:N` = best resolution up to N; `vcodec:h264` breaks ties toward H.264 so MP4s play everywhere.
// WebM can't hold H.264/AAC, so if a video has no WebM streams the merge falls back to MKV.
fn container(ext: &str, h: u32) -> Option<Vec<String>> {
    let res = format!("res:{h}");
    Some(match ext {
        "mp4" => strings(&[
            "-f", "bv*[ext=mp4]+ba[ext=m4a]/b[ext=mp4]/bv*+ba/b", "--merge-output-format", "mp4", "--remux-video", "mp4",
            "-S", &format!("{res},vcodec:h264"),
        ]),
        "mkv" => strings(&["-f", "bv*+ba/b", "-S", &res, "--merge-output-format", "mkv"]),
        "webm" => strings(&[
            "-f", "bv*[ext=webm]+ba[ext=webm]/b[ext=webm]/bv*+ba/b", "-S", &res, "--merge-output-format", "webm/mkv",
        ]),
        _ => return None,
    })
}

// Audio-only downloads: the page's key, the file extension, and the name shown in messages. yt-dlp fetches the
// best audio stream as-is and the app converts it with the bundled ffmpeg (yt-dlp's own -x can't make WMA and
// wants ffprobe, which isn't bundled). Only formats the bundled ffmpeg can encode are listed.
const AUDIO: [(&str, &str, &str); 5] = [
    ("audio-mp3", "mp3", "MP3"),
    ("audio-aac", "m4a", "AAC"),
    ("audio-wav", "wav", "WAV"),
    ("audio-flac", "flac", "FLAC"),
    ("audio-wma", "wma", "WMA"),
];

/// For an audio-only key, the file extension it ends up with and its display name.
pub fn audio_target(quality: &str) -> Option<(&'static str, &'static str)> {
    AUDIO.iter().find(|(key, ..)| *key == quality).map(|(_, ext, name)| (*ext, *name))
}

/// A preset key, an audio key, or "custom-<container>-<height>" from the Custom row (validated: it comes from the page).
pub fn quality_args(quality: &str) -> Result<Vec<String>, String> {
    let args = match quality {
        "best4k" => container("mkv", 2160),
        "mp4_4k" => container("mp4", 2160),
        "mp4_1080" => container("mp4", 1080),
        "mp4_720" => container("mp4", 720),
        "mp4_default" => Some(strings(&["-t", "mp4"])), // yt-dlp's own MP4 preset (H.264 + AAC preferred)
        // YouTube's AAC comes as .m4a, so AAC prefers it and needs no conversion
        "audio-aac" => Some(strings(&["-f", "ba[ext=m4a]/ba/b"])),
        q if audio_target(q).is_some() => Some(strings(&["-f", "ba/b"])),
        _ => quality.strip_prefix("custom-").and_then(|rest| rest.split_once('-')).and_then(|(ext, h)| {
            let h: u32 = h.bytes().all(|b| b.is_ascii_digit()).then(|| h.parse().ok())??;
            if HEIGHTS.contains(&h) { container(ext, h) } else { None }
        }),
    };
    args.ok_or_else(|| format!("Unknown quality: {quality}"))
}

/// Links come from the clipboard/user; only web URLs may reach yt-dlp.
pub fn assert_url(url: &str) -> Result<String, String> {
    let u = url::Url::parse(url.trim()).map_err(|e| format!("Invalid link: {e}"))?;
    if !matches!(u.scheme(), "http" | "https") {
        return Err("Only http(s) links are supported".into());
    }
    Ok(u.into())
}

/// `cookies` are `cookie_args`: an age-restricted video can't even be looked up without them.
/// `--playlist-items 1`: a link to a playlist alone has no video of its own, so its first one is previewed
/// instead of every video being looked up.
pub fn info_args(url: &str, cookies: &[String]) -> Vec<String> {
    let mut args = strings(&[
        "--no-playlist", "--playlist-items", "1", "--no-warnings", "--encoding", "utf-8",
        "-O", "%(.{id,title,channel,uploader,duration_string,thumbnail,height,fps})j",
    ]);
    args.extend_from_slice(cookies);
    args.extend(strings(&["--", url])); // `--` so a link can never be read as an option
    args
}

/// Looks up the playlist a link belongs to without opening each video: one line per video, for `playlist_info`.
pub fn playlist_args(url: &str, cookies: &[String]) -> Vec<String> {
    let mut args = strings(&[
        "--yes-playlist", "--flat-playlist", "--no-warnings", "--encoding", "utf-8",
        "-O", "%(.{playlist_title,id,title})j",
    ]);
    args.extend_from_slice(cookies);
    args.extend(strings(&["--", url]));
    args
}

/// `playlist_args`' output as the page gets it: the playlist's title, how many videos it has, and each video's id
/// and title (to name the ones a download skips). None when the link has no playlist.
pub fn playlist_info(output: &str) -> Option<Value> {
    let lines: Vec<Value> = output.lines().filter_map(|l| serde_json::from_str(l).ok()).collect();
    let title = lines.first()?["playlist_title"].as_str()?; // a video on its own has none
    let videos: Vec<Value> = lines.iter().map(|v| json!({ "id": v["id"], "title": v["title"] })).collect();
    Some(json!({ "title": title, "count": videos.len(), "videos": videos }))
}

/// Settings → Cookies as yt-dlp options, so it uses a signed-in YouTube session (age-restricted and members-only
/// videos). `mode` is "off", "firefox" or "file"; `file` is the cookies.txt chosen in Settings. Only Firefox is offered
/// as a browser: on Windows, Chrome and Edge lock their cookies to themselves (app-bound encryption since Chrome 127),
/// so yt-dlp can't read them; their users export a cookies.txt instead.
pub fn cookie_args(mode: &str, file: Option<&str>) -> Result<Vec<String>, String> {
    match mode {
        "firefox" => Ok(strings(&["--cookies-from-browser", "firefox"])),
        "file" => match file.filter(|f| !f.is_empty()) {
            Some(file) => Ok(strings(&["--cookies", file])),
            None => Err("Choose a cookies file in Settings, or turn Cookies off".into()),
        },
        _ => Ok(vec![]),
    }
}

// yt-dlp errors that a signed-in session fixes: "Sign in to confirm your age", "Sign in to confirm you're not a
// bot", members-only videos.
fn needs_cookies(error: &str) -> bool {
    let error = error.to_lowercase();
    error.contains("sign in to confirm") || error.contains("members-only") || error.contains("age-restricted")
}

/// An error as the app shows it. For the ones a signed-in session fixes, yt-dlp's command-line advice ("Use
/// --cookies-from-browser or --cookies…", then links) gives way to where Settings → Cookies is; `cookies_on` says
/// whether it's already on. Other errors come back unchanged.
pub fn explain(error: &str, cookies_on: bool) -> String {
    if !needs_cookies(error) {
        return error.to_string();
    }
    let error = error.split(" Use --cookies").next().unwrap_or(error).trim_end();
    if cookies_on {
        format!("{error} The cookies may be signed out or expired: sign in again, then update them in Settings.")
    } else {
        format!("{error} Turn on Cookies in Settings to use your signed-in YouTube session.")
    }
}

// Videos: the resolution keeps different qualities of one video from sharing a name, which made yt-dlp skip a
// new download as "already downloaded"; `&...|` leaves it out when there's no height.
// Audio: the stream is only the source for the conversion, so it gets yt-dlp's own ".f<format>" naming plus the
// target ("Title [id].f251-mp3.webm"). It can never be mistaken for a finished "Title [id].mp3" (which yt-dlp
// would report as already downloaded, and the conversion would then delete), and two formats of one video
// downloading at once don't share a file.
// A whole playlist goes into a folder named after it; yt-dlp turns the title into a safe folder name.
fn output_template(quality: &str, playlist: bool) -> String {
    let folder = if playlist { "%(playlist_title,playlist_id|Playlist)s/" } else { "" };
    match audio_target(quality) {
        Some((ext, _)) => format!("{folder}%(title)s [%(id)s].f%(format_id)s-{ext}.%(ext)s"),
        None => format!("{folder}{OUTPUT_TEMPLATE}"),
    }
}

/// `limit` is a SPEED_LIMITS cap for this download, or None for no cap; `cookies` are `cookie_args`.
/// `archive` is None for one video: the link's own, or the first one of a link that only names a playlist or a
/// channel (the one the preview showed, as in `info_args`). For a whole playlist it's the file where yt-dlp notes
/// each finished video, so Resume and Retry skip those without looking them up again. yt-dlp moves on to the next
/// video after an error.
pub fn download_args(
    url: &str,
    quality: &str,
    folder: &str,
    ffmpeg: Option<&str>,
    limit: Option<&str>,
    cookies: &[String],
    archive: Option<&str>,
) -> Result<Vec<String>, String> {
    let mut args = quality_args(quality)?;
    match archive {
        Some(archive) => args.extend(strings(&["--yes-playlist", "--download-archive", archive])),
        None => args.extend(strings(&["--no-playlist", "--playlist-items", "1"])),
    }
    // A playlist's video also reports its place and how many there are, for "3 of 17" and its file name.
    let (meta, place) = match archive {
        Some(_) => (",playlist_index,playlist_count", "\t%(playlist_index|)s\t%(playlist_count|)s"),
        None => ("", ""),
    };
    args.extend(strings(&[
        "--encoding", "utf-8", "--newline", "--progress",
        "--progress-template", "download:DL|%(info.vcodec)s|%(progress)j",
        "--print", &format!("video:META|%(.{{title,thumbnail{meta}}})j"),
        "--print", "after_move:FILE|%(filepath)s",
        // The parts of the Settings → File names name. `S` makes yt-dlp sanitize a part as a file name (no
        // / \ : * ? " < > | and no control characters), so the tabs between them can't come from a title.
        "--print", &format!("after_move:NAME|%(title)S\t%(channel,uploader|)S\t%(height|)s\t%(vcodec|)s\t%(acodec|)s{place}"),
        "-P", folder, "-o", &output_template(quality, archive.is_some()),
    ]));
    if let Some(ffmpeg) = ffmpeg {
        args.extend(strings(&["--ffmpeg-location", ffmpeg]));
    }
    if let Some(limit) = limit {
        args.extend(strings(&["--limit-rate", limit]));
    }
    args.extend_from_slice(cookies);
    args.extend(strings(&["--", url]));
    Ok(args)
}

/// Turns one line of yt-dlp output into an event for the UI, or None if it's noise.
pub fn parse_line(line: &str) -> Option<Value> {
    if let Some(file) = line.strip_prefix("FILE|") {
        return Some(json!({ "file": file }));
    }
    if let Some(name) = line.strip_prefix("NAME|") {
        let mut parts = name.split('\t');
        let mut next = || parts.next().unwrap_or_default();
        return Some(json!({ "name": {
            "title": next(), "author": next(), "height": next(), "vcodec": next(), "acodec": next(),
            "index": next(), "count": next(), // a playlist's video: its place, and how many there are
        } }));
    }
    if let Some(meta) = line.strip_prefix("META|") {
        let m = serde_json::from_str::<Value>(meta).ok().filter(Value::is_object)?;
        if !m["playlist_index"].is_null() {
            // a playlist's next video: the row keeps the playlist's own title and thumbnail
            let mut ev = json!({ "video": m["title"], "index": m["playlist_index"] });
            if !m["playlist_count"].is_null() {
                ev["count"] = m["playlist_count"].clone(); // else the page keeps the count from its lookup
            }
            return Some(ev);
        }
        return Some(json!({ "title": m["title"], "thumbnail": m["thumbnail"] }));
    }
    let (vcodec, progress) = line.strip_prefix("DL|")?.split_once('|')?;
    let p: Value = serde_json::from_str(progress).ok()?;
    let stream = if vcodec == "none" { "audio" } else { "video" };
    let filename = &p["filename"]; // the stream file being written, e.g. "…\Title [id].f701.mp4"
    let or = |v: &Value, fallback: Value| if v.is_null() { fallback } else { v.clone() };
    match p["status"].as_str() {
        Some("finished") => Some(json!({ "state": "processing", "stream": stream, "filename": filename })),
        Some("downloading") => Some(json!({
            "state": "downloading", "stream": stream, "filename": filename,
            "downloaded": or(&p["downloaded_bytes"], json!(0)),
            "total": or(&p["total_bytes"], p["total_bytes_estimate"].clone()),
            "speed": p["speed"],
            "eta": p["eta"],
        })),
        _ => None,
    }
}

// "Title [id].f701.mp4" -> "Title [id]": drops the extension and a yt-dlp format id (".f701"). Format ids have
// no spaces, so a title like "Best.fails [id].webm" keeps its ".fails [id]".
fn stem(name: &str) -> &str {
    let Some((base, _)) = name.rsplit_once('.').filter(|(_, ext)| !ext.is_empty()) else { return name };
    match base.rsplit_once('.') {
        Some((stem, format)) if format.len() > 1 && format.starts_with('f') && !format.contains(' ') => stem,
        _ => base,
    }
}

/// File names for converting a downloaded audio file: "Title [id].f251-mp3.webm" + "mp3" ->
/// ("Title [id].f251-mp3.temp.mp3", "Title [id].mp3"). The temp name follows yt-dlp's own temp naming, so
/// `leftover_names` cleans it up, and it never collides with another format's conversion.
pub fn audio_names(name: &str, ext: &str) -> (String, String) {
    let base = name.rsplit_once('.').map_or(name, |(base, _)| base);
    (format!("{base}.temp.{ext}"), format!("{}.{ext}", stem(name)))
}

/// ffmpeg arguments that turn a downloaded audio file into `output` (the `.temp.` name from `audio_names`).
pub fn convert_args(input: &str, output: &str, ext: &str) -> Vec<String> {
    let codec: &[&str] = match ext {
        "mp3" => &["-c:a", "libmp3lame", "-q:a", "0"], // LAME V0: the best VBR quality, about 245 kbps
        "m4a" => &["-c:a", "aac", "-b:a", "192k"],
        "wav" => &["-c:a", "pcm_s16le", "-rf64", "auto"], // RF64 only past 4 GB
        "flac" => &["-c:a", "flac"],
        // WMA can't hold more than 2 channels or rates above 48 kHz; 44.1 kHz stereo plays everywhere
        "wma" => &["-c:a", "wmav2", "-b:a", "192k", "-ac", "2", "-ar", "44100"],
        _ => &[], // not reachable from AUDIO; ffmpeg would pick the extension's default encoder
    };
    let mut args = strings(&["-hide_banner", "-loglevel", "error", "-nostdin", "-y", "-i", input, "-vn"]);
    args.extend(strings(codec));
    args.push(output.to_string());
    args
}

/// Which entries of a folder listing are leftovers of one download. `streams` are the file names yt-dlp
/// reported writing ("Title [id].f701.mp4"); leftovers are those files, their .part / .part-FragN /
/// .ytdl files, ffmpeg's merge temp file ("Title [id].temp.mkv"), and a temp file named after the stream
/// itself ("Title [id].f251-mp3.temp.mp3": an audio conversion, or yt-dlp's fix-up of an .m4a). Anything else,
/// like an earlier download of the same video in another quality, never matches.
pub fn leftover_names<'a>(streams: &[String], names: &'a [String]) -> Vec<&'a String> {
    let own_temps = streams.iter().map(|s| format!("{}.temp.", s.rsplit_once('.').map_or(s.as_str(), |(base, _)| base)));
    let stems: Vec<String> = streams.iter().map(|s| format!("{}.temp.", stem(s))).chain(own_temps).collect();
    names
        .iter()
        .filter(|n| {
            streams.iter().any(|s| *n == s || n.starts_with(&format!("{s}.part")) || **n == format!("{s}.ytdl"))
                || stems.iter().any(|temp| n.starts_with(temp))
        })
        .collect()
}

// yt-dlp's codec ids -> the short names people know: "avc1.640028" -> "h264". Unknown ids keep their first part
// when it's a plain word ("opus", "vp9"); "none" (no such stream) and anything odd give nothing.
fn codec_name(id: &str) -> Option<&str> {
    match id.split('.').next()? {
        "avc1" | "avc3" => Some("h264"),
        "hev1" | "hvc1" => Some("h265"),
        "vp09" => Some("vp9"),
        "av01" => Some("av1"),
        "mp4a" => Some("aac"),
        "none" | "" => None,
        other => other.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-').then_some(other),
    }
}

/// The finished file's name in a Settings → File names style, from a NAME| line's parts (`parse_line`):
/// classic "Title (1080p).mp4", basic "Title.mp4", pretty "Title - Author (1080p, h264).mp4",
/// nerdy "Title_1080p_h264_aac.mp4". Codecs are named only for videos; an audio file's extension already
/// says it. A playlist's video leads with its place, padded so the files sort in the playlist's order:
/// "03 Title (1080p).mp4", nerdy "03_Title_1080p_h264_aac.mp4". None (no title) keeps yt-dlp's working name.
pub fn styled_name(style: &str, parts: &Value, ext: &str) -> Option<String> {
    let part = |key: &str| parts[key].as_str().map(str::trim).filter(|s| !s.is_empty());
    let title = part("title")?;
    let height = part("height").map(|h| format!("{h}p"));
    let vcodec = part("vcodec").and_then(codec_name);
    let acodec = vcodec.and(part("acodec").and_then(codec_name));
    let index = part("index").filter(|i| i.bytes().all(|b| b.is_ascii_digit())).map(|i| {
        let width = part("count").map_or(0, str::len).max(2);
        format!("{i:0>width$}")
    });
    let stem = match style {
        "basic" => title.to_string(),
        "pretty" => {
            let author = part("author").map(|a| format!(" - {a}")).unwrap_or_default();
            let details: Vec<&str> = height.as_deref().into_iter().chain(vcodec).collect();
            let details = if details.is_empty() { String::new() } else { format!(" ({})", details.join(", ")) };
            format!("{title}{author}{details}")
        }
        "nerdy" => title.split_whitespace().chain(height.as_deref()).chain(vcodec).chain(acodec).collect::<Vec<_>>().join("_"),
        _ => height.map_or_else(|| title.to_string(), |h| format!("{title} ({h})")), // classic
    };
    let stem = match index {
        Some(index) => format!("{index}{}{stem}", if style == "nerdy" { "_" } else { " " }),
        None => stem,
    };
    let name = format!("{stem}.{ext}");
    (!name.contains(['/', '\\'])).then_some(name) // yt-dlp sanitized the parts; never leave the folder anyway
}

/// The n-th choice for a name that's taken: "Title.mp4", "Title (2).mp4", "Title (3).mp4"…
pub fn numbered_name(name: &str, n: u32) -> String {
    match (n, name.rsplit_once('.')) {
        (0 | 1, _) => name.to_string(),
        (_, Some((stem, ext))) => format!("{stem} ({n}).{ext}"),
        (_, None) => format!("{name} ({n})"),
    }
}

// An "ERROR:" line as (the video's id, the message): a leading "[extractor] id: " gives the id and is dropped.
fn error_parts(line: &str) -> Option<(Option<&str>, &str)> {
    let rest = line.strip_prefix("ERROR:")?.trim_start();
    let split = rest
        .strip_prefix('[')
        .and_then(|r| r.split_once(']'))
        .filter(|(extractor, _)| !extractor.is_empty())
        .and_then(|(_, r)| r.trim_start().split_once(':'))
        .filter(|(id, _)| !id.is_empty() && id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-'));
    Some(split.map_or((None, rest), |(id, message)| (Some(id), message.trim_start())))
}

/// "ERROR: [youtube] abc123: Video unavailable" -> "Video unavailable"
pub fn error_text(output: &str) -> Option<String> {
    output.lines().rev().find_map(error_parts).map(|(_, message)| message.to_string())
}

/// A video that a playlist download skipped, from its "ERROR:" line: `{ id, error }`; the id is null when
/// yt-dlp names none.
pub fn skipped(line: &str) -> Option<Value> {
    error_parts(line).map(|(id, error)| json!({ "id": id, "error": error }))
}

/// A download's id comes from the page and names its playlist's archive file, so only a UUID's characters pass.
pub fn safe_id(id: &str) -> bool {
    !id.is_empty() && id.len() <= 64 && id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
}

/// "ffmpeg version 6.1.1-essentials_build-www.gyan.dev Copyright…" -> "6.1.1"
pub fn ffmpeg_version(output: &str) -> Option<String> {
    output.match_indices("version ").find_map(|(i, m)| {
        let v: String = output[i + m.len()..]
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '.')
            .collect();
        v.starts_with(|c: char| c.is_ascii_digit()).then_some(v)
    })
}

/// The latest ArgonStream release, which the app asks GitHub for at each start.
pub const RELEASES_API: &str = "https://api.github.com/repos/Khryz-Navarro/ArgonStream/releases/latest";
const RELEASES: &str = "https://github.com/Khryz-Navarro/ArgonStream/releases/";

/// "0.1.16" -> [0, 1, 16]; anything but three plain numbers is None.
fn version_parts(v: &str) -> Option<[u64; 3]> {
    let parts: Vec<u64> =
        v.split('.').map(|p| p.parse().ok().filter(|_| p.bytes().all(|b| b.is_ascii_digit()))).collect::<Option<_>>()?;
    parts.try_into().ok()
}

/// GitHub's answer for the latest release, if it's newer than `current` ("0.1.16"): its version, its page, and
/// the Windows installer with its SHA-256. The installer is null when its asset is missing or anything about it
/// looks wrong, so Update falls back to the page. Every URL must be this repo's own release URL for that version.
pub fn newer_release(current: &str, release: &Value) -> Option<Value> {
    let version = release["tag_name"].as_str()?.strip_prefix('v')?;
    if version_parts(version)? <= version_parts(current)? {
        return None;
    }
    let page = release["html_url"].as_str().filter(|p| *p == format!("{RELEASES}tag/v{version}"))?;
    let name = format!("ArgonStream-Setup-{version}.exe");
    let installer = release["assets"].as_array().into_iter().flatten().find(|a| a["name"] == name.as_str()).and_then(|a| {
        let url = a["browser_download_url"].as_str().filter(|u| *u == format!("{RELEASES}download/v{version}/{name}"))?;
        let sha256 = a["digest"].as_str()?.strip_prefix("sha256:")?;
        let sha256 = Some(sha256).filter(|h| h.len() == 64 && h.bytes().all(|b| b.is_ascii_hexdigit()))?;
        Some(json!({ "url": url, "sha256": sha256.to_ascii_lowercase(), "size": a["size"].as_u64()? }))
    });
    Some(json!({ "version": version, "page": page, "installer": installer }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn after<'a>(args: &'a [String], flag: &str) -> &'a str {
        &args[args.iter().position(|a| a == flag).unwrap() + 1]
    }

    #[test]
    fn every_quality_builds_args_that_end_with_dashdash_url() {
        for quality in QUALITIES {
            let args = download_args("https://youtu.be/x", quality, "C:\\out", Some("ff.exe"), None, &[], None).unwrap();
            assert_eq!(args[args.len() - 2..], ["--", "https://youtu.be/x"]);
            assert!(args.iter().any(|a| a == "--no-playlist"));
            assert_eq!(after(&args, "-P"), "C:\\out");
            assert_eq!(after(&args, "-o"), OUTPUT_TEMPLATE);
            assert_eq!(after(&args, "--ffmpeg-location"), "ff.exe");
        }
        let args = download_args("https://youtu.be/x", "mp4_720", ".", None, None, &[], None).unwrap();
        assert!(!args.iter().any(|a| a == "--ffmpeg-location"));
        assert!(!args.iter().any(|a| a == "--limit-rate"), "no cap unless one is set");
    }

    #[test]
    fn speed_limits_become_ytdlps_limit_rate() {
        for quality in ["mp4_1080", "audio-mp3"] {
            let args = download_args("https://youtu.be/x", quality, ".", None, Some("5M"), &[], None).unwrap();
            assert_eq!(after(&args, "--limit-rate"), "5M");
            assert_eq!(args[args.len() - 2..], ["--", "https://youtu.be/x"]); // the link stays last, after `--`
        }
        // slowest first, then "unlimited"; every cap is a number with yt-dlp's K or M
        let (caps, last) = SPEED_LIMITS.split_at(SPEED_LIMITS.len() - 1);
        assert_eq!(last, ["unlimited"]);
        let bytes = |cap: &str| {
            let (n, unit) = cap.split_at(cap.len() - 1);
            n.parse::<u64>().unwrap() * if unit == "K" { 1 << 10 } else { assert_eq!(unit, "M"); 1 << 20 }
        };
        assert!(caps.windows(2).all(|w| bytes(w[0]) < bytes(w[1])), "{caps:?}");
    }

    #[test]
    fn cookies_reach_ytdlp_for_lookups_and_downloads() {
        assert_eq!(cookie_args("off", Some("C:\\c.txt")).unwrap(), Vec::<String>::new());
        assert_eq!(cookie_args("firefox", None).unwrap(), ["--cookies-from-browser", "firefox"]);
        assert_eq!(cookie_args("file", Some("C:\\my cookies.txt")).unwrap(), ["--cookies", "C:\\my cookies.txt"]);
        for missing in [None, Some("")] {
            assert!(cookie_args("file", missing).unwrap_err().contains("Choose a cookies file"));
        }
        // both the preview lookup and the download get them, always before `--` so the link stays last
        let cookies = cookie_args("file", Some("C:\\c.txt")).unwrap();
        let info = info_args("https://youtu.be/x", &cookies);
        assert_eq!(after(&info, "--cookies"), "C:\\c.txt");
        assert_eq!(info[info.len() - 2..], ["--", "https://youtu.be/x"]);
        for quality in ["mp4_1080", "audio-mp3"] {
            let args = download_args("https://youtu.be/x", quality, ".", None, Some("5M"), &cookies, None).unwrap();
            assert_eq!(after(&args, "--cookies"), "C:\\c.txt");
            assert_eq!(args[args.len() - 2..], ["--", "https://youtu.be/x"]);
        }
        assert!(!download_args("https://youtu.be/x", "mp4_1080", ".", None, None, &[], None).unwrap().iter().any(|a| a.starts_with("--cookies")));
    }

    #[test]
    fn needs_cookies_spots_errors_a_signed_in_session_fixes() {
        for error in [
            "Sign in to confirm your age. This video may be inappropriate for some users.",
            "Sign in to confirm you’re not a bot. Use --cookies-from-browser or --cookies for the authentication.",
            "Join this channel to get access to members-only content like this video, and other exclusive perks.",
            "This video is age-restricted and only available on YouTube.",
        ] {
            assert!(needs_cookies(error), "{error}");
        }
        for error in ["Video unavailable. This video is private", "Unable to download webpage", "HTTP Error 404: Not Found"] {
            assert!(!needs_cookies(error), "{error}");
            assert_eq!(explain(error, false), error); // unchanged
        }
        // yt-dlp 2026.10.26's real message for an age-restricted video, after error_text
        let age = error_text("ERROR: [youtube] HtVdAasjOgU: Sign in to confirm your age. This video may be inappropriate for some users. Use --cookies-from-browser or --cookies for the authentication. See  https://github.com/yt-dlp/yt-dlp/wiki/FAQ#how-do-i-pass-cookies-to-yt-dlp  for how to manually pass cookies.").unwrap();
        assert_eq!(
            explain(&age, false),
            "Sign in to confirm your age. This video may be inappropriate for some users. Turn on Cookies in Settings to use your signed-in YouTube session."
        );
        assert!(explain(&age, true).ends_with("The cookies may be signed out or expired: sign in again, then update them in Settings."));
        assert!(!explain(&age, true).contains("--cookies"));
    }

    #[test]
    fn quality_presets_pick_the_right_container_and_resolution_cap() {
        let q = |k| quality_args(k).unwrap();
        assert_eq!(after(&q("best4k"), "-S"), "res:2160");
        assert!(q("best4k").iter().any(|a| a == "mkv"));
        assert_eq!(after(&q("mp4_4k"), "-S"), "res:2160,vcodec:h264");
        assert_eq!(after(&q("mp4_1080"), "-S"), "res:1080,vcodec:h264");
        assert_eq!(after(&q("mp4_720"), "-S"), "res:720,vcodec:h264");
        for k in ["mp4_4k", "mp4_1080", "mp4_720"] {
            assert!(q(k).iter().any(|a| a == "mp4"));
        }
        assert_eq!(q("mp4_default"), ["-t", "mp4"]);
        // the exact arguments, in order, as the README documents them
        assert_eq!(q("mp4_1080"), [
            "-f", "bv*[ext=mp4]+ba[ext=m4a]/b[ext=mp4]/bv*+ba/b", "--merge-output-format", "mp4", "--remux-video", "mp4",
            "-S", "res:1080,vcodec:h264",
        ]);
        assert_eq!(q("best4k"), ["-f", "bv*+ba/b", "-S", "res:2160", "--merge-output-format", "mkv"]);
    }

    #[test]
    fn audio_keys_download_only_the_best_audio_stream() {
        assert_eq!(quality_args("audio-aac").unwrap(), ["-f", "ba[ext=m4a]/ba/b"]);
        for (key, ext) in [("audio-mp3", "mp3"), ("audio-aac", "m4a"), ("audio-wav", "wav"), ("audio-flac", "flac"), ("audio-wma", "wma")] {
            if key != "audio-aac" {
                assert_eq!(quality_args(key).unwrap(), ["-f", "ba/b"], "{key}");
            }
            let args = download_args("https://youtu.be/x", key, "C:\\out", Some("ff.exe"), None, &[], None).unwrap();
            assert!(!args.iter().any(|a| a == "-x" || a == "--merge-output-format"), "{key}: no extraction or merge");
            // the source gets its own ".f<format>-<target>" name, never the finished file's
            assert_eq!(after(&args, "-o"), format!("%(title)s [%(id)s].f%(format_id)s-{ext}.%(ext)s"));
            assert_eq!(args[args.len() - 2..], ["--", "https://youtu.be/x"]);
        }
        assert_eq!(audio_target("audio-mp3"), Some(("mp3", "MP3")));
        assert_eq!(audio_target("audio-aac"), Some(("m4a", "AAC")));
        assert_eq!(audio_target("audio-wav"), Some(("wav", "WAV")));
        assert_eq!(audio_target("audio-flac"), Some(("flac", "FLAC")));
        assert_eq!(audio_target("audio-wma"), Some(("wma", "WMA")));
        assert_eq!(audio_target("mp4_1080"), None);
    }

    #[test]
    fn convert_args_pick_each_formats_encoder() {
        let codec = |ext| {
            let a = convert_args("C:\\out\\T [x].f251-mp3.webm", "C:\\out\\T [x].f251-mp3.temp.out", ext);
            assert_eq!(a[..8], ["-hide_banner", "-loglevel", "error", "-nostdin", "-y", "-i", "C:\\out\\T [x].f251-mp3.webm", "-vn"]);
            assert_eq!(a.last().unwrap(), "C:\\out\\T [x].f251-mp3.temp.out");
            a[8..a.len() - 1].join(" ")
        };
        assert_eq!(codec("mp3"), "-c:a libmp3lame -q:a 0");
        assert_eq!(codec("m4a"), "-c:a aac -b:a 192k");
        assert_eq!(codec("wav"), "-c:a pcm_s16le -rf64 auto");
        assert_eq!(codec("flac"), "-c:a flac");
        assert_eq!(codec("wma"), "-c:a wmav2 -b:a 192k -ac 2 -ar 44100");
    }

    #[test]
    fn audio_names_keep_the_title_and_mark_the_temp_file() {
        assert_eq!(
            audio_names("Me at the zoo [jNQXAC9IVRw].f251-mp3.webm", "mp3"),
            ("Me at the zoo [jNQXAC9IVRw].f251-mp3.temp.mp3".into(), "Me at the zoo [jNQXAC9IVRw].mp3".into())
        );
        assert_eq!(audio_names("Best.fails [x].f251-wav.webm", "wav").1, "Best.fails [x].wav"); // a dot in the title survives
        assert_eq!(audio_names("T [x].f140-m4a.m4a", "m4a").1, "T [x].m4a"); // AAC: only a rename
    }

    #[test]
    fn leftover_names_cleans_up_an_audio_download_but_keeps_finished_files() {
        let streams = strings(&["Mr. T [abc].f251-mp3.webm"]);
        let folder = strings(&[
            "Mr. T [abc].f251-mp3.webm.part",     // interrupted download
            "Mr. T [abc].f251-mp3.webm",          // downloaded, conversion canceled
            "Mr. T [abc].f251-mp3.temp.mp3",      // half-written conversion
            "Mr. T [abc].mp3",                    // an earlier finished MP3: keep
            "Mr. T [abc].f251-wav.webm.part",     // a WAV of the same video downloading at the same time: keep
            "Mr. T [abc].f251-wav.temp.wav",      // and its conversion: keep
        ]);
        assert_eq!(leftover_names(&streams, &folder), folder[..3].iter().collect::<Vec<_>>());
        // yt-dlp's own fix-up of an .m4a writes "<file minus ext>.temp.m4a"
        let m4a = strings(&["T [x].f140-m4a.m4a"]);
        let folder = strings(&["T [x].f140-m4a.temp.m4a", "T [x].m4a"]);
        assert_eq!(leftover_names(&m4a, &folder), folder[..1].iter().collect::<Vec<_>>());
    }

    #[test]
    fn unknown_quality_is_rejected() {
        for quality in ["nope", "toString", "custom-mp4-1000", "custom-avi-1080", "custom-mp4-1080 --exec x", "custom-mp4-+1080", "custom-mp4-", "audio-ogg", "audio-", "audio-mp3 --exec x"] {
            let err = download_args("https://youtu.be/x", quality, ".", None, None, &[], None).unwrap_err();
            assert!(err.contains("Unknown quality"), "{quality}");
        }
    }

    #[test]
    fn custom_quality_maps_resolution_and_file_type_to_ytdlp_options() {
        let q = |k| quality_args(k).unwrap();
        assert_eq!(after(&q("custom-mp4-1440"), "-S"), "res:1440,vcodec:h264");
        assert_eq!(after(&q("custom-mp4-1440"), "--merge-output-format"), "mp4");
        assert_eq!(after(&q("custom-mkv-4320"), "-S"), "res:4320");
        assert_eq!(after(&q("custom-mkv-4320"), "--merge-output-format"), "mkv");
        assert_eq!(after(&q("custom-webm-480"), "-S"), "res:480");
        assert_eq!(after(&q("custom-webm-480"), "--merge-output-format"), "webm/mkv");
        for h in HEIGHTS {
            assert!(quality_args(&format!("custom-mkv-{h}")).is_ok());
        }
        // the fixed presets are the same builders, so 4K Best == Custom MKV at 2160p
        assert_eq!(q("best4k"), q("custom-mkv-2160"));
    }

    #[test]
    fn assert_url_only_lets_web_links_through() {
        assert_eq!(assert_url(" https://www.youtube.com/watch?v=abc ").unwrap(), "https://www.youtube.com/watch?v=abc");
        assert!(assert_url("--exec calc").is_err());
        assert!(assert_url("file:///C:/Windows/win.ini").unwrap_err().contains("http"));
        assert!(assert_url("").is_err());
        assert!(info_args("https://youtu.be/x", &[]).ends_with(&["--".to_string(), "https://youtu.be/x".to_string()]));
    }

    #[test]
    fn a_link_to_a_playlist_alone_previews_only_its_first_video() {
        let info = info_args("https://www.youtube.com/playlist?list=PL1", &[]);
        assert!(info.iter().any(|a| a == "--no-playlist"));
        assert_eq!(after(&info, "--playlist-items"), "1");
    }

    #[test]
    fn a_playlist_is_looked_up_without_opening_each_video() {
        let cookies = cookie_args("firefox", None).unwrap();
        let args = playlist_args("https://youtu.be/x?list=PL1", &cookies);
        assert!(args.iter().any(|a| a == "--yes-playlist") && args.iter().any(|a| a == "--flat-playlist"));
        assert_eq!(after(&args, "--cookies-from-browser"), "firefox");
        assert_eq!(args[args.len() - 2..], ["--", "https://youtu.be/x?list=PL1"]);

        // yt-dlp 2026.10.26's real lines for "Official Blender Open Movies", cut to two videos
        let out = concat!(
            r#"{"playlist_title": "Official Blender Open Movies", "id": "l5OZu-IrXpw", "title": "SINGULARITY - Painterly Space Adventure"}"#, "\n",
            r#"{"playlist_title": "Official Blender Open Movies", "id": "u9lj-c29dxI", "title": "WING IT! - Blender Open Movie"}"#, "\n",
        );
        assert_eq!(playlist_info(out), Some(json!({
            "title": "Official Blender Open Movies", "count": 2,
            "videos": [
                { "id": "l5OZu-IrXpw", "title": "SINGULARITY - Painterly Space Adventure" },
                { "id": "u9lj-c29dxI", "title": "WING IT! - Blender Open Movie" },
            ],
        })));
        // a video on its own, nothing at all, or noise: no playlist
        assert_eq!(playlist_info(r#"{"id": "x", "title": "T"}"#), None);
        assert_eq!(playlist_info(""), None);
        assert_eq!(playlist_info("WARNING: something"), None);
    }

    #[test]
    fn a_whole_playlist_goes_into_its_own_folder_and_notes_finished_videos() {
        for quality in ["mp4_1080", "audio-mp3"] {
            let args = download_args("https://youtu.be/x?list=PL1", quality, "C:\\out", None, None, &[], Some("C:\\data\\id.txt")).unwrap();
            assert!(args.iter().any(|a| a == "--yes-playlist"), "{quality}");
            assert!(!args.iter().any(|a| a == "--no-playlist" || a == "--playlist-items"), "{quality}: every video");
            assert_eq!(after(&args, "--download-archive"), "C:\\data\\id.txt");
            assert_eq!(after(&args, "-P"), "C:\\out");
            assert!(after(&args, "-o").starts_with("%(playlist_title,playlist_id|Playlist)s/%(title)s [%(id)s]"), "{quality}");
            assert_eq!(args[args.len() - 2..], ["--", "https://youtu.be/x?list=PL1"]);
        }
        // one video: no archive, no folder of its own, and nothing about a playlist in what yt-dlp prints
        let one = download_args("https://youtu.be/x?list=PL1", "mp4_1080", "C:\\out", None, None, &[], None).unwrap();
        assert!(one.iter().any(|a| a == "--no-playlist") && !one.iter().any(|a| a == "--yes-playlist" || a == "--download-archive"));
        assert_eq!(after(&one, "-o"), OUTPUT_TEMPLATE);
        assert!(one.iter().any(|a| a == "video:META|%(.{title,thumbnail})j"));
        assert!(!one.iter().any(|a| a.contains("playlist_index")));
        // even for a link that only names a playlist or a channel: its first video, the one the preview showed
        assert_eq!(after(&one, "--playlist-items"), "1");
    }

    #[test]
    fn a_playlists_videos_report_their_place_and_lead_their_names_with_it() {
        // yt-dlp 2026.10.26's real META line for the third of 17 videos
        assert_eq!(
            parse_line(r#"META|{"title": "CHARGE - Blender Open Movie", "thumbnail": "https://i.ytimg.com/x.jpg", "playlist_index": 3, "playlist_count": 17}"#),
            Some(json!({ "video": "CHARGE - Blender Open Movie", "index": 3, "count": 17 }))
        );
        // no total from yt-dlp: no "count", so the page keeps the one from its lookup
        assert_eq!(parse_line(r#"META|{"title": "T", "playlist_index": 3}"#), Some(json!({ "video": "T", "index": 3 })));
        let args = download_args("https://youtu.be/x", "mp4_1080", ".", None, None, &[], Some("a.txt")).unwrap();
        assert!(args.iter().any(|a| a == "video:META|%(.{title,thumbnail,playlist_index,playlist_count})j"));
        assert!(args.iter().any(|a| a.ends_with("\t%(playlist_index|)s\t%(playlist_count|)s")), "yt-dlp prints the place");
        let video = &parse_line("NAME|Video Title\tAuthor\t1080\tavc1.640028\tmp4a.40.2\t3\t17").unwrap()["name"];
        assert_eq!(styled_name("classic", video, "mp4").as_deref(), Some("03 Video Title (1080p).mp4"));
        assert_eq!(styled_name("basic", video, "mp4").as_deref(), Some("03 Video Title.mp4"));
        assert_eq!(styled_name("pretty", video, "mp4").as_deref(), Some("03 Video Title - Author (1080p, h264).mp4"));
        assert_eq!(styled_name("nerdy", video, "mp4").as_deref(), Some("03_Video_Title_1080p_h264_aac.mp4"));
        // padded to the playlist's length, so 3 sorts before 120
        let long = &parse_line("NAME|T\t\t\tnone\topus\t3\t120").unwrap()["name"];
        assert_eq!(styled_name("basic", long, "mp3").as_deref(), Some("003 T.mp3"));
        // anything but a number is left out
        assert_eq!(styled_name("basic", &json!({ "title": "T", "index": "NA", "count": "17" }), "mp4").as_deref(), Some("T.mp4"));
    }

    #[test]
    fn skipped_names_the_video_and_why() {
        assert_eq!(
            skipped("ERROR: [youtube] Xk29aLq0P1c: Private video. Sign in if you've been granted access to this video"),
            Some(json!({ "id": "Xk29aLq0P1c", "error": "Private video. Sign in if you've been granted access to this video" }))
        );
        assert_eq!(skipped("ERROR: Unable to download webpage"), Some(json!({ "id": null, "error": "Unable to download webpage" })));
        assert_eq!(skipped("[download] Downloading item 3 of 17"), None);
    }

    #[test]
    fn safe_id_only_lets_a_uuids_characters_name_a_file() {
        assert!(safe_id("3f2b8c1e-9a4d-4c7b-8e21-0d5f6a7b8c9d"));
        for id in ["", "..", "a/b", "a\\b", "..\\..\\x", "a.txt", "a b", "id:1", &"a".repeat(65)] {
            assert!(!safe_id(id), "{id}");
        }
    }

    #[test]
    fn parse_line_reads_progress_finish_file_and_ignores_noise() {
        let p = json!({ "status": "downloading", "downloaded_bytes": 50, "total_bytes": 200, "speed": 10.5, "eta": 15, "filename": "C:\\out\\T [x].f701.mp4" });
        assert_eq!(
            parse_line(&format!("DL|avc1.640028|{p}")),
            Some(json!({ "state": "downloading", "stream": "video", "filename": "C:\\out\\T [x].f701.mp4", "downloaded": 50, "total": 200, "speed": 10.5, "eta": 15 }))
        );
        assert_eq!(
            parse_line(r#"DL|none|{"status":"downloading","total_bytes_estimate":9}"#),
            Some(json!({ "state": "downloading", "stream": "audio", "filename": null, "downloaded": 0, "total": 9, "speed": null, "eta": null }))
        );
        assert_eq!(
            parse_line(r#"DL|none|{"status":"finished","filename":"T [x].f251.webm"}"#),
            Some(json!({ "state": "processing", "stream": "audio", "filename": "T [x].f251.webm" }))
        );
        assert_eq!(parse_line("FILE|C:\\out\\a | b [x].mp4"), Some(json!({ "file": "C:\\out\\a | b [x].mp4" })));
        assert_eq!(
            parse_line(r#"META|{"title": "강남 | Style", "thumbnail": "https://i.ytimg.com/x.jpg"}"#),
            Some(json!({ "title": "강남 | Style", "thumbnail": "https://i.ytimg.com/x.jpg" }))
        );
        assert_eq!(parse_line("META|{broken"), None);
        assert_eq!(parse_line(r#"[Merger] Merging formats into "x.mp4""#), None);
        assert_eq!(parse_line("DL|avc1|{not json"), None);
        assert_eq!(parse_line(r#"DL|avc1|{"status":"error"}"#), None);
    }

    #[test]
    fn leftover_names_finds_only_this_downloads_partial_and_temporary_files() {
        let s = |v: &[&str]| strings(v);
        let streams = s(&["Mr. T [abc].f701.mp4", "Mr. T [abc].f251.webm"]);
        let folder = s(&[
            "Mr. T [abc].f701.mp4.part",       // video stream, interrupted
            "Mr. T [abc].f701.mp4.part-Frag12", // a leftover fragment
            "Mr. T [abc].f251.webm",            // audio stream, finished before the cancel
            "Mr. T [abc].f251.webm.ytdl",
            "Mr. T [abc].temp.mkv",   // ffmpeg was merging
            "Mr. T [abc].mp4",        // an earlier, finished download of the same video: keep
            "Mr. T [abc].f22.mp4.part", // a different format, not written by this download: keep
            "Other [xyz].f701.mp4.part", // another video: keep
        ]);
        assert_eq!(leftover_names(&streams, &folder), folder[..5].iter().collect::<Vec<_>>());
        // single-file downloads have no ".fNNN" part; the merge temp rule still uses the right stem
        let folder = s(&["A [q].mp4.part", "A [q].temp.mp4", "A [q].mkv"]);
        assert_eq!(leftover_names(&s(&["A [q].mp4"]), &folder), folder[..2].iter().collect::<Vec<_>>());
        assert!(leftover_names(&[], &s(&["Mr. T [abc].f701.mp4.part"])).is_empty());
    }

    #[test]
    fn stem_strips_the_format_id_and_extension() {
        assert_eq!(stem("Mr. T [abc].f701.mp4"), "Mr. T [abc]");
        assert_eq!(stem("A [q].mp4"), "A [q]");
        assert_eq!(stem("A."), "A.");
        assert_eq!(stem("noext"), "noext");
        assert_eq!(stem("x.f.mp4"), "x.f"); // a format id needs a character after the f
        assert_eq!(stem("Best.fails [x].webm"), "Best.fails [x]"); // ".fails [x]" has a space: title, not a format id
        assert_eq!(stem("Best.fails [x].f251.webm"), "Best.fails [x]");
    }

    #[test]
    fn styled_names_follow_the_file_names_setting() {
        let line = "NAME|Video Title\tAuthor\t1080\tavc1.640028\tmp4a.40.2";
        let args = download_args("https://youtu.be/x", "mp4_1080", ".", None, None, &[], None).unwrap();
        assert!(args.iter().any(|a| a.starts_with("after_move:NAME|%(title)S\t")), "yt-dlp prints the parts");
        let video = &parse_line(line).unwrap()["name"];
        assert_eq!(styled_name("classic", video, "mp4").as_deref(), Some("Video Title (1080p).mp4"));
        assert_eq!(styled_name("basic", video, "mp4").as_deref(), Some("Video Title.mp4"));
        assert_eq!(styled_name("pretty", video, "mp4").as_deref(), Some("Video Title - Author (1080p, h264).mp4"));
        assert_eq!(styled_name("nerdy", video, "mp4").as_deref(), Some("Video_Title_1080p_h264_aac.mp4"));
        assert_eq!(styled_name("unknown", video, "mp4"), styled_name("classic", video, "mp4"));

        // 4K Best: AV1 + Opus, and no channel name
        let av1 = &parse_line("NAME|T\t\t2160\tav01.0.12M.10\topus").unwrap()["name"];
        assert_eq!(styled_name("pretty", av1, "mkv").as_deref(), Some("T (2160p, av1).mkv"));
        assert_eq!(styled_name("nerdy", av1, "mkv").as_deref(), Some("T_2160p_av1_opus.mkv"));
        let vp9 = &parse_line("NAME|T\tA\t1440\tvp09.00.50.08\topus").unwrap()["name"];
        assert_eq!(styled_name("nerdy", vp9, "webm").as_deref(), Some("T_1440p_vp9_opus.webm"));

        // audio: no resolution or video codec, so no codecs at all (the extension names it)
        let audio = &parse_line("NAME| Song  Name \tBand\t\tnone\topus").unwrap()["name"];
        assert_eq!(styled_name("classic", audio, "mp3").as_deref(), Some("Song  Name.mp3"));
        assert_eq!(styled_name("basic", audio, "mp3").as_deref(), Some("Song  Name.mp3"));
        assert_eq!(styled_name("pretty", audio, "mp3").as_deref(), Some("Song  Name - Band.mp3"));
        assert_eq!(styled_name("nerdy", audio, "m4a").as_deref(), Some("Song_Name.m4a"));

        // nothing usable keeps yt-dlp's working name
        assert_eq!(styled_name("classic", &parse_line("NAME|  ").unwrap()["name"], "mp4"), None);
        assert_eq!(styled_name("classic", &Value::Null, "mp4"), None);
        assert_eq!(styled_name("basic", &json!({ "title": "a/b" }), "mp4"), None);
        assert_eq!(styled_name("basic", &json!({ "title": "..\\x" }), "mp4"), None);
        // an odd codec id never reaches the name
        let odd = json!({ "title": "T", "height": "720", "vcodec": "x y/z", "acodec": "ec-3" });
        assert_eq!(styled_name("nerdy", &odd, "mp4").as_deref(), Some("T_720p.mp4"));
    }

    #[test]
    fn numbered_name_counts_up_before_the_extension() {
        assert_eq!(numbered_name("Video Title.mp4", 1), "Video Title.mp4");
        assert_eq!(numbered_name("Video Title.mp4", 2), "Video Title (2).mp4");
        assert_eq!(numbered_name("Mr. T (1080p).mkv", 3), "Mr. T (1080p) (3).mkv");
        assert_eq!(numbered_name("noext", 2), "noext (2)");
    }

    #[test]
    fn error_text_keeps_only_the_human_part_of_the_last_error_line() {
        let out = "WARNING: something\nERROR: [youtube] Xk29aLq0P1c: Video unavailable. This video is private\n";
        assert_eq!(error_text(out).as_deref(), Some("Video unavailable. This video is private"));
        assert_eq!(error_text("ERROR: Unable to download webpage").as_deref(), Some("Unable to download webpage"));
        assert_eq!(error_text("all good"), None);
        assert_eq!(error_text(""), None);
    }

    #[test]
    fn ffmpeg_version_reads_the_banner() {
        let out = "ffmpeg version 6.1.1-essentials_build-www.gyan.dev Copyright (c) 2000-2023";
        assert_eq!(ffmpeg_version(out).as_deref(), Some("6.1.1"));
        assert_eq!(ffmpeg_version("built with gcc version 12.2.0"), Some("12.2.0".into())); // first match wins, like the JS regex
        assert_eq!(ffmpeg_version("no banner"), None);
    }

    // The shape of GitHub's answer for v0.1.16 (assets trimmed to the fields the app reads).
    fn release() -> Value {
        let base = "https://github.com/Khryz-Navarro/ArgonStream/releases";
        json!({
            "tag_name": "v0.1.16",
            "html_url": format!("{base}/tag/v0.1.16"),
            "assets": [
                { "name": "ArgonStream-0.1.16-amd64.deb", "size": 32_000_000,
                  "browser_download_url": format!("{base}/download/v0.1.16/ArgonStream-0.1.16-amd64.deb"),
                  "digest": "sha256:ed32ccf75c2ebbe3c620aa925d021f2579d28c24eef1e2d37c1771d8104fcc03" },
                { "name": "ArgonStream-Setup-0.1.16.exe", "size": 23_456_789,
                  "browser_download_url": format!("{base}/download/v0.1.16/ArgonStream-Setup-0.1.16.exe"),
                  "digest": "sha256:A0BE52FEC92F8871A331DBE9B02C49CE66987E04FA237CB2304D524543CAE464" }
            ]
        })
    }

    #[test]
    fn newer_release_offers_only_a_newer_version() {
        let r = newer_release("0.1.15", &release()).unwrap();
        assert_eq!(r["version"], "0.1.16");
        assert_eq!(r["page"], "https://github.com/Khryz-Navarro/ArgonStream/releases/tag/v0.1.16");
        let installer = &r["installer"];
        assert_eq!(
            installer["url"],
            "https://github.com/Khryz-Navarro/ArgonStream/releases/download/v0.1.16/ArgonStream-Setup-0.1.16.exe"
        );
        assert_eq!(installer["sha256"], "a0be52fec92f8871a331dbe9b02c49ce66987e04fa237cb2304d524543cae464");
        assert_eq!(installer["size"], 23_456_789);
        assert!(newer_release("0.1.9", &release()).is_some(), "versions compare as numbers, not text");
        for current in ["0.1.16", "0.1.17", "0.2.0", "1.0.0"] {
            assert_eq!(newer_release(current, &release()), None, "{current}");
        }
    }

    #[test]
    fn newer_release_ignores_odd_tags_and_foreign_pages() {
        for tag in ["0.1.17", "v0.1", "v0.1.17-beta", "v+0.1.17", "v0.1.17.1", "latest", ""] {
            let mut r = release();
            r["tag_name"] = json!(tag);
            assert_eq!(newer_release("0.1.0", &r), None, "{tag}");
        }
        let mut r = release();
        r["html_url"] = json!("https://example.com/Khryz-Navarro/ArgonStream/releases/tag/v0.1.16");
        assert_eq!(newer_release("0.1.0", &r), None);
        assert_eq!(newer_release("not a version", &release()), None);
    }

    #[test]
    fn newer_release_drops_an_installer_that_looks_wrong() {
        let exe = 1; // the installer's index in release()["assets"]
        let broken: [(&str, Value); 6] = [
            ("browser_download_url", json!("https://example.com/ArgonStream-Setup-0.1.16.exe")),
            ("browser_download_url", json!("https://github.com/Khryz-Navarro/ArgonStream/releases/download/v0.1.15/ArgonStream-Setup-0.1.16.exe")),
            ("digest", json!("md5:0123456789abcdef0123456789abcdef")),
            ("digest", json!("sha256:a0be52")),
            ("digest", Value::Null),
            ("size", Value::Null),
        ];
        for (field, value) in broken {
            let mut r = release();
            r["assets"][exe][field] = value.clone();
            let offer = newer_release("0.1.15", &r).unwrap();
            assert_eq!(offer["installer"], Value::Null, "{field} = {value}");
            assert_eq!(offer["version"], "0.1.16", "the banner still shows, with the page");
        }
        let mut r = release();
        r["assets"].as_array_mut().unwrap().remove(exe);
        assert_eq!(newer_release("0.1.15", &r).unwrap()["installer"], Value::Null);
    }
}
