# perf tool statistics

Tests showed that Fastiron's scalability is worse than Quicksilver's. This file contains the raw perf 
reports used to understand what is going on. The simulation is stopped at 20 cycles.

## Regular execution



### Sequential 

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

### Rayon 

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

## XS-cache-less execution

### Sequential

```
Performance counter stats for './target/release/fastiron -i input_files/QS_originals/CTS2_Benchmark/CTS2_1.inp -r 1':

        130 341,70 msec task-clock:u              #    0,999 CPUs utilized          
                 0      context-switches:u        #    0,000 /sec                   
                 0      cpu-migrations:u          #    0,000 /sec                   
            14 881      page-faults:u             #  114,169 /sec                   
   207 021 372 474      cycles:u                  #    1,588 GHz                      (50,00%)
   438 590 283 453      instructions:u            #    2,12  insn per cycle           (62,50%)
    83 449 680 705      branches:u                #  640,238 M/sec                    (62,50%)
       704 410 446      branch-misses:u           #    0,84% of all branches          (62,51%)
   116 272 027 851      L1-dcache-loads:u         #  892,055 M/sec                    (62,51%)
    12 794 670 510      L1-dcache-load-misses:u   #   11,00% of all L1-dcache accesses  (62,50%)
     1 526 257 735      LLC-loads:u               #   11,710 M/sec                    (50,00%)
       238 372 909      LLC-load-misses:u         #   15,62% of all LL-cache accesses  (49,99%)

     130,515779794 seconds time elapsed

     130,283677000 seconds user
       0,059920000 seconds sys
```

### Rayon

```
 Performance counter stats for './target/release/fastiron -i input_files/QS_originals/CTS2_Benchmark/CTS2_1.inp -r 0':

        264 988,89 msec task-clock:u              #    7,778 CPUs utilized          
                 0      context-switches:u        #    0,000 /sec                   
                 0      cpu-migrations:u          #    0,000 /sec                   
            14 945      page-faults:u             #   56,399 /sec                   
   418 848 601 284      cycles:u                  #    1,581 GHz                      (50,01%)
   447 953 401 977      instructions:u            #    1,07  insn per cycle           (62,52%)
    85 201 657 236      branches:u                #  321,529 M/sec                    (62,54%)
       741 227 576      branch-misses:u           #    0,87% of all branches          (62,53%)
   118 789 836 575      L1-dcache-loads:u         #  448,282 M/sec                    (62,52%)
    25 911 379 733      L1-dcache-load-misses:u   #   21,81% of all L1-dcache accesses  (62,47%)
     2 924 566 443      LLC-loads:u               #   11,037 M/sec                    (49,97%)
       212 558 312      LLC-load-misses:u         #    7,27% of all LL-cache accesses  (49,98%)

      34,068105661 seconds time elapsed

     264,643334000 seconds user
       0,399992000 seconds sys
```

## SF-computation-less execution

### Sequential

```
Performance counter stats for './target/release/fastiron -i input_files/QS_originals/CTS2_Benchmark/CTS2_1.inp -r 1':

         95 702,16 msec task-clock:u              #    1,000 CPUs utilized          
                 0      context-switches:u        #    0,000 /sec                   
                 0      cpu-migrations:u          #    0,000 /sec                   
            14 880      page-faults:u             #  155,482 /sec                   
   152 028 455 179      cycles:u                  #    1,589 GHz                      (50,00%)
   298 071 099 554      instructions:u            #    1,96  insn per cycle           (62,50%)
    49 320 835 708      branches:u                #  515,358 M/sec                    (62,49%)
       669 592 685      branch-misses:u           #    1,36% of all branches          (62,50%)
    70 746 472 425      L1-dcache-loads:u         #  739,236 M/sec                    (62,50%)
     7 274 509 862      L1-dcache-load-misses:u   #   10,28% of all L1-dcache accesses  (62,50%)
     1 390 884 036      LLC-loads:u               #   14,533 M/sec                    (50,01%)
       246 225 893      LLC-load-misses:u         #   17,70% of all LL-cache accesses  (50,00%)

      95,712658684 seconds time elapsed

      95,666869000 seconds user
       0,035996000 seconds sys
```


### Rayon

```
 Performance counter stats for './target/release/fastiron -i input_files/QS_originals/CTS2_Benchmark/CTS2_1.inp -r 0':

        183 731,82 msec task-clock:u              #    7,661 CPUs utilized          
                 0      context-switches:u        #    0,000 /sec                   
                 0      cpu-migrations:u          #    0,000 /sec                   
            14 944      page-faults:u             #   81,336 /sec                   
   289 705 744 070      cycles:u                  #    1,577 GHz                      (50,01%)
   299 701 157 029      instructions:u            #    1,03  insn per cycle           (62,49%)
    49 585 892 788      branches:u                #  269,882 M/sec                    (62,47%)
       685 391 267      branch-misses:u           #    1,38% of all branches          (62,48%)
    71 140 662 187      L1-dcache-loads:u         #  387,198 M/sec                    (62,50%)
    10 897 441 540      L1-dcache-load-misses:u   #   15,32% of all L1-dcache accesses  (62,51%)
     1 947 668 985      LLC-loads:u               #   10,601 M/sec                    (50,04%)
       222 708 758      LLC-load-misses:u         #   11,43% of all LL-cache accesses  (50,02%)

      23,983502202 seconds time elapsed

     183,294629000 seconds user
       0,483521000 seconds sys
```
