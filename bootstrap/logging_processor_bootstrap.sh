#!/bin/sh

tail -n+1 -F $LOG_PATH | grep -G '\w' --line-buffered | grep -Gv '^==>' | ./app