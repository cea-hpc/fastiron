# Fastiron - Naive version

## Homogeneous7 - 10,000 Particles

```
[Timer Report]
Timer Name                       | Total number of calls      Shortest cycle (µs)    Average per cycle (µs)     Longest cycle (µs)    Total in section (µs)    Efficiency rating (%)
Section::Main                    |                     1          1.329937e6                1.329937e6             1.329937e6               1.329937e6                     100.0
Section::CycleInit               |                    10            1.0267e4                  1.0764e4               1.1212e4                1.07643e5                      96.0
Section::CycleTracking           |                    10           1.13194e5                 1.21641e5              1.50017e5               1.216416e6                      81.1
Section::CycleTrackingKernel     |                   110                 7e0                     4.9e1                 2.23e2                   4.94e2                      22.1
Section::CycleTrackingComm       |                   120              1.73e2                    1.98e2                 2.67e2                  1.986e3                      74.4
Section::CycleFinalize           |                    20              2.04e2                    2.52e2                    3e2                  2.525e3                      84.1
```

## Homogeneous7 - 100,000 Particles

```
[Timer Report]
Timer Name                       | Total number of calls      Shortest cycle (µs)    Average per cycle (µs)     Longest cycle (µs)    Total in section (µs)    Efficiency rating (%)
Section::Main                    |                     1          3.411096e7                3.411096e7             3.411096e7               3.411096e7                     100.0
Section::CycleInit               |                    10          2.197272e6                2.211564e6             2.217284e6              2.2115647e7                      99.7
Section::CycleTracking           |                    10          1.173878e6                1.197086e6             1.233285e6              1.1970862e7                      97.1
Section::CycleTrackingKernel     |                   110               5.4e1                    1.01e2                 4.11e2                  1.012e3                      24.6
Section::CycleTrackingComm       |                   120             2.036e3                   2.256e3                2.929e3                 2.2567e4                      77.0
Section::CycleFinalize           |                    20             1.478e3                   1.551e3                 1.62e3                 1.5512e4                      95.7
```

## CTS2_1 - 40960 Particles

The performance scaling of the naive version being very poor, the simulation wasn't completed all 
the way through. The flamegraph was done using data gathered in the first 3 and 6 cycles of the 
simulation. It is interesting to note that the time spent in the `cycle_tracking` function grows 
as the simulation progresses, as showed by the tallies summary : 

```
cycle   |    start     source         rr        split       absorb      scatter      fission      produce    collision       escape       census      num_seg   scalar_flux   cycleInit (s)  cycleTracking (s)  cycleFinalize (s)
      0 |        0       4096          0        36864       154707      1408830       124185       248370      1687722            0        10438      3392593    1.243407e8  3.28279e-1         8.489814e0            3.39e-4
      1 |    10438       4096          0        26438       183799      1605682       146693       293386      1936174            0         3866      3865559    1.585038e8  2.92625e-1         1.470643e1            5.22e-4
      2 |     3866       4096          0        33010       171783      1512653       136828       273656      1821264            0         6017      3648944    1.564038e8  2.86444e-1        1.6751339e1            4.77e-4
      3 |     6017       4096          0        30841       178809      1566842       143287       286574      1888938            0         5432      3779548    1.638133e8  2.73205e-1        2.2295347e1             4.9e-4
      4 |     5432       4096        380        31492       178066      1570321       142603       285206      1890990            0         5177      3782522    1.647084e8  2.84283e-1        2.5831261e1            4.57e-4
      5 |     5177       4096         48        31693       176614      1553914       141209       282418      1871737            0         5513      3746309    1.590727e8  2.95563e-1        2.6705336e1            3.94e-4

```