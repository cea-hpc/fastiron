# Sequential Analysis

The study of the influence of the number of particles was done using the `homogeneous7`
problem while the rest was done using the `CTS2_1` benchmark. The data recorded in the 
`csv` files can be viewed using `column`: 

```bash
column -s=';' -t < a_file.csv
```

Note that this analysis precedes some additionnal changes to be done to the code 
before release (TODO: add link to PRs?).

## Correlation Study

The goal of this short study is to identify how events influence the time spent in each
section. For this, we can define random variables (RV) using the value taken by tallies 
at each cycle.

The events of interest are the following: 

- `source`, `rr`, `split`: to be linked with `ppControl (s)` and `cycleSync (s)` 
sections.
- `absorb`, `scatter`, `fission`, `collision`, `census`, `escape`: to be linked with 
`cycleTracking (s)` section.

The sample space is the same for all RV: the cycle indexes. The measurable space is
positive integers for event RV, positive reals for section RV.

The data used for this is located in the `CTS2_1_data/` folder, the code used for 
processing and plotting data is located [here][1]. The results are showed below: 

![tracking](figures/heatmap_tracking.png)

![popsync](figures/heatmap_popsync.png)


[1]: https://github.com/imrn99/fi_stats

## Scaling

Data for the scaling study can be recorded using the `scaling.sh` script. By looking at 
the printed output of the program, one can see already that the figure of merit does not
vary significantly. This means that the time taken to compute a segment for a particle
-- including particle look-up & update -- is independent from the total number of particle, 
which is expected.

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
Figure of merit: 1.072e6e6 [segments / cycle tracking time]
```

Here is the current version timer report, formatted using `column`:  

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
