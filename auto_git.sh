#!/bin/bash

a=`date`
while true
do
git add *
git commit -m $a
fi 
