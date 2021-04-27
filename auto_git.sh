#!/bin/bash

a=`date`
while true
do
git add *
git commit -m "$a" 

sleep 1
echo $a
done
