#!/bin/bash

printf "challenge number: "
read challenge_number
cp -r ./template ./src/bin/"$challenge_number"
