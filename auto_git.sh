#!/bin/bash

a=`date`
while true
do
git add *
git commit -m "$a" 
sleep 60*5
done
