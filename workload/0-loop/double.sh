#!/bin/bash

# require BC

# c for children
# sudo trace-cmd record -e sched -c -F ./double.sh TOTAL_MS [INBETWEEN_MS] 2> /dev/null

# Times in mills
TIME_TOTAL=$1 #in mills
TIME_INBETWEEN=$2 #in mills to wait before changing

if [ "$TIME_INBETWEEN" -gt "$TIME_TOTAL" ]; then
	echo "Total time must be larger than interval before signal"
	exit
fi


# Can put to 19 but not to -20 (increase but not decrease)
# I need to swap
if test -z "$TIME_INBETWEEN"; then
	nice -n 19 ./a.out $TIME_TOTAL & #low time and priority
	pid1=$!
	./b.out $TIME_TOTAL & # high time and priority (priority 0)
	pid2=$!
	echo "STARTED NORMAL MODE. With pids: $pid1 $pid2"
else
	./a.out $TIME_TOTAL & #low time and priority
	pid1=$!
	./b.out $TIME_TOTAL & #low time and priority
	pid2=$!
	echo "STARTED TWO TIME MODE. With pids: $pid1 $pid2"

	# wait for interval
	SEC_INBETWEEN=$(echo "scale=2; $TIME_INBETWEEN/1000" | bc)
	sleep $SEC_INBETWEEN

	# change both to high time and priority
	renice -20 -p $pid1
	renice -20 -p $pid2
fi

wait $pid1 || echo "error on pid1"
wait $pid2 || echo "error on pid2"
