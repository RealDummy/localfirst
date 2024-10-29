#! /bin/bash

for i in $(seq 1000); do
    echo "Add( $(( $RANDOM % 10000 )) )" >> test/$1
done