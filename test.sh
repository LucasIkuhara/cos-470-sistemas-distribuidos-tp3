#!/bin/bash

# Define cleanup procedure
cleanup() {
    echo "Caught Interrupt signal. Cleaning up..."
    for pid in ${pids[*]}; do
        kill $pid
    done
    exit
}

trap "cleanup" SIGINT

# Run client 32 times
for (( i=1; i<=32; i++ ))
do
  echo "Running client $i..."
  ./target/debug/client &
  pids[$i]=$!
done

# Wait for all background processes to finish
for pid in ${pids[*]}; do
    wait $pid
done
