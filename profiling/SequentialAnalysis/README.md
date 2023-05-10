# Sequential Analysis

This folder contains an overview of the sequential performances of Fastiron as well as the
referenced data used for the three specific analysis aside from benchmarking:

- A behavioral study: using tallied data to identify which events are costly, and how well
  the performances scale ith the total number of particles.
- A comparison with the original mini-app, [Quicksilver].
- A comparison in performances when executing the program using `f32` or `f64` types for 
  computations.

The `csv` files can be visualized using `column`: 

```bash
column -s=';' -t < a_file.csv # Might have to change the separator to comma for scaling data
```

[Quicksilver]: https://github.com/LLNL/Quicksilver