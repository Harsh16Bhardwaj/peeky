param([switch]$SkipTests)

$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $PSScriptRoot
$output = Join-Path $root "dist"
Set-Location $root

if ($env:PEEKY_CARGO_TARGET_DIR) {
    $env:CARGO_TARGET_DIR = $env:PEEKY_CARGO_TARGET_DIR
} else {
    $env:CARGO_TARGET_DIR = "D:\Peeky-build\target"
}
New-Item -ItemType Directory -Path $env:CARGO_TARGET_DIR -Force | Out-Null
$buildTemp = Join-Path ([IO.Path]::GetPathRoot($env:CARGO_TARGET_DIR)) "Peeky-temp"
New-Item -ItemType Directory -Path $buildTemp -Force | Out-Null
$env:TEMP = $buildTemp
$env:TMP = $buildTemp

$cargoBin = "$env:USERPROFILE\.cargo\bin"
$mingwBin = "D:\Toolchains\msys64\mingw64\bin"
$releaseRoot = Join-Path $env:CARGO_TARGET_DIR "release"
$packageVersion = (Get-Content (Join-Path $root "package.json") -Raw | ConvertFrom-Json).version
if (-not (Get-Command cl.exe -ErrorAction SilentlyContinue)) {
    if (-not (Test-Path (Join-Path $mingwBin "gcc.exe"))) {
        throw "No Windows C++ linker is available. Run setup-dev.ps1 first."
    }
    $env:RUSTUP_TOOLCHAIN = "stable-x86_64-pc-windows-gnu"
    $cleanPath = $env:Path -split ';' | Where-Object {
        $_ -and $_ -notlike 'C:\MinGW*' -and $_ -notlike '*Git\usr\bin*'
    }
    $env:Path = "$cargoBin;$mingwBin;" + ($cleanPath -join ';')
}

if (-not (Test-Path "node_modules")) { npm.cmd install }
if (-not $SkipTests) {
    npm.cmd test
    cargo test --manifest-path src-tauri\crates\peeky-core\Cargo.toml
    cargo check --manifest-path src-tauri\Cargo.toml
}

npm.cmd run tauri:build

New-Item -ItemType Directory -Path $output -Force | Out-Null
$app = Join-Path $releaseRoot "Peeky.exe"
$setup = Join-Path $releaseRoot "bundle\nsis\Peeky_${packageVersion}_x64-setup.exe"
if (-not (Test-Path $app)) { throw "Peeky.exe was not produced." }
if (-not (Test-Path $setup)) { throw "The NSIS installer for version $packageVersion was not produced." }

Copy-Item $app (Join-Path $output "Peeky.exe") -Force
Copy-Item $setup (Join-Path $output "Peeky-Setup-x64.exe") -Force
$loader = Join-Path $releaseRoot "WebView2Loader.dll"
if (Test-Path $loader) {
    Copy-Item $loader (Join-Path $output "WebView2Loader.dll") -Force
} else {
    Remove-Item (Join-Path $output "WebView2Loader.dll") -Force -ErrorAction SilentlyContinue
}

$portable = Join-Path $output "Peeky-Portable-x64.zip"
Remove-Item $portable -Force -ErrorAction SilentlyContinue
$portableFiles = @((Join-Path $output "Peeky.exe"))
if (Test-Path (Join-Path $output "WebView2Loader.dll")) {
    $portableFiles += Join-Path $output "WebView2Loader.dll"
}

$archiveCreated = $false
for ($attempt = 1; $attempt -le 5 -and -not $archiveCreated; $attempt++) {
    try {
        Compress-Archive -LiteralPath $portableFiles -DestinationPath $portable -CompressionLevel Optimal
        $archiveCreated = $true
    } catch {
        Remove-Item $portable -Force -ErrorAction SilentlyContinue
        if ($attempt -eq 5) { throw }
        Start-Sleep -Seconds 2
    }
}

$artifacts = @(
    Join-Path $output "Peeky.exe"
    Join-Path $output "Peeky-Setup-x64.exe"
    $portable
)
if (Test-Path (Join-Path $output "WebView2Loader.dll")) {
    $artifacts += Join-Path $output "WebView2Loader.dll"
}
$checksums = foreach ($artifact in $artifacts) {
    $hash = Get-FileHash $artifact -Algorithm SHA256
    "$($hash.Hash)  $([IO.Path]::GetFileName($artifact))"
}
Set-Content -Path (Join-Path $output "SHA256SUMS.txt") -Value $checksums -Encoding ascii
Write-Host "Release artifacts written to $output" -ForegroundColor Green
