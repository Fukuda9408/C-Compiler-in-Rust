#!/bin/bash

git pull origin main
cargo build
bash ./test.sh
