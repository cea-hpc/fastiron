## WHAT

This crate allows you to create a bindable thread pool. This is simply a wrapper around the Rayon's thread pool. It supports all the "major" functions with the same signature. At the time of creation of this pool, a binding policy is specified. All the threads in this pool will be bound to the cores using this policy. Currently we bind to one single numa node but other policies will eventually be added at a later point.

Note that if the machine is hyperthreaded, the system will try not to map two threads on the same physical core.

## WHY
This crate uses an existing HWLOC-RS crate by daschl. However, it is much more programmer friendly since the same API is exposed and that makes it super easy to use.
