#!/bin/bash

if command -v arm-none-eabi-gdb &> /dev/null; then
GDB="arm-none-eabi-gdb"
elif command -v gdb-multiarch &> /dev/null; then
GDB="gdb-multiarch"
else
echo "ERROR: GDB binary missing! You either need arm-none-eabi-gdb or gdb-multiarch in your PATH"
exit 1
fi

if [[ $(uname) == 'Linux' ]]; then
BMPGDBDEV="/dev/ttyBmpGdb"
elif [[ $(uname) == 'Darwin' ]]; then
BMPGDBDEV=$(ls /dev/cu.usbmodem[0-9A-F][0-9A-F][0-9A-F][0-9A-F][0-9A-F][0-9A-F][0-9A-F][0-9A-F]1)
else
echo "ERROR: BMP serial device autodetection is not supported for this operating system"
exit 1
fi

${GDB} -nx --batch -ex "tar ext ${BMPGDBDEV}" -x ./scripts/gdb_load.scr $*
