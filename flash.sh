#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}HiFive Pro P550 Flashing Tool${NC}"
echo "--------------------------------"

# Check if running in Docker
if [ ! -f /.dockerenv ]; then
    echo -e "${RED}Error: This script must be run inside the Docker container${NC}"
    exit 1
fi

# Function to check if command exists
check_command() {
    if ! command -v $1 &> /dev/null; then
        echo -e "${RED}Error: $1 is not installed${NC}"
        exit 1
    fi
}

# Check required commands
check_command openocd
check_command riscv64-unknown-elf-gdb
check_command cargo

# Build the project
echo -e "\n${YELLOW}Building project...${NC}"
cargo build --target riscv64gc-unknown-none-elf --release
if [ $? -ne 0 ]; then
    echo -e "${RED}Build failed!${NC}"
    exit 1
fi
echo -e "${GREEN}Build successful${NC}"

# Start OpenOCD in the background
echo -e "\n${YELLOW}Starting OpenOCD...${NC}"
sudo openocd -f interface/ftdi/olimex-arm-usb-tiny-h.cfg -f target/sifive-hifive1.cfg &
OPENOCD_PID=$!

# Wait for OpenOCD to start
sleep 2

# Check if OpenOCD is running
if ! ps -p $OPENOCD_PID > /dev/null; then
    echo -e "${RED}Failed to start OpenOCD${NC}"
    exit 1
fi

# Create GDB command file
echo "target remote localhost:3333" > gdb_commands.txt
echo "load" >> gdb_commands.txt
echo "continue" >> gdb_commands.txt

# Flash using GDB
echo -e "\n${YELLOW}Flashing firmware...${NC}"
riscv64-unknown-elf-gdb -x gdb_commands.txt target/riscv64gc-unknown-none-elf/release/os

# Cleanup
kill $OPENOCD_PID
rm gdb_commands.txt

echo -e "\n${GREEN}Flashing complete!${NC}"