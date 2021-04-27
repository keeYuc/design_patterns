#!/bin/bash

while true
do
git add *
git commit -m "$`date`" 
sleep  $((5*1))
done
