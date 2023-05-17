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

Previous version is `1.2-RuSeq`. Current version corresponds to commit `de548f7`.

| Section              | Percent Change |
|----------------------|----------------|
| Total execution time |         -70.5% |
| PopulationControl    |           4.0% |
| CycleTracking        |         -70.7% |
| CycleSync            |           1.9% |