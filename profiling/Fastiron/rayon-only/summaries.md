# Fastiron - RuSeq version

The tallies and timers report can be visualized using column: 

```bash
column -s=';' -t < tallies_report.csv
column -s=';' -t < timers_report.csv
```

**Figure of merit**: `2.600e6 [segments / cycle tracking time]`


## Tokei Report

Note that this is prone to change since the implem is currently more of a 
DIY implem than clean code.

```
===============================================================================
 Language            Files        Lines         Code     Comments       Blanks
===============================================================================
 TOML                    1           31           21            3            7
-------------------------------------------------------------------------------
 Markdown                1           37            0           24           13
 |- Shell                1            2            2            0            0
 (Total)                             39            2           24           13
-------------------------------------------------------------------------------
 Rust                   46         7152         5871          424          857
 |- Markdown            39         1176           87          969          120
 (Total)                           8328         5958         1393          977
===============================================================================
 Total                  48         7220         5892          451          877
===============================================================================
```

## Previous Version Comparison

### Percents

Previous version is `1.2-RuSeq`. Current version corresponds to commit `de548f7`.

| Section              | Percent Change |
|----------------------|----------------|
| Total execution time |         -70.5% |
| PopulationControl    |           4.0% |
| CycleTracking        |         -70.7% |
| CycleSync            |           1.9% |

### Implementation

1. Separated `Tallies` from `MonteCarloUnit`. This may be avoidable since reference to tallies 
  are now immutable & they are updated using atomics.
2. Removed cross-section cache-ing from `MonteCarloUnit`. This was done to make the structure 
  read-only.
3. `Balance` structure now uses only `AtomicU64`. This allows us to get rid of unecessary locks
  on thetallies.
4. Mutex are used for extra container and send queue accesses. Those do not seem avoidable, so 
  minimizing lock-time should be the objective.

Some of these changes worsen the sequential performances, most importantly the cache removal.