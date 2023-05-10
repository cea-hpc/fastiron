# Raw timers report

```
Timer Name                       | Total number of calls      Shortest cycle (µs)    Average per cycle (µs)     Longest cycle (µs)    Total in section (µs)    Efficiency rating (%)
Section::Main                    |                     1        6.89469621e8              6.89469621e8           6.89469621e8             6.89469621e8                     100.0
Section::CycleInit               |                   100             7.172e3                   7.774e3                1.336e4                7.77484e5                      58.2
Section::CycleTracking           |                   100          6.106006e6                6.881901e6             7.343373e6             6.88190113e8                      93.7
Section::CycleTrackingKernel     |                  5353          6.101052e6                6.876278e6              7.33737e6             6.87627895e8                      93.7
Section::CycleTrackingComm       |                  5353             4.934e3                   5.602e3                6.124e3                5.60238e5                      91.5
Section::CycleFinalize           |                   100                 0e0                       0e0                    0e0                      7e0                      86.2
```

**Figure of merit**: `5.431e5 [segments / cycle tracking time]`