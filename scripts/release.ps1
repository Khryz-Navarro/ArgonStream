# .\scripts\release.ps1 [patch|minor|major]
# On a feature branch with everything committed: bump the version in Cargo.toml (once per branch,
# relative to main), push the branch, and open a pull request into main. Merging the PR makes GitHub
# publish the release (.github/workflows/release.yml builds the installer).
param([ValidateSet('patch', 'minor', 'major')] [string]$Level = 'patch')

function Fail([string]$msg) { Write-Host "release: $msg" -ForegroundColor Red; exit 1 }
# Judged by exit code: Windows PowerShell turns a tool's normal stderr output (git and cargo progress) into
# errors when the script's output is redirected, so $? is unreliable for them.
function Found([string]$cmd) { [bool](Get-Command $cmd -ErrorAction SilentlyContinue) }
function Run {
  if (-not (Found $args[0])) { Fail "``$($args[0])`` isn't installed or isn't on PATH." }
  & $args[0] @($args | Select-Object -Skip 1)
  if ($LASTEXITCODE) { Fail "``$args`` failed." }
}
# The [package] version: Cargo.toml's first line that starts with `version =`.
function VersionOf([string]$toml) { if ($toml -match '(?m)^version = "(\d+\.\d+\.\d+)"') { [version]$Matches[1] } }

git config core.hooksPath .githooks # turns on the pre-push check
$branch = git branch --show-current
if (-not $branch -or $branch -eq 'main') { Fail 'changes reach main through a pull request. Create a branch first: git switch -c feat/my-change (or bug/my-fix)' }
if (git status --porcelain) { Fail 'commit your changes first.' }

Run git fetch origin main
$main = VersionOf ((git show origin/main:Cargo.toml 2>$null) -join "`n") # none on a brand-new repo
$current = VersionOf ([IO.File]::ReadAllText("$PWD\Cargo.toml"))
if (-not $current) { Fail 'no version found in Cargo.toml.' }

if (-not $main -or $current -gt $main) {
  Write-Host "release: branch is already at v$current (main is $(if ($main) { "v$main" } else { 'unversioned' })), not bumping again."
} else {
  $base = $main # not the branch's own version: a branch cut before main moved on would bump to main's
  $new = switch ($Level) {
    'major' { "$($base.Major + 1).0.0" }
    'minor' { "$($base.Major).$($base.Minor + 1).0" }
    'patch' { "$($base.Major).$($base.Minor).$($base.Build + 1)" }
  }
  $toml = [IO.File]::ReadAllText("$PWD\Cargo.toml")
  [IO.File]::WriteAllText("$PWD\Cargo.toml", ([regex]'(?m)^version = "[^"]*"').Replace($toml, "version = `"$new`"", 1))
  if (Found cargo) { cargo update --workspace --offline } # writes the new version into Cargo.lock
  if (-not (Found cargo) -or $LASTEXITCODE) { git checkout -- Cargo.toml Cargo.lock; Fail '`cargo update` failed, so nothing was changed. Is Rust installed and on PATH?' }
  Run git add Cargo.toml Cargo.lock
  Run git commit -m "Release v$new" # the tag is created by the Release workflow after the merge
  $current = $new
}

Run git push -u origin HEAD

$pr = gh pr view --json url --jq .url 2>$null # nothing yet for a new branch
if ($LASTEXITCODE -eq 0 -and $pr) { Write-Host "release: pull request already open: $pr" }
else { Run gh pr create --base main --head $branch --fill }
Write-Host "release: merge the pull request to publish v$current (the Release workflow builds the installer)."
