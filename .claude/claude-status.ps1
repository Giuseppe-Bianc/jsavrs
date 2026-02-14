# Read JSON input from stdin
$input = $input | Out-String

if ($input.Trim().Length -gt 0) {
    try {
        # Parse JSON
        $json = $input | ConvertFrom-Json
        
        # Extract values
        $model_name = $json.model.display_name
        $context_used = $json.context_window.used_percentage
        $context_remaining = $json.context_window.remaining_percentage
        
        # Initialize status components
        $model_display = ""
        $context_display = ""
        
        # Add model name if available
        if ($model_name -and $model_name -ne "null") {
            $model_display = $model_name
        }
        
        # Add context usage if available
        if ($context_used -and $context_used -ne "null") {
            # Create a 20 character progress bar
            $progress_chars = [Math]::Floor($context_used * 0.2)
            
            # Ensure progress_chars is within bounds
            if ($progress_chars -lt 0) { $progress_chars = 0 }
            if ($progress_chars -gt 20) { $progress_chars = 20 }
            
            # Create the progress bar
            $filled = "█" * $progress_chars
            $empty = "░" * (20 - $progress_chars)
            
            $context_display = "[$filled$empty] $context_used%"
        }
        
        # Combine components
        $status_line = ""
        if ($model_display) {
            $status_line = $model_display
        }
        
        if ($context_display) {
            if ($status_line) {
                $status_line = "$status_line │ $context_display"
            } else {
                $status_line = $context_display
            }
        }
        
        # Output the status line
        Write-Output $status_line
        
    } catch {
        Write-Output "Error: $($_.Exception.Message)"
    }
} else {
    Write-Output ""
}