param (
    [Parameter(Mandatory=$true)]
    [ValidateSet("working", "idle")]
    [string]$status,

    [Parameter(Mandatory=$false)]
    [string]$taskId = $null
)

$endpoint = "http://100.98.137.48:8765/api/agents/Jim/heartbeat"
$token = "ombra-uap-2026" # Correction: The user said "umbra-uap-2026"

# Ensure token is correct from user request
$token = "umbra-uap-2026"

$body = @{
    name = "Jim"
    status = $status
    activeTaskId = if ($status -eq "idle") { $null } else { $taskId }
} | ConvertTo-Json

$headers = @{
    "X-Agent-Token" = $token
    "Content-Type"  = "application/json"
}

try {
    Write-Host "Sending UAP heartbeat: Status=$status, TaskId=$taskId"
    $response = Invoke-RestMethod -Uri $endpoint -Method Post -Headers $headers -Body $body
    Write-Host "Successfully sent heartbeat."
    $response
} catch {
    Write-Error "Failed to send UAP heartbeat: $_"
}
