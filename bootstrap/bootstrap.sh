#!/bin/sh

touch $LOG_PATH/$BIN_NAME.log

$1 >> $LOG_PATH/$BIN_NAME.log 2>&1