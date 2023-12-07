#!/bin/sh

oha -z 15s -c 1000 --disable-keepalive http://127.0.0.1:8080/combo/load_test -H "Content-Type: application/json" -d '{  "origin" : "AU",  "colour" : "blue",  "property": "Some Property",  "value": "high"}' -m POST