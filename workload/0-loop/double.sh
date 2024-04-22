#!/bin/bash

# require BC

# c for children
# sudo trace-cmd record -e sched -c -F ./double.sh TIME [INTERVAL] 2> /dev/null

TIME=$1 #in mills
TIME_INTERVAL=$2 #in mills to wait before changing


if test -z "$TIME_INTERVAL"; then
	nice -n 19 ./a.out $TIME & #low time and priority
	pid1=$!
	nice -n -20 ./b.out $TIME & # high time and priority
	pid2=$!
	echo "STARTED NORMAL MODE. With pids: $pid1 $pid2"
else
	nice -n 19 ./a.out $TIME & #low time and priority
	pid1=$!
	nice -n 19 ./b.out $TIME & #low time and priority
	pid2=$!
	echo "STARTED TWO TIME MODE. With pids: $pid1 $pid2"

	# wait for interval
	TIME_SEC=$(echo "scale=2; $TIME_INTERVAL/1000" | bc)
	sleep $TIME_SEC

	# change both to high time and priority
	renice -20 -p $pid1
	renice -20 -p $pid2
fi

wait $pid1 || echo "error on pid1"
wait $pid2 || echo "error on pid2"
