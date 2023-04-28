# Sequential Analysis

The study of the influence of the number of particles was done using the `homogeneous7`
problem while the rest was done using the `CTS2_1` benchmark. The data recorded in the 
`csv` files can be viewed using `column`: 

```bash
column -s=';' -t < a_file.csv
```

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

The data used for this is located in the `CTS2_1_data/` folder.

## Scaling

Data for the scaling study can be recorded using the `scaling.sh` script. By looking at 
the printed output of the program, one can see already that the figure of merit does not
vary significantly. This means that the time taken to compute a segment for a particle
--including particle look-up & update-- is independent from the total number of particle, 
which is expected.

## Rustified Edition Comparison