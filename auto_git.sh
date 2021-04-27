#!/bin/bash

while true
do
git add *
git commit -m "`date`" 
git push
sleep  $((5*60))
done
