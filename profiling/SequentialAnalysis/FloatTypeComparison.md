# Comparison between execution on `f32` & `f64`

From Fastiron `1.1` and onward, support for `f32`/`f64` is implemented. This file is updated with the latest 
sequential data on floating-type comparison. The version currently used is `1.2-RuSeq`.

## Behaviors

 `f32` Fastiron                                     | `f64` Fastiron
----------------------------------------------------|----------------------------------------------------
![f32_tracking](figures/FI_32/heatmap_tracking.png) | ![f64_tracking](figures/FI_64/heatmap_tracking.png)
![f32_popsync](figures/FI_32/heatmap_popsync.png)   | ![f64_popsync](figures/FI_64/heatmap_popsync.png)
 `FoM: 8.085e5 [segments / cycle tracking time]`    | `FoM: 7.638e5 [segments / cycle tracking time]`

There is an approximate 5% difference of Figure of Merit between the two versions. From this we can guess
that the memory one of multiple limiting factors overall. Otherwise, the increase in _FoM_ would have been 
much higher.

Interestingly, event correlation to `CycleTracking` seems exacerbated when using `f64` for computations.
While the difference may not be significant for `Census`, it is for all others. The opposite phenomenon 
is observed for correlations to `CycleSync`. The time spent in the section seems much more dependant of 
the events, especially splitting, when using `f32`.

This can be explained by writing each time variable with two sub-times: **T_{total} = T_{affected} + T_{static}**.
All events cost a certain time, two parts make it up: A part affected by the floating type used (e.g. arithmetic 
operations) and the rest. This also applies to total section times. \
Depending on the relative importance of T_{affected} and T_{static} at both timer and event scale, the correlation 
coefficient will evolve differently when swapping **from `f64` to `f32`**:

| Timer Behavior        | Event Behavior        | Effect on time values | Effect on correlation coefficient |
|-----------------------|-----------------------|-----------------------|-----------------------------------|
| T_{affected} dominant | T_{affected} dominant | Both go down significantly            | Depends of how much the event time makes up for the total timer value |
| T_{affected} dominant | T_{static} dominant   | Timer goes down, event stays constant | The event becomes closer to being a bottleneck, coefficient is more or less exacerbated |
| T_{static} dominant   | T_{static} dominant   | Both do not change significantly      | Depends of how much the event time makes up for the total timer value |
| T_{static} dominant   | T_{affected} dominant | Event goes down, timer stays constant | The event is less likely to be the bottleneck, coefficient is more or less attenuated |



## Scaling

 `f32` Fastiron                                       | `f64` Fastiron
------------------------------------------------------|----------------------------------------------------
![f32_tracking](figures/FI_32/scaling_tracking.png)   | ![f64_tracking](figures/FI_64/scaling_tracking.png)
![f32_ppcontrol](figures/FI_32/scaling_ppcontrol.png) | ![f64_ppcontrol](figures/FI_64/scaling_ppcontrol.png)
![f32_sync](figures/FI_32/scaling_sync.png)           | ![f64_sync](figures/FI_64/scaling_sync.png)