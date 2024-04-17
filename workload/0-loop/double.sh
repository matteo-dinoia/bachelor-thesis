#!/bin/bash


# c for children
# sudo trace-cmd record -e sched -c -F ./double.sh 1000

pid1=0
pid2=0

TIME=$1

nice -n 19 ./a.out $TIME & #low time and priority
pid1=$!
nice -n -20 ./b.out $TIME & # high time and priority
pid2=$!

echo "STARTED. With pids: $pid1 $pid2"

wait $pid1 || echo "error on pid1"
wait $pid2 || echo "error on pid2"