$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $PSScriptRoot
$output = Join-Path $root "dist"
$required = @("Peeky.exe", "Peeky-Setup-x64.exe", "Peeky-Portable-x64.zip", "SHA256SUMS.txt")

foreach ($name in $required) {
    $path = Join-Path $output $name
    if (-not (Test-Path $path)) { throw "Missing release artifact: $name" }
    if ((Get-Item $path).Length -eq 0) { throw "Empty release artifact: $name" }
}

$checksumLines = Get-Content (Join-Path $output "SHA256SUMS.txt")
foreach ($name in @("Peeky.exe", "Peeky-Setup-x64.exe", "Peeky-Portable-x64.zip")) {
    $expected = ($checksumLines | Where-Object { $_ -like "*  $name" }).Split(" ")[0]
    $actual = (Get-FileHash (Join-Path $output $name) -Algorithm SHA256).Hash
    if ($expected -ne $actual) { throw "Checksum mismatch: $name" }
}

$loader = Join-Path $output "WebView2Loader.dll"
if (Test-Path $loader) {
    $expected = ($checksumLines | Where-Object { $_ -like "*  WebView2Loader.dll" }).Split(" ")[0]
    $actual = (Get-FileHash $loader -Algorithm SHA256).Hash
    if ($expected -ne $actual) { throw "Checksum mismatch: WebView2Loader.dll" }
}

$portable = Join-Path $output "Peeky-Portable-x64.zip"
if (-not (Test-Path $portable) -or (Get-Item $portable).Length -eq 0) {
    throw "Missing portable release archive."
}

Write-Host "Release artifacts and checksums are valid." -ForegroundColor Green
