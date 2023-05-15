# Fastiron - RuSeq version

The tallies and timers report can be visualized using column: 

```bash
column -s=';' -t < tallies_report.csv
column -s=';' -t < timers_report.csv
```

**Figure of merit**: `7.630e5 [segments / cycle tracking time]`

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

The implementation of the `csv` output and stats tool will allow for automated performance 
comparison in the next versions.

## Tokei Report

```
===============================================================================
 Language            Files        Lines         Code     Comments       Blanks
===============================================================================
 TOML                    1           24           18            1            5
-------------------------------------------------------------------------------
 Markdown                1           30            0           20           10
 |- Shell                1            2            2            0            0
 (Total)                             32            2           20           10
-------------------------------------------------------------------------------
 Rust                   46         6755         5559          364          832
 |- Markdown            39         1159           83          960          116
 (Total)                           7914         5642         1324          948
===============================================================================
 Total                  48         6809         5577          385          847
===============================================================================

```