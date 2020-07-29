#!/usr/bin/env fish

date
echo "Sending 10 Requests"
./target/debug/clignitas -a stress -k "./keys/key_1.file" -n 10 -m 4
echo "Waiting 30 seconds"
sleep 30
echo "Sending 100 Requests"
./target/debug/clignitas -a stress -k "./keys/key_1.file" -n 100 -m 4
echo "Waiting 30 seconds"
sleep 30
echo "Sending 1 000 Requests"
./target/debug/clignitas -a stress -k "./keys/key_1.file" -n 1000 -m 4
echo "Waiting 30 seconds"
sleep 30
echo "Sending 10 000 Requests"
./target/debug/clignitas -a stress -k "./keys/key_1.file" -n 10000 -m 4
