[CmdletBinding()]
param(
    [switch]$SkipChecks
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot
Set-Location $repoRoot

function Invoke-Step {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Name,
        [Parameter(Mandatory = $true)]
        [string]$Command
    )

    Write-Host ""
    Write-Host "==> $Name"
    Invoke-Expression $Command
}

if (-not $SkipChecks) {
    Invoke-Step -Name "frontend tests" -Command "npm test"
    Invoke-Step -Name "frontend build" -Command "npm run build"
    Invoke-Step -Name "rust tests" -Command "cargo test --manifest-path src-tauri/Cargo.toml"
    Invoke-Step -Name "rust clippy" -Command "cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings"
}

Invoke-Step -Name "tauri msi bundle" -Command "npx tauri build --bundles msi"

$msi = Get-ChildItem -Path "$repoRoot\src-tauri\target\release\bundle\msi\*.msi" |
    Sort-Object LastWriteTime -Descending |
    Select-Object -First 1

if (-not $msi) {
    throw "kein msi im bundle-ordner gefunden"
}

$hash = Get-FileHash -Path $msi.FullName -Algorithm SHA256

Write-Host ""
Write-Host "installer: $($msi.FullName)"
Write-Host "sha256:    $($hash.Hash)"
