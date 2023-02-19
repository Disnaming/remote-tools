#!/bin/bash

$DOMAIN = "disna-m.com"
$TARGET_IP = "127.0.0.1"
$INITIAL_IP = "1.1.1.1"

sudo python3 dns-rebinder.py $DOMAIN $TARGET_IP $INITIAL_IP