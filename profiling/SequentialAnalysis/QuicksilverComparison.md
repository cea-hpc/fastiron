 # Comparison between Quicksilver & Fastiron

## Performances

As a reference, here is the timer report from Quicksilver, of the `CTS2_1` benchmark with the figure of 
merit. **Note that Quicksilver's timers report does not function properly in a sequential context**. The 
displayed values are actually the total time spent in each section. We can divide it by 100 to obtain 
the average in each section per cycle (except for `main`).

```
Timer                       Cumulative   Cumulative   Cumulative   Cumulative   Cumulative   Cumulative
Name                            number    microSecs    microSecs    microSecs    microSecs   Efficiency
                              of calls          min          avg          max       stddev       Rating
main                                 1    7.750e+08    7.750e+08    7.750e+08    0.000e+00       100.00
cycleInit                          100    6.089e+05    6.089e+05    6.089e+05    0.000e+00       100.00
cycleTracking                      100    7.738e+08    7.738e+08    7.738e+08    0.000e+00       100.00
cycleTracking_Kernel             58833    7.729e+08    7.729e+08    7.729e+08    0.000e+00       100.00
cycleTracking_MPI                64186    8.419e+05    8.419e+05    8.419e+05    0.000e+00       100.00
cycleTracking_Test_Done              0    0.000e+00    0.000e+00    0.000e+00    0.000e+00         0.00
cycleFinalize                      200    3.289e+05    3.289e+05    3.289e+05    0.000e+00       100.00
Figure of merit: 4.830e+05 [Num Segments / Cycle Tracking Time]
```

Here is the current version's (_RuSeq_) timer report for the same problem, formatted using `column`, with 
the figure of merit:

```
Timer Name                      #calls  Shortest(µs)  Average(µs)   Longest(µs)   Total(µs)     Efficiency(%)
Section::Main                   1       4.91137709e8  4.91137709e8  4.91137709e8  4.91137709e8  100.0
Section::PopulationControl      100     5.741e3       6.626e3       1.3985e4      6.62613e5     47.4
Section::CycleTracking          100     4.313593e6    4.898239e6    5.154654e6    4.89823961e8  95.0
Section::CycleTrackingKernel    3122    4.310273e6    4.894426e6    5.150612e6    4.89442686e8  95.0
Section::CycleTrackingComm      3122    3.304e3       3.795e3       4.659e3       3.79571e5     81.5
Section::CycleSync              101     1.052e3       6.445e3       1.4027e4      6.44585e5     46.0
Figure of merit: 7.630e5 [segments / cycle tracking time]
```

Here are a few percentages. They were computed using the [relative change][2] definition with the old 
values as reference:

- Overall execution time has _decreased_ by **36.7%**.
- The `CycleTracking` section execution time has _decreased_ by **36.7%**.
- The figure of merit, i.e. the number of segments computed per second, has 
  _increased_ by **58%**.

In both programs, `CycleTracking` represent most of the execution time while the time spent in 
other sections are orders of magnitude lower.

## Behavior

## Implementation

Using [`tokei`][https://github.com/XAMPPRocky/tokei], we can have insights into the project size.
Here is the report for the `src/` folder of Quicksilver's repository: 

```
===============================================================================
 Language            Files        Lines         Code     Comments       Blanks
===============================================================================
 C++                    41         7963         5936          765         1262
 C++ Header             75         5016         3605          478          933
 Makefile                1          344           81          214           49
 SVG                     1          491          479            0           12
===============================================================================
 Total                 118        13814        10101         1457         2256
===============================================================================
```