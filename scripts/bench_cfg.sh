# This is one of two scripts provided to compute some numbers about 
# the behavior of Fastiron

# This script is used to set a few options for the execution 
# behavior. This is done to achieve overall more consistent
# numbers from one sample to another.

# From https://easyperf.net/blog/2019/08/02/Perf-measurement-environment-on-Linux
# To be run as root!
echo "Disabling turbo boost"
echo 1 > /sys/devices/system/cpu/intel_pstate/no_turbo

echo "Using performance governor"
for i in /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor
do
  echo performance > $i
done