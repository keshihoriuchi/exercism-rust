#!/bin/bash

N=2400
URL='http://127.0.0.1:55001/counter'

for C in 1 16 160
do
    ab -c $C -n $N $URL > result_${C}.txt
done
