# This script used to reverse the change made by the bench_cfg.sh script
# The governor option might change according to the machine

# To be run as root!
echo "Enabling turbo boost"
echo 0 > /sys/devices/system/cpu/intel_pstate/no_turbo

echo "Using performance governor"
for i in /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor
do
  echo ondemand > $i
done