[CmdletBinding()]
param(
    [string]$BinaryPath,
    [string]$CargoPath = "cargo",
    [ValidateSet("all", "codex", "agy")][string[]]$Agent = @("all"),
    [switch]$Offline
)
$ErrorActionPreference = "Stop"
$pluginId = "herdr-agent-usage"
$herdrBinary = if ($env:HERDR_BIN_PATH) { $env:HERDR_BIN_PATH } else { (Get-Command herdr -ErrorAction Stop).Source }
. (Join-Path $PSScriptRoot "scripts/herdr-action.ps1")
if (-not $BinaryPath) {
    Push-Location $PSScriptRoot
    try {
        $buildArgs = @("build", "--release", "--locked")
        if ($Offline) { $buildArgs += "--offline" }
        & $CargoPath @buildArgs
        if ($LASTEXITCODE -ne 0) { throw "Rust build failed" }
        $BinaryPath = Join-Path $PSScriptRoot "target/release/herdr-agent-usage.exe"
    } finally { Pop-Location }
}
$BinaryPath = (Resolve-Path -LiteralPath $BinaryPath).Path
$destination = Join-Path $PSScriptRoot "target/release/herdr-agent-usage.exe"
$destinationDirectory = Split-Path $destination
New-Item -ItemType Directory -Path $destinationDirectory -Force | Out-Null
if ($BinaryPath -ne $destination) { Copy-Item -LiteralPath $BinaryPath -Destination $destination -Force }
Invoke-HerdrChecked @("plugin", "link", $PSScriptRoot, "--disabled") | Out-Null
$configDirectory = ((Invoke-HerdrChecked @("plugin", "config-dir", $pluginId)) -join "`n").Trim()
New-Item -ItemType Directory -Path $configDirectory -Force | Out-Null
$selection = if ($Agent -contains "all") { "all" } else { "only," + ($Agent -join ",") }
[IO.File]::WriteAllText((Join-Path $configDirectory "agents"), $selection + "`n", [Text.UTF8Encoding]::new($false))
Invoke-HerdrChecked @("plugin", "enable", $pluginId) | Out-Null
try {
    Invoke-PluginAction "configure"
} catch {
    # A failed installation must not keep running events against partial config.
    Invoke-HerdrChecked @("plugin", "disable", $pluginId) | Out-Null
    throw
}
Write-Output "Installed herdr-agent-usage. Existing StatusLine is retained for uninstall, never executed by the collector."
