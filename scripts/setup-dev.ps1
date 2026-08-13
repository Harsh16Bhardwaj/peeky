param([switch]$SkipDependencies)

$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $PSScriptRoot
Set-Location $root

if (-not (Get-Command node -ErrorAction SilentlyContinue)) {
    throw "Node.js 20 or newer is required."
}

if (-not (Get-Command rustup -ErrorAction SilentlyContinue)) {
    $installer = Join-Path $env:TEMP "rustup-init.exe"
    Invoke-WebRequest "https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe" -OutFile $installer
    & $installer -y --default-toolchain stable --default-host x86_64-pc-windows-msvc
    $env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
}

rustup default stable-msvc

$cl = Get-Command cl.exe -ErrorAction SilentlyContinue
if (-not $cl) {
    $vswhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
    if (Test-Path $vswhere) {
        $install = & $vswhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
        if ($install) {
            $devShell = Join-Path $install "Common7\Tools\Launch-VsDevShell.ps1"
            if (Test-Path $devShell) { & $devShell -Arch amd64 -HostArch amd64 }
        }
    }
}

if (-not (Get-Command cl.exe -ErrorAction SilentlyContinue)) {
    $msysRoot = "D:\Toolchains\msys64"
    $mingwBin = Join-Path $msysRoot "mingw64\bin"
    if (-not (Test-Path (Join-Path $mingwBin "gcc.exe"))) {
        New-Item -ItemType Directory -Path "D:\Toolchains" -Force | Out-Null
        $msysInstaller = Join-Path $env:TEMP "msys2-base.sfx.exe"
        Invoke-WebRequest "https://repo.msys2.org/distrib/x86_64/msys2-base-x86_64-20260611.sfx.exe" -OutFile $msysInstaller
        & $msysInstaller -y "-oD:\Toolchains"
        & (Join-Path $msysRoot "usr\bin\bash.exe") -lc "pacman -Sy --noconfirm mingw-w64-x86_64-gcc"
    }
    rustup toolchain install stable-x86_64-pc-windows-gnu --profile minimal
    Write-Host "MSVC was unavailable; configured the isolated GNU Windows toolchain." -ForegroundColor Yellow
}

if (-not $SkipDependencies) {
    npm.cmd install
}

Write-Host "Peeky development environment is ready." -ForegroundColor Green
