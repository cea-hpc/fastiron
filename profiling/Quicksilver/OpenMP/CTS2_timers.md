# Raw timers report

**Warning**: Quicksilver's timers report does not function properly in a sequential context.
The displayed values are actually the total time spent in each section.

```
Timer                       Cumulative   Cumulative   Cumulative   Cumulative   Cumulative   Cumulative
Name                            number    microSecs    microSecs    microSecs    microSecs   Efficiency
                              of calls          min          avg          max       stddev       Rating
main                                 1    1.808e+08    1.808e+08    1.808e+08    0.000e+00       100.00
cycleInit                          100    1.217e+06    1.217e+06    1.217e+06    0.000e+00       100.00
cycleTracking                      100    1.789e+08    1.789e+08    1.789e+08    0.000e+00       100.00
cycleTracking_Kernel             58833    1.776e+08    1.776e+08    1.776e+08    0.000e+00       100.00
cycleTracking_MPI                64186    1.278e+06    1.278e+06    1.278e+06    0.000e+00       100.00
cycleTracking_Test_Done              0    0.000e+00    0.000e+00    0.000e+00    0.000e+00         0.00
cycleFinalize                      200    3.377e+05    3.377e+05    3.377e+05    0.000e+00       100.00

```

**Figure of merit**: `2.088e+06 [Num Segments / Cycle Tracking Time]`
