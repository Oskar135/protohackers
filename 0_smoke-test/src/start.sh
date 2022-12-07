#!/bin/bash

COMMAND=$1

function testnc {
    echo "Starting nc server for testing purposes"
    nc -l -p 3500 127.0.0.1 & 
    # Writer would be nc 127.0.0.1 3500
}

if [ "$COMMAND" = "test" ]; then testnc
else
    echo "Running Programs"
fi

sleep 1
echo "Compiling and executing"
cargo build 
cargo run &
python3 pythoncode/writer.py