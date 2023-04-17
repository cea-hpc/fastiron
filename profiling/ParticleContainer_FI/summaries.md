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

## CTS2_1 - 40960 Particles - `f64`

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
Section::Main                    |                     1          3.493894e8                3.493894e8             3.493894e8               3.493894e8                     100.0
Section::CycleInit               |                   100             5.045e3                    5.42e3               1.2292e4                5.42019e5                      44.1
Section::CycleTracking           |                   100          3.289216e6                3.485305e6             4.606365e6             3.48530558e8                      75.7
Section::CycleTrackingKernel     |                  5353          3.284158e6                3.479917e6             4.599556e6             3.47991801e8                      75.7
Section::CycleTrackingComm       |                  5353             5.023e3                   5.375e3                7.352e3                5.37509e5                      73.1
Section::CycleFinalize           |                   100                 0e0                       0e0                    0e0                      3e0                      46.4
```

## CTS2_1 - 40960 Particles - `f32`

```
[Timer Report]
Timer Name                       | Total number of calls      Shortest cycle (µs)    Average per cycle (µs)     Longest cycle (µs)    Total in section (µs)    Efficiency rating (%)
Section::Main                    |                     1        3.88161488e8              3.88161488e8           3.88161488e8             3.88161488e8                     100.0
Section::CycleInit               |                   100             3.669e3                   4.128e3               1.1514e4                4.12805e5                      35.8
Section::CycleTracking           |                   100          1.551045e6                 3.87512e6            1.3311673e7             3.87512005e8                      29.1
Section::CycleTrackingKernel     |                  5262          1.549607e6                3.872175e6            1.3310105e7             3.87217594e8                      29.1
Section::CycleTrackingComm       |                  5262              7.67e2                   2.934e3                5.245e3                2.93493e5                      56.0
Section::CycleFinalize           |                   100                 0e0                       0e0                    0e0                      3e0                      73.2
```

## Figure of merit comparison

This table contains for each test and simulation version the number of segments computed per 
second. The highest figure is **highlighted**. All tests beside the CTS2 benchmark were ran with 
100,000 particles. 

Test          | Quicksilver | `ParticleContainer` Fastiron - `f64` | `ParticleContainer` Fastiron - `f32`
--------------|-------------|--------------------------------------|-------------------------------------
CTS2_1        | 9.835e5     | **1.072e6**                          | 8.192e5
AllAbsorb     | 1.278e6     | 1.857e6                              | **1.904e6**
AllEscape     | **4.439e6** | 4.009e6                              | 3.214e6
AllScattering | 2.083e6     | 2.091e6                              | **2.132e6**
NoCollision   | 3.178e6     | **3.199e6**                          | 2.564e6
NoFission     | 1.794e6     | 1.873e6                              | **1.998e6**
Homogeneous   | 1.995e+06   | 2.030e6                              | **2.160e6**