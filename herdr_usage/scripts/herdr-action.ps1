function Find-FirstLogId($Value) {
    if ($null -eq $Value) { return $null }
    if ($Value.PSObject.Properties.Name -contains "log_id") { return [string]$Value.log_id }
    if ($Value -is [System.Collections.IEnumerable] -and $Value -isnot [string]) {
        foreach ($item in $Value) { $found = Find-FirstLogId $item; if ($found) { return $found } }
    } elseif ($Value -is [PSCustomObject]) {
        foreach ($property in $Value.PSObject.Properties) { $found = Find-FirstLogId $property.Value; if ($found) { return $found } }
    }
    return $null
}
function Invoke-HerdrChecked([string[]]$Arguments) {
    $result = & $herdrBinary @Arguments
    if ($LASTEXITCODE -ne 0) { throw "Herdr command failed: $($Arguments -join ' ')" }
    return $result
}
function Find-LogRecord($Value, [string]$LogId) {
    if ($null -eq $Value) { return $null }
    if ($Value.PSObject.Properties.Name -contains "log_id" -and $Value.log_id -eq $LogId) { return $Value }
    if ($Value -is [System.Collections.IEnumerable] -and $Value -isnot [string]) {
        foreach ($item in $Value) { $found = Find-LogRecord $item $LogId; if ($found) { return $found } }
    } elseif ($Value -is [PSCustomObject]) {
        foreach ($property in $Value.PSObject.Properties) { $found = Find-LogRecord $property.Value $LogId; if ($found) { return $found } }
    }
    return $null
}
function Invoke-PluginAction([string]$Action) {
    $response = (Invoke-HerdrChecked @("plugin", "action", "invoke", $Action, "--plugin", $pluginId)) -join "`n"
    $parsed = $response | ConvertFrom-Json
    $logId = Find-FirstLogId $parsed
    if (-not $logId) { throw "Could not identify the configuration action log" }
    $deadline = [DateTime]::UtcNow.AddSeconds(60)
    while ([DateTime]::UtcNow -lt $deadline) {
        $logs = ((Invoke-HerdrChecked @("plugin", "log", "list", "--plugin", $pluginId, "--limit", "50")) -join "`n") | ConvertFrom-Json
        $record = Find-LogRecord $logs $logId
        if ($record.status -eq "succeeded") { return }
        if ($record.status -and $record.status -ne "running") { throw "Plugin action $Action failed: $($record.status)" }
        Start-Sleep -Milliseconds 250
    }
    throw "Plugin action $Action timed out"
}
