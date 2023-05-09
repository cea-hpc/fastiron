# Fastiron - Naive version

**The shortest and longest cycle measures here were done using erroneous logic. The average and total are correct though.**

## Homogeneous7 - 10,000 Particles

```
Timer Name                       | Total number of calls      Shortest cycle (µs)    Average per cycle (µs)     Longest cycle (µs)    Total in section (µs)    Efficiency rating (%)
Section::Main                    |                     1          2.915979e6                2.915979e6             2.915979e6               2.915979e6                     100.0
Section::CycleInit               |                    10            2.3651e4                  2.4949e4               2.6028e4                2.49491e5                      95.9
Section::CycleTracking           |                    10            2.5374e5                 2.65761e5              2.71379e5                2.65761e6                      97.9
Section::CycleTrackingKernel     |                   110                 8e0                     9.4e1                 5.32e2                   9.49e2                      17.8
Section::CycleTrackingComm       |                   120              2.66e2                    2.83e2                  3.1e2                  2.839e3                      91.5
Section::CycleFinalize           |                    20              1.65e2                    1.83e2                 2.04e2                  1.834e3                      89.7
Figure of merit: 8.379e5 [segments / cycle tracking time]
```

## Homogeneous7 - 100,000 Particles

```
Timer Name                       | Total number of calls      Shortest cycle (µs)    Average per cycle (µs)     Longest cycle (µs)    Total in section (µs)    Efficiency rating (%)
Section::Main                    |                     1         7.5306993e7               7.5306993e7            7.5306993e7              7.5306993e7                     100.0
Section::CycleInit               |                    10          4.561948e6                4.831855e6             4.580761e6              4.8318557e7                     105.5
Section::CycleTracking           |                    10          2.614239e6                2.697098e6             2.960617e6              2.6970983e7                      91.1
Section::CycleTrackingKernel     |                   110               8.3e1                    1.71e2                 7.31e2                   1.71e3                      23.4
Section::CycleTrackingComm       |                   120             3.357e3                   3.502e3                3.571e3                 3.5029e4                      98.1
Section::CycleFinalize           |                    20              1.94e2                    2.23e2                 2.46e2                  2.231e3                      90.6
Figure of merit: 8.287e5 [segments / cycle tracking time]
```

## CTS2_1 - 40960 Particles

The performance scaling of the naive version being very poor, the simulation wasn't completed all 
the way through. The flamegraph was done using data gathered in the first 3 and 6 cycles of the 
simulation. 

It is interesting to note that the time spent in the `cycle_tracking` function grows 
as the simulation progresses, as showed by the tallies summary. There is no such 
behavior in Quicksilver.

```
cycle   |    start     source         rr        split       absorb      scatter      fission      produce    collision       escape       census      num_seg   scalar_flux   cycleInit (s)  cycleTracking (s)  cycleFinalize (s)
      0 |        0       4096          0        36864       154707      1408830       124185       248370      1687722            0        10438      3392593    1.243407e8  8.47633e-1        1.4892728e1            4.33e-4
      1 |    10438       4096          0        26438       183799      1605682       146693       293386      1936174            0         3866      3865559    1.585038e8  6.47045e-1        2.0176099e1            3.42e-4
      2 |     3866       4096          0        33010       171783      1512653       136828       273656      1821264            0         6017      3648944    1.564038e8  6.25159e-1        2.0433194e1            3.91e-4
      3 |     6017       4096          0        30841       178809      1566842       143287       286574      1888938            0         5432      3779548    1.638133e8  5.93636e-1        2.5217456e1            3.67e-4
      4 |     5432       4096        380        31492       178066      1570321       142603       285206      1890990            0         5177      3782522    1.647084e8  6.07219e-1        2.8406273e1            3.65e-4
      5 |     5177       4096         48        31693       176614      1553914       141209       282418      1871737            0         5513      3746309    1.590727e8  6.08588e-1        3.0126254e1             3.5e-4
```