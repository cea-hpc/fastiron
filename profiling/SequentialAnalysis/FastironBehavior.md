# Behavior of Fastiron


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

The data used for this is located in the `reference_data/` folder, the code used for 
processing and plotting data is available in the `fastiron-stats` folder. For the 
analysis, the coefficients might be referred to by the name of their variables when 
it's not ambiguous.

### `CycleTracking`

![tracking](figures/FI_64/heatmap_tracking.png)

This figure yields a number of interesting points. First, the `NumSeg` variable is 
not the one with the highest correlation coefficient. This means that while the 
number of computed segment is (heavily) linked to the total time spent in the tracking
section, it is not the most important factor.

The `Census` variable has a negative coefficient, meaning that the time spent in the 
tracking section, overall, scales negatively with the number of particle reaching census.
This is coherent as a particle reaching census means that no more segments will be 
computed for it, reducing the "time left to spend" in the tracking section.

From the two previous coefficients, and the one of the `Collision` variable, we can 
speculate that **the distribution of events is more important than the overall number of 
segments**. An additional piece of evidence could be found by tallying facet 
crossing events: the reaction-specific coefficients hint at the cost of this kind of
outcome, `Absorb` being the variable with the highest coefficient. If this hypothesis 
is correct, we would also find a high coefficient for facet crossings.

Concerning reaction-specific coefficients, we can see that the `Absorb` and `Fission`
values are higher than `Scatter`. Considering that absorption happens both in reactions
and at the problem's bounds, it is coherent with the cost of the events. As for fission,
the reaction requires sampling for particles created during the reaction, a costly 
process both in execution time and memory.

### `PopulationControl` & `CycleSync`

![popsync](figures/FI_64/heatmap_popsync.png)


- `PopulationControl`/`Rr`: The coefficient is quite close to zero, meaning the time 
  spent doing population control is almost independent from the number of
  russian-rouletted particle. Note that this value is biased in this benchmark as 
  there is no russian-roulette due to overpopulation, they are only due to the low
  weight threshold.
- `PopulationControl`/`Split`: The coefficient is low positive. We can guess
  that creating particle is a costly task, so the more we create, the more time we 
  spend in the section. The low value could be explained by the number of "task" 
  ran for population control: sourcing, splitting, and two types of russian-roulette.
  The splitting can only account for so much of the total time, hence the 
  low-but-positive coefficient.
- `CycleSync`/`Rr`: The coefficient is low positive. Note that this value is biased 
  in this benchmark as there is no russian-roulette due to overpopulation, they are 
  only due to the low weight threshold.
- `CycleSync`/`Split`: The coefficient is negative, with a somewhat high value. This 
  can be explained by the logic controling the splitting process. Splitting happens 
  after sourcing, which happens after the synchronization phase. Splitting happens if
  the problem is underpopulated, the lower the number of particles, the higher the 
  number of split. A high split number means that, at the end of the last cycle, few
  particles were left. The lower number of particles result in a shorter sync phase. 


## Scaling

Data for the scaling study can be recorded using the `scaling.sh` script. By looking at 
the printed output of the program, one can already see that the figure of merit does not
vary significantly. This means that the time taken to compute a single segment for a particle
-- including look-up & update -- is independent from the total number of particle, 
which is expected. The script used to plot the data is accessible [here][1].

### `PopulationControl` & `CycleTracking`

![scaling_popcontrol](figures/FI_64/scaling_ppcontrol.png)

![scaling_tracking](figures/FI_64/scaling_tracking.png)

The time spent in the `PopulationControl` and `CycleTracking` sections seems to scale 
linearly with the target number of particles of the simulation. This is the expected 
results as there are no index lookup or nested iterators in our code; Not that Rust 
would allow it anyway.

### `CycleSync`

![scaling_sync](figures/FI_64/scaling_sync.png)

The value for 70,000 particles seems to be incorrect, a similar divergence can be observed on 
the `PopulationControl` diagram. This is probably a normal fluctuation and can be ignored.

At first, the sync phase seems to be independent from the target number of particles as it stays
constant. However, if we look at the script used to collect the data, we can see that no file 
name was specified for the energy spectrum to be printed. This means that the code is not keeping 
track of the spectrum, hence removing the dependance over the number of particles. If we were to 
run the script again, specifying a file name using `-e`, we would observe a similar figure to that 
of `PopulationControl` and `CycleTracking`.