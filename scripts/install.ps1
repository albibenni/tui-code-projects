$ErrorActionPreference = "Stop"

$BinaryName = "new-project-tui"
$ProjectRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path

Write-Host "Installing $BinaryName from $ProjectRoot..."

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Error "cargo was not found. Install Rust first: winget install -e --id Rustlang.Rustup"
    exit 1
}

cargo install --path $ProjectRoot --force
if ($LASTEXITCODE -ne 0) {
    Write-Error "Failed to install binary."
    exit $LASTEXITCODE
}

Write-Host "Binary installed to $env:USERPROFILE\.cargo\bin"

$AliasName = Read-Host "Enter desired alias (default: np)"
if ([string]::IsNullOrWhiteSpace($AliasName)) {
    $AliasName = "np"
}

if (-not (Test-Path $PROFILE)) {
    New-Item -ItemType File -Path $PROFILE -Force | Out-Null
}

$AliasLine = "Set-Alias -Name $AliasName -Value $BinaryName"
$AliasExists = Select-String -Path $PROFILE -Pattern "Set-Alias -Name $AliasName " -SimpleMatch -Quiet

if ($AliasExists) {
    Write-Host "Alias '$AliasName' already exists in $PROFILE. Skipping..."
} else {
    Add-Content -Path $PROFILE -Value ""
    Add-Content -Path $PROFILE -Value "# Alias for $BinaryName"
    Add-Content -Path $PROFILE -Value $AliasLine
    Write-Host "Added alias '$AliasName' to $PROFILE"
}

$CargoBin = Join-Path $env:USERPROFILE ".cargo\bin"
$PathEntries = ($env:Path -split ";") | ForEach-Object { $_.TrimEnd("\") }
$CargoBinNormalized = $CargoBin.TrimEnd("\")

if (-not ($PathEntries -contains $CargoBinNormalized)) {
    Write-Warning "$CargoBin is not in PATH. Add it in System Settings if needed."
}

Write-Host "Done. Restart PowerShell or run: . `$PROFILE"
