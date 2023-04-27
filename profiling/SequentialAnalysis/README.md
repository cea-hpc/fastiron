# Sequential Analysis

---

## Methodology

The study of the influence of the number of particles was done using the `homogeneous7`
problem while the rest was done using the `CTS2_1` benchmark. The data recorded in the 
`csv` file can be viewed using `column`: 

```bash
column -s=';' -t < a_file.csv
```

The data is used in two ways: 

- Compute the correlation coefficient between specific events and recorded times for 
given sections.
- Curve-fitting for the link between total number of particles and time spent in each 
section.