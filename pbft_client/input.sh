#!/bin/bash
int=1
while(( $int<=100 ))
do
    echo "put key"$int"=value"$int
    let "int++"
done