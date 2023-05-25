# perf tool statistics

Tests showed that Fastiron's scalability is worse than Quicksilver's. This file contains the raw perf 
reports used to understand what is going on.

## Sequential 

```
Performance counter stats for './target/release/fastiron -i input_files/QS_originals/CTS2_Benchmark/CTS2_1.inp -r 1':

         99 461,62 msec task-clock:u              #    1,000 CPUs utilized          
                 0      context-switches:u        #    0,000 /sec                   
                 0      cpu-migrations:u          #    0,000 /sec                   
            14 880      page-faults:u             #  149,605 /sec                   
   157 817 923 412      cycles:u                  #    1,587 GHz                      (50,00%)
   298 102 398 985      instructions:u            #    1,89  insn per cycle           (62,50%)
    49 279 089 978      branches:u                #  495,458 M/sec                    (62,50%)
       668 881 465      branch-misses:u           #    1,36% of all branches          (62,50%)
    71 139 638 081      L1-dcache-loads:u         #  715,247 M/sec                    (62,50%)
     7 659 402 433      L1-dcache-load-misses:u   #   10,77% of all L1-dcache accesses  (62,50%)
     1 476 923 547      LLC-loads:u               #   14,849 M/sec                    (50,00%)
       272 570 633      LLC-load-misses:u         #   18,46% of all LL-cache accesses  (50,00%)

      99,474195638 seconds time elapsed

      99,433680000 seconds user
       0,027997000 seconds sys
```

## Rayon 

```
Performance counter stats for './target/release/fastiron -i input_files/QS_originals/CTS2_Benchmark/CTS2_1.inp -r 0':

        193 263,40 msec task-clock:u              #    7,481 CPUs utilized          
                 0      context-switches:u        #    0,000 /sec                   
                 0      cpu-migrations:u          #    0,000 /sec                   
            14 945      page-faults:u             #   77,330 /sec                   
   304 811 157 269      cycles:u                  #    1,577 GHz                      (50,01%)
   308 457 199 270      instructions:u            #    1,01  insn per cycle           (62,53%)
    50 938 884 424      branches:u                #  263,572 M/sec                    (62,51%)
       721 572 674      branch-misses:u           #    1,42% of all branches          (62,47%)
    73 749 651 136      L1-dcache-loads:u         #  381,602 M/sec                    (62,49%)
    11 409 539 658      L1-dcache-load-misses:u   #   15,47% of all L1-dcache accesses  (62,50%)
     2 104 922 669      LLC-loads:u               #   10,891 M/sec                    (50,02%)
       271 547 464      LLC-load-misses:u         #   12,90% of all LL-cache accesses  (50,02%)

      25,834007657 seconds time elapsed

     192,669044000 seconds user
       0,646754000 seconds sys

```