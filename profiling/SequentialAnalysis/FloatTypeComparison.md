# Comparison between execution on `f32` & `f64`



## Behaviors

 `f32` Fastiron                                     | `f64` Fastiron
----------------------------------------------------|----------------------------------------------------
![f32_tracking](figures/FI_32/heatmap_tracking.png) | ![f64_tracking](figures/FI_64/heatmap_tracking.png)
![f32_popsync](figures/FI_32/heatmap_popsync.png)   | ![f64_popsync](figures/FI_64/heatmap_popsync.png)
 `8.085e5 [segments / cycle tracking time]`         | `7.638e5 [segments / cycle tracking time]`


## Scaling

 `f32` Fastiron                                       | `f64` Fastiron
------------------------------------------------------|----------------------------------------------------
![f32_tracking](figures/FI_32/scaling_tracking.png)   | ![f64_tracking](figures/FI_64/scaling_tracking.png)
![f32_ppcontrol](figures/FI_32/scaling_ppcontrol.png) | ![f64_ppcontrol](figures/FI_64/scaling_ppcontrol.png)
![f32_sync](figures/FI_32/scaling_sync.png)           | ![f64_sync](figures/FI_64/scaling_sync.png)