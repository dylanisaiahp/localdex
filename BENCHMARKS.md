# 📊 ldx Benchmarks

> **entries/s** = filesystem entries (files + directories) scanned per second. Higher is better.

Run your own: `./scripts/dev.sh benchmark` — outputs a `.md` report. Contributions welcome!

---

## 🖥️ Windows — i5-13400F

| Component | Spec |
|-----------|------|
| **CPU** | Intel Core i5-13400F (10 physical / 16 logical cores) |
| **RAM** | 32GB |
| **OS** | Windows 11 64-bit |
| **Storage** | 500GB SSD (C:) + 1TB SSD (D:) |
| **Engine** | ldx v0.2.0 + parex v0.1.0 |

> 20 warm runs per combination.

### Thread Scaling Summary

| Directory | Entries | Sweet spot | Peak speed |
|-----------|---------|------------|------------|
| `C:\Program Files` | 97k | t=16 | **1,491,712/s** ✅ |
| `C:\Users\dylan` | 40k | t=12 | **733,677/s** ✅ |
| `D:\` | 40k | t=14–16 | **702,785/s** ✅ |
| `C:\` | 954k | t=12 | **490,109/s** ✅ |

### `C:\Program Files` — App Installs (97k entries)

| Threads | Avg | Median | Min | Max |
|---------|-----|--------|-----|-----|
| t=1 | 240,009 | 242,637 | 196,292 | 244,792 |
| t=2 | 437,303 | 439,774 | 415,451 | 447,906 |
| t=4 | 757,654 | 757,734 | 731,520 | 776,045 |
| t=6 | 1,037,236 | 1,039,680 | 967,242 | 1,070,880 |
| t=8 | 1,146,591 | 1,145,172 | 1,109,722 | 1,178,194 |
| t=10 | 1,234,493 | 1,234,333 | 1,191,375 | 1,284,442 |
| t=12 | 1,328,615 | 1,331,237 | 1,293,939 | 1,375,898 |
| t=14 | 1,421,184 | 1,421,973 | 1,363,090 | 1,453,836 |
| **t=16** | **1,491,712** | **1,482,665** | **1,431,841** | **1,539,872** ✅ |

### `C:\Users\dylan` — Home Folder (40k entries)

| Threads | Avg | Median | Min | Max |
|---------|-----|--------|-----|-----|
| t=1 | 105,205 | 102,636 | 94,254 | 131,729 |
| t=2 | 241,015 | 241,552 | 233,698 | 244,068 |
| t=4 | 415,172 | 415,168 | 409,350 | 423,978 |
| t=6 | 565,558 | 564,892 | 549,562 | 577,262 |
| t=8 | 622,446 | 625,437 | 576,332 | 643,910 |
| t=10 | 684,701 | 687,959 | 652,094 | 705,739 |
| **t=12** | **733,677** | **736,621** | **694,880** | **773,670** ✅ |
| t=14 | 696,897 | 701,536 | 607,756 | 749,477 |
| t=16 | 654,150 | 662,517 | 573,265 | 711,714 |

### `D:\` — Dev Drive (40k entries)

| Threads | Avg | Median | Min | Max |
|---------|-----|--------|-----|-----|
| t=1 | 133,841 | 134,230 | 122,904 | 136,342 |
| t=2 | 239,490 | 240,564 | 220,721 | 243,317 |
| t=4 | 408,666 | 408,700 | 399,339 | 423,104 |
| t=6 | 539,889 | 541,911 | 507,730 | 562,466 |
| t=8 | 569,958 | 581,289 | 341,281 | 614,747 |
| t=10 | 624,108 | 625,319 | 582,535 | 659,347 |
| t=12 | 647,755 | 645,707 | 586,600 | 700,452 |
| t=14 | 699,710 | 694,386 | 658,563 | 760,308 |
| **t=16** | **702,785** | **718,690** | **380,718** | **789,258** ✅ |

### `C:\` — Full OS Drive (954k entries)

| Threads | Avg | Median | Min | Max |
|---------|-----|--------|-----|-----|
| t=1 | 50,330 | 47,233 | 46,161 | 85,940 |
| t=2 | 107,640 | 108,227 | 100,750 | 112,833 |
| t=4 | 318,970 | 319,139 | 313,636 | 320,618 |
| t=6 | 426,060 | 426,804 | 404,269 | 433,254 |
| t=8 | 465,919 | 466,973 | 438,564 | 482,274 |
| t=10 | 479,445 | 480,552 | 416,383 | 502,805 |
| **t=12** | **490,109** | **486,225** | **474,602** | **524,093** ✅ |
| t=14 | 463,513 | 463,306 | 427,750 | 499,343 |
| t=16 | 430,815 | 434,328 | 401,978 | 455,895 |

> C:\ peaks at t=12 then drops — more threads causes contention on a large, deeply nested NTFS drive.

---

## 💡 Thread Recommendations

| Dataset size | Real-world example | Recommended |
|-------------|-------------------|-------------|
| < 100k entries | App installs, project folder | t=16 |
| 100k–500k entries | Home directory | t=10–12 |
| 500k+ entries | Full OS drive | t=10–12 |

Use `-t N` to set thread count and `-S` to see your speed.

---

## 🐧 Linux Benchmarks

*Contributions welcome! Run `./scripts/dev.sh benchmark` and open a PR with the `.md` output.*

---

## 🍎 macOS Benchmarks

*Contributions welcome! Run `./scripts/dev.sh benchmark` and open a PR with the `.md` output.*

---

## 🖴 HDD / External Drives

Performance varies heavily by drive age and fragmentation. HDD benchmark contributions especially welcome.
