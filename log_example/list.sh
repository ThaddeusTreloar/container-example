#!/bin/sh

cat log.json | rg -o '"id":"[0-9A-F]+"' | rg -o '[0-9A-F]+' | sort | uniq