[CmdletBinding()]
param(
    [string]$Checkout = (Join-Path $PSScriptRoot '../work/cinnabar-builder')
)

$ErrorActionPreference = 'Stop'
$upstream = 'https://github.com/cinnabar-mc/cinnabar.git'
$base = 'dd7d7dd53a52c40491e0eb49b5bc700a4974191e'
$checkoutPath = $ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($Checkout)

if (Test-Path -LiteralPath $checkoutPath) {
    throw "Checkout already exists. Use it directly or choose a new path: $checkoutPath"
}

git clone --filter=blob:none --single-branch --branch dev --origin upstream --no-checkout $upstream $checkoutPath
if ($LASTEXITCODE) { throw 'Could not clone Cinnabar.' }

git -C $checkoutPath switch --create builder-features $base
if ($LASTEXITCODE) { throw "Could not check out pinned base $base." }

git -C $checkoutPath tag builder-base $base
if ($LASTEXITCODE) { throw 'Could not create the builder-base tag.' }

Write-Output "Cinnabar builder checkout: $checkoutPath"
