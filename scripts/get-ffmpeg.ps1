# Downloads the ffmpeg the app bundles into binaries/, where tauri.conf.json's bundle.externalBin
# expects it. Run once before the first build. It's the same build the Electron version gets from
# the ffmpeg-static npm package (release b6.1.1).
$ErrorActionPreference = 'Stop'
$base = 'https://github.com/eugeneware/ffmpeg-static/releases/download/b6.1.1'
$dir = Join-Path $PSScriptRoot '..\binaries'
$exe = Join-Path $dir 'ffmpeg-x86_64-pc-windows-msvc.exe'
New-Item -ItemType Directory -Force $dir | Out-Null

curl.exe -fL -o "$exe.gz" "$base/ffmpeg-win32-x64.gz"
if ($LASTEXITCODE) { throw "Couldn't download ffmpeg (curl exit code $LASTEXITCODE)" }
curl.exe -fsSL -o (Join-Path $dir 'ffmpeg.LICENSE') "$base/win32-x64.LICENSE"
if ($LASTEXITCODE) { throw "Couldn't download the ffmpeg license (curl exit code $LASTEXITCODE)" }

$in = [IO.File]::OpenRead("$exe.gz")
$out = [IO.File]::Create($exe)
try { (New-Object IO.Compression.GZipStream($in, [IO.Compression.CompressionMode]::Decompress)).CopyTo($out) }
finally { $out.Dispose(); $in.Dispose() }
Remove-Item "$exe.gz"
& $exe -hide_banner -version | Select-Object -First 1
