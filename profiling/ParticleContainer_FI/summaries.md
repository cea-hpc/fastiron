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

```

```