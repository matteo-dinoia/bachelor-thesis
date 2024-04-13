#!/bin/bash

pid1=0
pid2=0

TIME=$1

./a.out $TIME &
pid1=$!
./b.out $TIME &
pid2=$!

echo "STARTED. With pids: $pid1 $pid2"

wait $pid1 || echo "error on pid1"
wait $pid2 || echo "error on pid2"