# Sequential Analysis

The study on the influence of the number of particles was done using the `homogeneous7`
problem while the rest was done using the `CTS2_1` benchmark. The data used for this is 
located in the `reference_data/` folder, the code used for processing and plotting data 
is available in the `fastiron-stats` folder. It can be visualized using `column`:

```bash
column -s=';' -t < a_file.csv # Might have to change the separator to comma for scaling data
```




## Rustified Edition Comparison

As a reference, here is the old timer report from the `CTS2_1` benchmark with the figure of merit: 

```
[Timer Report]
Timer Name                       | Total number of calls      Shortest cycle (µs)    Average per cycle (µs)     Longest cycle (µs)    Total in section (µs)    Efficiency rating (%)
Section::Main                    |                     1          3.493894e8                3.493894e8             3.493894e8               3.493894e8                     100.0
Section::CycleInit               |                   100             5.045e3                    5.42e3               1.2292e4                5.42019e5                      44.1
Section::CycleTracking           |                   100          3.289216e6                3.485305e6             4.606365e6             3.48530558e8                      75.7
Section::CycleTrackingKernel     |                  5353          3.284158e6                3.479917e6             4.599556e6             3.47991801e8                      75.7
Section::CycleTrackingComm       |                  5353             5.023e3                   5.375e3                7.352e3                5.37509e5                      73.1
Section::CycleFinalize           |                   100                 0e0                       0e0                    0e0                      3e0                      46.4
Figure of merit: 1.072e6 [segments / cycle tracking time]
```

Here is the current version's timer report for the same problem, formatted using `column`, with the figure of merit:  

```
Timer Name                      #calls  Shortest(µs)  Average(µs)   Longest(µs)   Total(µs)     Efficiency(%)
Section::Main                   1       2.99980778e8  2.99980778e8  2.99980778e8  2.99980778e8  100.0
Section::PopulationControl      100     4.459e3       5.24e3        7.914e3       5.2403e5      66.2
Section::CycleTracking          100     2.619447e6    2.990111e6    3.141689e6    2.9901115e8   95.2
Section::CycleTrackingKernel    5353    2.612993e6    2.982677e6    3.133991e6    2.98267726e8  95.2
Section::CycleTrackingComm      5353    6.441e3       7.415e3       8.377e3       7.41538e5     88.5
Section::CycleSync              101     1.044e3       4.411e3       7.449e3       4.41133e5     59.2
Figure of merit: 1.250e6 [segments / cycle tracking time]
```

Here are a few percentages. They were computed using the [relative change][2] definition with the old 
values as reference:

- Overall execution time has _decreased_ by **14%**.
- Excluding `Main` and `CycleSync`/`CycleFinalize`, efficiency has _increased_, 
  on average, by **30,7%**. Note that efficiency is defined using an outlier 
  (longest cycle), hence this metric isn't very  significant and could be a fluke.
- The `CycleTracking` section execution time has _decreased_ by **14%**.
- The figure of merit, i.e. the number of segments computed per second, has 
  _increased_ by **16%**.

`CycleTracking` represent **99.7%** of the execution time. Time spent in 
`PopulationControl` and `CycleSync` are orders of magnitude lower.

The timers have been updated to better represent the new structure of the program: The 
`CycleFinalize`/`CycleInit` paradigm has been dropped for a single `CycleSync` section 
to both simplify the code and make it more flexible. They have been defined in order 
to keep some coherence between `PopulationControl` and `CycleInit`, repurpose 
`CycleFinalize` to fit the new structure and leave the `CycleTracking` untouched for 
figure of merit computations.

The population control functions have been integrated in the processing section of the 
program. An interesting observation is that the `CycleInit` and `PopulationControl` 
values have comparable values despite the deletion of the `MCBaseParticle` structure, 
hence a supposedly heavier particle initialization.


[1]: https://github.com/imrn99/fi_stats
[2]: https://en.wikipedia.org/wiki/Relative_change_and_difference#Definition