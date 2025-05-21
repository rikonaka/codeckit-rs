# Benchmarks

```bash
codeckit                time:   [4.5727 ms 4.6602 ms 4.7443 ms]
                        change: [+1.0834% +4.3456% +7.5095%] (p = 0.01 < 0.05)
                        Performance has regressed.
Found 21 outliers among 100 measurements (21.00%)
  1 (1.00%) low severe
  18 (18.00%) low mild
  2 (2.00%) high mild
```

```bash
base64                  time:   [3.1505 ms 3.2401 ms 3.3237 ms]
                        change: [-4.3543% -1.0684% +2.4691%] (p = 0.56 > 0.05)
                        No change in performance detected.
Found 28 outliers among 100 measurements (28.00%)
  24 (24.00%) low severe
  2 (2.00%) high mild
  2 (2.00%) high severe
```

Needless to say, my implementation is slower than the `base64` crates. But it is very easy to use.