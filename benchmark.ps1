# ldx Benchmark Script
# Runs ldx with varying thread counts across multiple directories
# Output: benchmark_results.csv
#
# Usage:
#   .\benchmark.ps1           # warm cache
#   .\benchmark.ps1 -Runs 20  # more runs for better stats

param(
    [int]$Runs = 10
)

$Dirs = @("C:\", "C:\Users\dylan", "C:\Program Files", "D:\")
$Threads = @(1, 2, 4, 6, 8, 10, 12, 14, 16)
$Results = @()

$totalRuns = $Dirs.Count * $Threads.Count * $Runs
$currentRun = 0

Write-Host "ldx Benchmark" -ForegroundColor Cyan
Write-Host "Runs per combination: $Runs" -ForegroundColor Cyan
Write-Host "Total runs: $totalRuns" -ForegroundColor Cyan
Write-Host ""

foreach ($dir in $Dirs)
{
    foreach ($t in $Threads)
    {
        $speeds = @()

        for ($i = 1; $i -le $Runs; $i++)
        {
            $currentRun++
            Write-Progress -Activity "Benchmarking" `
                -Status "Dir: $dir | Threads: $t | Run $i/$Runs" `
                -PercentComplete (($currentRun / $totalRuns) * 100)

            # Run ldx and capture output
            $output = & ldx -a -q -S -d $dir -t $t 2>&1

            # Parse entries/s from output line like:
            # "Scanned 945,428 entries | 512,944 entries/s | Threads: 8"
            $statsLine = $output | Where-Object { $_ -match "entries/s" }
            if ($statsLine -match "([\d,]+) entries/s")
            {
                $speed = [int]($matches[1] -replace ",", "")
                $speeds += $speed
            }
        }

        if ($speeds.Count -gt 0)
        {
            $avg    = [int](($speeds | Measure-Object -Average).Average)
            $min    = [int](($speeds | Measure-Object -Minimum).Minimum)
            $max    = [int](($speeds | Measure-Object -Maximum).Maximum)
            $sorted = $speeds | Sort-Object
            $median = if ($sorted.Count % 2 -eq 0)
            {
                [int](($sorted[$sorted.Count / 2 - 1] + $sorted[$sorted.Count / 2]) / 2)
            } else
            {
                [int]($sorted[[math]::Floor($sorted.Count / 2)])
            }

            $Results += [PSCustomObject]@{
                Directory = $dir
                Threads   = $t
                Runs      = $speeds.Count
                Avg       = $avg
                Median    = $median
                Min       = $min
                Max       = $max
                AllSpeeds = ($speeds -join ";")
            }

            Write-Host "  $dir | t=$t | avg=$avg | median=$median | min=$min | max=$max" -ForegroundColor Green
        }
    }
}

Write-Progress -Activity "Benchmarking" -Completed

$outFile = "benchmark_results.csv"
$Results | Export-Csv -Path $outFile -NoTypeInformation
Write-Host ""
Write-Host "Done! Results saved to $outFile" -ForegroundColor Cyan
