[CmdletBinding()]
param()
$ErrorActionPreference = "Stop"
$pluginId = "herdr-agent-usage"
$herdrBinary = if ($env:HERDR_BIN_PATH) { $env:HERDR_BIN_PATH } else { (Get-Command herdr -ErrorAction Stop).Source }
. (Join-Path $PSScriptRoot "scripts/herdr-action.ps1")
Invoke-PluginAction "uninstall"
Invoke-HerdrChecked @("plugin", "disable", $pluginId) | Out-Null
Invoke-HerdrChecked @("plugin", "unlink", $pluginId) | Out-Null
Write-Output "Removed herdr-agent-usage and restored its managed configuration."
