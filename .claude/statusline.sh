#!/bin/bash

# Read the JSON input
input=$(cat)

# Extract values using jq
model_name=$(echo "$input" | jq -r '.model.display_name')
context_used=$(echo "$input" | jq -r '.context_window.used_percentage // empty')
context_remaining=$(echo "$input" | jq -r '.context_window.remaining_percentage // empty')

# Initialize status components
model_display=""
context_display=""

# Add model name if available
if [ -n "$model_name" ] && [ "$model_name" != "null" ]; then
    model_display="$model_name"
fi

# Add context usage if available
if [ -n "$context_used" ] && [ "$context_used" != "null" ]; then
    # Create a simple progress bar visualization
    # 20 character progress bar
    # Using awk instead of bc for better compatibility
    progress_chars=$(awk "BEGIN {print int($context_used * 0.2)}")

    # Ensure progress_chars is within bounds
    if [ "$progress_chars" -lt 0 ]; then
        progress_chars=0
    elif [ "$progress_chars" -gt 20 ]; then
        progress_chars=20
    fi

    # Create the progress bar
    filled=""
    empty=""

    for ((i=0; i<progress_chars; i++)); do
        filled="${filled}█"
    done

    for ((i=0; i<(20-progress_chars); i++)); do
        empty="${empty}░"
    done

    context_display="[${filled}${empty}] ${context_used}%"
fi

# Combine components
status_line=""
if [ -n "$model_display" ]; then
    status_line="$model_display"
fi

if [ -n "$context_display" ]; then
    if [ -n "$status_line" ]; then
        status_line="$status_line │ $context_display"
    else
        status_line="$context_display"
    fi
fi

# Output the status line
echo "$status_line"