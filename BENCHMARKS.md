# 📊 ldx Benchmarks

> **entries/s** = filesystem entries (files + directories) scanned per second. Higher is better.

Want to run your own? Use `./scripts/benchmark.sh` — outputs a CSV. Contributions welcome!

---

## 🖥️ Windows — i5-13400F, SSD

| Component | Spec |
|-----------|------|
| **CPU** | Intel Core i5-13400F (10 physical / 16 logical cores) |
| **RAM** | 32GB |
| **OS** | Windows 11 64-bit |
| **Storage** | 500GB SSD (C:) + 1TB SSD (D:) |

> 30 warm runs + 20 cold runs per configuration.

### Thread Scaling

| Directory | Real-world equivalent | Entries | Sweet spot | Peak speed |
|-----------|----------------------|---------|------------|------------|
| `C:\` | Full OS drive | 945k | 10 threads | 526,404/s |
| `C:\Users\dylan` | Home folder | 520k | 10–12 threads | 752,200/s |
| `C:\Program Files` | App installs | 86k | 16 threads | 1,641,700/s ✅ |
| `D:\` | Dev/data drive | 37k | 16 threads | 815,817/s |

<details>
<summary>Full thread scaling tables</summary>

#### `C:\` (945k entries) — Full OS Drive

| Threads | Warm Avg | Warm Median | Cold Avg | Cold Median |
|---------|----------|-------------|----------|-------------|
| 1 | 78,007/s | 70,511/s | 114,478/s | 125,380/s |
| 2 | 133,680/s | 133,169/s | 216,171/s | 216,116/s |
| 4 | 353,154/s | 354,510/s | 357,055/s | 357,571/s |
| 6 | 466,596/s | 467,436/s | 469,815/s | 468,738/s |
| 8 | 497,543/s | 498,302/s | 502,630/s | 499,327/s |
| **10** | **526,404/s** | **522,122/s** | **518,317/s** | **515,980/s** ✅ |
| 12 | 516,335/s | 516,611/s | 502,819/s | 502,919/s |
| 14 | 465,166/s | 467,256/s | 459,313/s | 460,702/s |
| 16 | 440,848/s | 437,274/s | 431,975/s | 430,744/s |

#### `C:\Users\dylan` (520k entries) — Home Folder

| Threads | Warm Avg | Warm Median | Cold Avg | Cold Median |
|---------|----------|-------------|----------|-------------|
| 1 | 148,005/s | 149,392/s | 163,079/s | 164,582/s |
| 2 | 263,567/s | 269,237/s | 273,696/s | 276,742/s |
| 4 | 450,877/s | 453,103/s | 462,633/s | 462,200/s |
| 6 | 603,919/s | 603,681/s | 618,517/s | 616,414/s |
| 8 | 647,473/s | 663,633/s | 693,803/s | 698,297/s |
| **10** | 731,840/s | 733,470/s | **743,145/s** | **739,625/s** ✅ |
| **12** | **747,916/s** | **752,200/s** | 721,559/s | 715,886/s |
| 14 | 726,490/s | 717,343/s | 699,147/s | 707,634/s |
| 16 | 685,486/s | 688,550/s | 647,972/s | 636,272/s |

#### `C:\Program Files` (86k entries) — App Installs

| Threads | Warm Avg | Warm Median | Cold Avg | Cold Median |
|---------|----------|-------------|----------|-------------|
| 1 | 290,563/s | 297,032/s | 306,049/s | 310,806/s |
| 2 | 509,440/s | 509,217/s | 529,256/s | 529,749/s |
| 4 | 841,393/s | 843,476/s | 867,909/s | 869,182/s |
| 6 | 1,125,433/s | 1,127,601/s | 1,148,363/s | 1,144,717/s |
| 8 | 1,240,273/s | 1,244,297/s | 1,295,313/s | 1,295,280/s |
| 10 | 1,336,502/s | 1,341,616/s | 1,372,124/s | 1,364,888/s |
| 12 | 1,444,864/s | 1,444,834/s | 1,461,014/s | 1,457,803/s |
| 14 | 1,553,522/s | 1,567,294/s | 1,573,560/s | 1,583,581/s |
| **16** | **1,618,843/s** | **1,617,653/s** | **1,641,700/s** | **1,646,164/s** ✅ |

#### `D:\` (37k entries) — Dev Drive

| Threads | Warm Avg | Warm Median | Cold Avg | Cold Median |
|---------|----------|-------------|----------|-------------|
| 1 | 153,285/s | 155,817/s | 156,267/s | 162,360/s |
| 2 | 263,950/s | 268,110/s | 278,817/s | 279,628/s |
| 4 | 443,506/s | 445,309/s | 453,977/s | 455,020/s |
| 6 | 583,630/s | 592,631/s | 593,988/s | 596,992/s |
| 8 | 641,250/s | 644,592/s | 661,291/s | 662,264/s |
| 10 | 675,445/s | 688,288/s | 707,140/s | 706,158/s |
| 12 | 685,660/s | 701,194/s | 750,338/s | 755,772/s |
| 14 | 704,422/s | 720,035/s | 791,790/s | 795,886/s |
| **16** | **749,385/s** | **744,496/s** | **815,817/s** | **817,494/s** ✅ |

</details>

### Cold vs Warm Cache (Windows SSD)

> Cold = first run after reboot. Warm = OS has cached metadata in RAM.

On SSD, cold and warm are nearly identical — NTFS caches metadata aggressively. The real bottleneck on Windows is the filesystem stack, not the drive.

---

## 🐧 Linux — Ryzen 7 5825U, CachyOS, NVMe

| Component | Spec |
|-----------|------|
| **CPU** | AMD Ryzen 7 5825U (8 cores / 16 threads, mobile) |
| **RAM** | 16GB |
| **OS** | CachyOS (BORE scheduler) |
| **Storage** | NVMe SSD (475GB) |

> 10 warm runs per configuration.

### Thread Scaling — `/home/dylan` (106k entries — Home Folder)

| Threads | Warm Avg | Warm Median | Efficiency |
|---------|----------|-------------|------------|
| 1 | 999,432/s | 1,000,164/s | 100% |
| 2 | 1,812,474/s | 1,813,606/s | 91% |
| 4 | 3,334,043/s | 3,334,091/s | 83% |
| 6 | 4,130,872/s | 4,057,115/s | 68% |
| 8 | 5,000,375/s | 4,981,887/s | 62% |
| 10 | 5,260,197/s | 5,355,170/s | 54% |
| 12 | 5,593,996/s | 5,715,085/s | 48% |
| 14 | 5,658,490/s | 5,718,812/s | 41% |
| **16** | **5,902,959/s** | **6,026,774/s** | 38% ✅ |

### Thread Scaling — `/usr` (System Dirs)

| Threads | Warm Avg | Warm Median |
|---------|----------|-------------|
| 1 | 1,200,595/s | 1,203,551/s |
| 4 | 3,828,810/s | 3,857,702/s |
| 8 | 5,566,237/s | 5,494,850/s |
| 12 | 5,784,335/s | 5,771,879/s |
| **16** | **6,274,455/s** | **6,211,923/s** ✅ |

> **Peak recorded: 7,065,858 entries/s** @ 16t on `/usr`

---

## ⚡ Linux vs Windows

| Metric | Windows (i5-13400F) | Linux (Ryzen 7 5825U) | Ratio |
|--------|--------------------|-----------------------|-------|
| Peak speed | 1,641,700/s | 7,065,858/s | **4.3x** |
| 16t home dir | ~685,000/s | 6,026,774/s | **8.8x** |
| Single thread | ~148,000/s | 1,000,164/s | **6.8x** |
| Cold full drive | ~94,000/s | 5,272,431/s | **56x** |

> Linux cold cache is faster than Windows warm cache. The gap is almost entirely the Windows filesystem stack (NTFS + Defender), not the CPU. These Linux numbers are from a **mobile** chip — desktop results pending.

---

## 💡 Thread Recommendations

| Dataset | Real-world example | Recommended threads |
|---------|--------------------|---------------------|
| < 100k entries | Project folder, app installs | 16 |
| 100k–500k entries | Home directory | 10–12 |
| 500k+ entries | Full OS drive | 10 |

> Use `-t N` to set thread count manually and `-S` to see your speed.

---

## 🖴 HDD / External Drives

Performance varies heavily by drive age, fragmentation, and fill level. If you have HDD benchmark results, run `./scripts/benchmark.sh` and open a PR with your CSV!

---

## 🍎 macOS Benchmarks

*Coming soon — contributions welcome! Run `./scripts/benchmark.sh` and open an issue with your results.*
