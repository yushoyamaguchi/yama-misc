#!/bin/bash

TRACE_PIPE="/sys/kernel/debug/tracing/trace_pipe"

echo "Monitoring BPF logs... Press Ctrl+C to stop"

if [[ ! -e "$TRACE_PIPE" ]]; then
    echo "Error: trace_pipe not found. Make sure the BPF program is loaded correctly."
    exit 1
fi

if [[ ! -r "$TRACE_PIPE" ]]; then
    echo "Error: Root privileges are required to access trace_pipe."
    exit 1
fi

# Exit on Ctrl+C
trap 'echo; echo "Stopping log monitoring."; exit 0' INT

# Log reading loop
while read -r line; do
    [[ -n "$line" ]] && echo "$line"
done < "$TRACE_PIPE"
