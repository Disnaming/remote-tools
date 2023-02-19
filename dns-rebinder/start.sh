#!/bin/bash

$DOMAIN = "disna-m.com"
$INITIAL_IP = "SECRET"

sudo python3 alternate-dns.py $DOMAIN --ip $INITIAL_IP --mode static_zero