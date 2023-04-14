# Fastiron - `ParticleContainer` version

## Homogeneous7 - 10,000 Particles

```
[Timer Report]
Timer Name                       | Total number of calls      Shortest cycle (µs)    Average per cycle (µs)     Longest cycle (µs)    Total in section (µs)    Efficiency rating (%)
Section::Main                    |                     1          1.239098e6                1.239098e6             1.239098e6               1.239098e6                     100.0
Section::CycleInit               |                    10             1.025e3                   1.326e3                2.411e3                 1.3263e4                      55.0
Section::CycleTracking           |                    10           1.13916e5                 1.22115e5              1.48191e5               1.221153e6                      82.4
Section::CycleTrackingKernel     |                    10           1.13915e5                 1.22114e5              1.48191e5               1.221148e6                      82.4
Section::CycleTrackingComm       |                    10                 0e0                       0e0                    0e0                      1e0                      46.2
Section::CycleFinalize           |                    10                 0e0                       0e0                    0e0                      0e0                      81.6
```

## Homogeneous7 - 100,000 Particles

```
[Timer Report]
Timer Name                       | Total number of calls      Shortest cycle (µs)    Average per cycle (µs)     Longest cycle (µs)    Total in section (µs)    Efficiency rating (%)
Section::Main                    |                     1         1.1147265e7               1.1147265e7            1.1147265e7              1.1147265e7                     100.0
Section::CycleInit               |                    10              8.31e3                  1.0301e4               1.6712e4                1.03013e5                      61.6
Section::CycleTracking           |                    10          1.081816e6                1.103869e6             1.145991e6              1.1038692e7                      96.3
Section::CycleTrackingKernel     |                    10          1.081816e6                1.103868e6             1.145991e6              1.1038688e7                      96.3
Section::CycleTrackingComm       |                    10                 0e0                       0e0                    0e0                      1e0                      70.3
Section::CycleFinalize           |                    10                 0e0                       0e0                    0e0                      0e0                      90.3
```

## CTS2_1 - 40960 Particles

This version has much better scaling than the Naive version. This is mainly due to the removal 
of index searches from the original structure. This was necessary before because of the usage 
of `Option<T>` and possible "holes" in the storage. 

The simulation now completes in a very similar amount of time as Quicksilver. Most of the 
execution time is spent in the `CycleTracking` section, this is where performances are
similar.\
The performances in the `CycleInit` section do not match, however the section does not
represent a significant enough part of the program to matter in overall performance.

```
[Timer Report]
Timer Name                       | Total number of calls      Shortest cycle (µs)    Average per cycle (µs)     Longest cycle (µs)    Total in section (µs)    Efficiency rating (%)
Section::Main                    |                     1        3.87337653e8              3.87337653e8           3.87337653e8             3.87337653e8                     100.0
Section::CycleInit               |                   100             5.257e3                    5.72e3               1.0363e4                5.72041e5                      55.2
Section::CycleTracking           |                   100          3.299985e6                3.864395e6             5.220484e6             3.86439589e8                      74.0
Section::CycleTrackingKernel     |                  5353           3.29491e6                3.858629e6             5.213755e6             3.85862962e8                      74.0
Section::CycleTrackingComm       |                  5353             5.064e3                   5.753e3                8.999e3                5.75341e5                      63.9
Section::CycleFinalize           |                   100                 0e0                       0e0                    0e0                      3e0                      50.8
```