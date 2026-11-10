[CmdletBinding()]
param(
    [string]$Checkout = (Join-Path $PSScriptRoot '../work/cinnabar-builder'),
    [ValidateSet('debug','release','play')][string]$Profile = 'release',
    [switch]$Check
)

$ErrorActionPreference = 'Stop'
$fork = (Resolve-Path -LiteralPath $Checkout).Path
$bin = Join-Path $fork "target/$Profile"
$client = Join-Path $bin 'cinnabar.exe'
$userRoot = Join-Path $fork '.local/builder-user'
$required = @($client, (Join-Path $bin 'bedrock-core.exe'))
$missing = @($required | Where-Object { !(Test-Path -LiteralPath $_ -PathType Leaf) })

if ($Check) {
    [pscustomobject]@{
        Checkout = $fork
        Client = $client
        UserRoot = $userRoot
        BuildToolsMissing = @('go','make' | Where-Object { !(Get-Command $_ -ErrorAction SilentlyContinue) })
        MissingBinaries = $missing
    }
    return
}

if ($missing.Count) {
    $buildProfile = if ($Profile -eq 'debug') { 'dev' } else { $Profile }
    throw "Build from the checkout first with 'make play-build PROFILE=$buildProfile'. Missing: $($missing -join ', ')"
}

$previousRoot = [Environment]::GetEnvironmentVariable('CINNABAR_USER_ROOT', 'Process')
try {
    $env:CINNABAR_USER_ROOT = $userRoot
    & $client
    if ($LASTEXITCODE) { throw "Cinnabar exited with code $LASTEXITCODE." }
} finally {
    [Environment]::SetEnvironmentVariable('CINNABAR_USER_ROOT', $previousRoot, 'Process')
}
