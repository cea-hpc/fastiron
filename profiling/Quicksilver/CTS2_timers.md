# Raw timers report

**Warning**: Quicksilver's timers report does not function properly in a sequential context.
The displayed values are actually the total time spent in each section.

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
```

**Figure of merit**: `4.830e+05 [Num Segments / Cycle Tracking Time]`
