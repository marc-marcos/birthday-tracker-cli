#!/bin/bash

cargo build --release

sudo cp target/release/birthday-tracker-cli /usr/bin/

birthday-tracker-cli -c

echo -e "\e[32mProgram installed successfully!\e[0m"
