$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $PSScriptRoot
Set-Location $root

$cargoBin = "$env:USERPROFILE\.cargo\bin"
$mingwBin = "D:\Toolchains\msys64\mingw64\bin"
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

npm.cmd run tauri:dev
exit $LASTEXITCODE

