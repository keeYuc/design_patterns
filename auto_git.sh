#!/bin/bash

a=`date`
while true
do
git add *
git commit -m "$a" 
sleep  expr 60*5
done
