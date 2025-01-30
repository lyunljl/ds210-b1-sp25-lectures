#!/bin/sh

CORRECT=n
while [ "$CORRECT" = "n" ]
do

# loop discontinues when you enter y i.e., when your name is correct

echo -n "Enter your name: "
read NAME

echo -n "Is ${NAME} correct (y/n)? "
read CORRECT

done
