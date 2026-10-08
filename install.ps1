# Terminal Theatre installer for Windows (PowerShell).
#
#   irm https://raw.githubusercontent.com/Mirzabaig313/TERMINAL-THEATRE/main/install.ps1 | iex
#
# Downloads the Windows build from GitHub Releases, checks its SHA-256, puts
# theatre.exe in %LOCALAPPDATA%\Programs\TerminalTheatre and adds that folder
# to your PATH. Set $env:THEATRE_VERSION = "v0.2.0" for a specific release.

$ErrorActionPreference = "Stop"

$repo = "Mirzabaig313/TERMINAL-THEATRE"
$version = if ($env:THEATRE_VERSION) { $env:THEATRE_VERSION } else { "latest" }
$dir = Join-Path $env:LOCALAPPDATA "Programs\TerminalTheatre"
# one build: Windows on ARM runs it through its x64 emulation
$target = "x86_64-pc-windows-msvc"
$archive = "theatre-$target.zip"
$base = if ($version -eq "latest") {
    "https://github.com/$repo/releases/latest/download"
} else {
    "https://github.com/$repo/releases/download/$version"
}

$tmp = Join-Path ([System.IO.Path]::GetTempPath()) ("theatre-" + [System.Guid]::NewGuid())
New-Item -ItemType Directory -Path $tmp | Out-Null
try {
    Write-Host "Downloading Terminal Theatre ($version, $target)..."
    Invoke-WebRequest "$base/$archive" -OutFile "$tmp\$archive" -UseBasicParsing
    Invoke-WebRequest "$base/$archive.sha256" -OutFile "$tmp\$archive.sha256" -UseBasicParsing
    $want = ((Get-Content "$tmp\$archive.sha256" -Raw).Trim() -split '\s+')[0].ToLower()
    $got = (Get-FileHash "$tmp\$archive" -Algorithm SHA256).Hash.ToLower()
    if ($want -ne $got) { throw "checksum mismatch: the download is damaged or tampered with" }

    Expand-Archive "$tmp\$archive" -DestinationPath $tmp -Force
    New-Item -ItemType Directory -Path $dir -Force | Out-Null
    Copy-Item "$tmp\theatre.exe" "$dir\theatre.exe" -Force

    $path = [Environment]::GetEnvironmentVariable("Path", "User")
    if (($path -split ';') -notcontains $dir) {
        [Environment]::SetEnvironmentVariable("Path", "$path;$dir", "User")
        Write-Host "Added $dir to your PATH (open a new terminal to use it)."
    }
    Write-Host ""
    Write-Host "Installed: $dir\theatre.exe"
    Write-Host "Run it with: theatre   (Windows Terminal recommended)"
} finally {
    Remove-Item $tmp -Recurse -Force -ErrorAction SilentlyContinue
}
