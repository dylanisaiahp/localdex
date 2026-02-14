cargo build --release
if ($LASTEXITCODE -ne 0)
{ exit 1
}
Copy-Item -Path .\target\release\localdex.exe -Destination C:\Tools\localdex.exe -Force
Copy-Item -Path C:\Tools\localdex.exe -Destination C:\Tools\ldx.exe -Force
if (Test-Path .\config.toml)
{
    Copy-Item -Path .\config.toml -Destination C:\Tools\config.toml -Force
}
Write-Host "Done! ldx updated." -ForegroundColor Green
