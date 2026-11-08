#!/bin/sh
# Linux counterpart of get-ffmpeg.ps1: downloads the ffmpeg the app bundles into binaries/, where
# tauri.linux.conf.json's bundle.externalBin expects it. Run once before the first build.
# It's the ffmpeg-static project's Linux x64 build (release b6.1.1, which ships ffmpeg 7.0.2 for Linux).
set -eu
base=https://github.com/eugeneware/ffmpeg-static/releases/download/b6.1.1
dir="$(dirname "$0")/../binaries"
# "argonstream-ffmpeg", not "ffmpeg": the .deb installs it into /usr/bin, beside any system ffmpeg
bin="$dir/argonstream-ffmpeg-x86_64-unknown-linux-gnu"
mkdir -p "$dir"

curl -fL -o "$bin.gz" "$base/ffmpeg-linux-x64.gz"
curl -fsSL -o "$dir/ffmpeg.LICENSE" "$base/linux-x64.LICENSE"
gzip -df "$bin.gz"

# The packages ship this file, so check it is exactly the known build (the same bytes ffmpeg-static installs).
expected=e7e7fb30477f717e6f55f9180a70386c62677ef8a4d4d1a5d948f4098aa3eb99
actual=$(sha256sum "$bin" | cut -d ' ' -f 1)
if [ "$actual" != "$expected" ]; then
  rm -f "$bin"
  echo "ffmpeg checksum mismatch (got $actual), refusing to use it" >&2
  exit 1
fi
chmod +x "$bin"
"$bin" -hide_banner -version | head -n 1
