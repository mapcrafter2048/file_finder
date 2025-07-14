# File Finder & Grep Tool - PowerShell Script
# This script allows you to run the tool from anywhere

$scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Definition
$exePath = Join-Path $scriptPath "target\release\file_finder.exe"

if (Test-Path $exePath) {
    & $exePath @args
} else {
    Write-Host "Error: file_finder.exe not found. Please build the project first with 'cargo build --release'" -ForegroundColor Red
    Write-Host "Expected path: $exePath" -ForegroundColor Yellow
}
