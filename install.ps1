# MSSH for Windows Installation Script

# --- Configuration ---
$Repo = "Caterpolaris/mssh"
$InstallDir = "$env:USERPROFILE\.mssh\bin"
$ExeName = "mssh.exe"

# --- Functions ---
function Get-Arch {
    $arch = $env:PROCESSOR_ARCHITECTURE
    if ($arch -eq "AMD64") {
        return "x86_64"
    } elseif ($arch -eq "ARM64") {
        return "aarch64"
    } else {
        Write-Error "Unsupported architecture: $arch"
        exit 1
    }
}

function Get-Latest-Release {
    param($repo)
    $url = "https://api.github.com/repos/$repo/releases/latest"
    try {
        $release = Invoke-RestMethod -Uri $url
        return $release.tag_name
    } catch {
        Write-Error "Failed to get latest release from $url. Error: $_"
        exit 1
    }
}

# --- Main Script ---
Write-Host "Starting MSSH installation..."

# 1. Determine architecture
$arch = Get-Arch
Write-Host "Detected architecture: $arch"

# 2. Get latest release version from GitHub
$latest_version = Get-Latest-Release -repo $Repo
if (-not $latest_version) {
    exit 1
}
Write-Host "Latest version is $latest_version"


# 3. Construct download URL
$fileName = "mssh-${latest_version}-windows-${arch}.zip"
$downloadUrl = "https://github.com/$Repo/releases/download/$latest_version/$fileName"
$downloadPath = "$env:TEMP\$fileName"

Write-Host "Downloading from $downloadUrl"

# 4. Download the release
try {
    Invoke-WebRequest -Uri $downloadUrl -OutFile $downloadPath -UseBasicParsing
} catch {
    Write-Error "Failed to download $downloadUrl. Error: $_"
    exit 1
}

Write-Host "Download complete."

# 5. Create installation directory
if (-not (Test-Path -Path $InstallDir)) {
    Write-Host "Creating installation directory: $InstallDir"
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
}

# 6. Unzip the archive
Write-Host "Extracting files..."
try {
    Expand-Archive -Path $downloadPath -DestinationPath $InstallDir -Force
} catch {
    Write-Error "Failed to extract archive. Make sure you have .NET Framework 4.5 or later. Error: $_"
    exit 1
}

# 7. Add to user's PATH if not already there
Write-Host "Configuring PATH..."
$userPath = [System.Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$InstallDir*") {
    Write-Host "Adding $InstallDir to your PATH."
    $newPath = "$userPath;$InstallDir"
    [System.Environment]::SetEnvironmentVariable("Path", $newPath, "User")
    Write-Host "PATH updated. Please restart your terminal for the changes to take effect."
} else {
    Write-Host "$InstallDir is already in your PATH."
}

# 8. Cleanup
Remove-Item -Path $downloadPath -Force

Write-Host "`nInstallation complete!"
Write-Host "Run 'mssh' in a new terminal to start."
