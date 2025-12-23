@echo off
SETLOCAL EnableDelayedExpansion EnableExtensions

REM ============================================================================
REM Cargo Profiling Script - Optimized Version (Always Verbose)
REM Purpose: Run heap profiling and flamegraph generation for Rust projects
REM ============================================================================

REM --- Configuration (Override via environment variables) ---
IF NOT DEFINED BIN_NAME SET "BIN_NAME=jsavrs"
IF NOT DEFINED INPUT_FILE SET "INPUT_FILE=vn_files\large_toy_program.vn"
IF NOT DEFINED FREQUENCY SET "FREQUENCY=9999"

REM --- Display Configuration ---
echo.
echo ============================================
echo   Profiling Configuration
echo ============================================
echo Binary Name:    %BIN_NAME%
echo Input File:     %INPUT_FILE%
echo Sample Freq:    %FREQUENCY% Hz
echo Started:        %DATE% %TIME%
echo ============================================
echo.

REM --- Pre-flight Checks ---
echo [%TIME%] Validating prerequisites...

REM Check if input file exists
IF NOT EXIST "%INPUT_FILE%" (
    echo [ERROR] Input file not found: %INPUT_FILE%
    echo Please verify the path and try again.
    EXIT /B 2
)
echo [OK] Input file found: %INPUT_FILE%

REM Check if cargo is available
cargo --version >nul 2>&1
IF !ERRORLEVEL! NEQ 0 (
    echo [ERROR] Cargo not found. Please ensure Rust toolchain is installed.
    echo Visit: https://rustup.rs/
    EXIT /B 3
)
echo [OK] Cargo toolchain available

REM Check if cargo-flamegraph is installed (optional check)
cargo flamegraph --help >nul 2>&1
IF !ERRORLEVEL! NEQ 0 (
    echo [WARNING] cargo-flamegraph may not be installed.
    echo Install with: cargo install flamegraph
    echo Continuing anyway...
) ELSE (
    echo [OK] cargo-flamegraph available
)

echo.

REM --- Step 1: Heap Profiling with dhat ---
echo [%TIME%] [STEP 1/2] Running heap profiling with dhat-heaps...
echo Command: cargo run --release --features dhat-heaps -- -i "%INPUT_FILE%"
echo.

cargo run --release --features dhat-heaps -- -i "%INPUT_FILE%"
SET "HEAP_EXIT_CODE=!ERRORLEVEL!"

echo.
IF !HEAP_EXIT_CODE! NEQ 0 (
    echo [ERROR] Heap profiling failed with exit code !HEAP_EXIT_CODE!
    echo Check the error messages above for details.
    EXIT /B !HEAP_EXIT_CODE!
) ELSE (
    echo [SUCCESS] Heap profiling completed successfully.
)

REM --- Step 2: Flamegraph Generation ---
echo.
echo [%TIME%] [STEP 2/2] Generating flamegraph...
echo NOTE: This requires elevated privileges via sudo.
echo Command: sudo cargo flamegraph --release --freq %FREQUENCY% --root --bin "%BIN_NAME%" -- -i "%INPUT_FILE%"
echo.

sudo cargo flamegraph --release --freq %FREQUENCY% --root --bin "%BIN_NAME%" -- -i "%INPUT_FILE%"
SET "FLAMEGRAPH_EXIT_CODE=!ERRORLEVEL!"

echo.
IF !FLAMEGRAPH_EXIT_CODE! NEQ 0 (
    echo [ERROR] Flamegraph generation failed with exit code !FLAMEGRAPH_EXIT_CODE!
    echo Common causes:
    echo   - Insufficient privileges ^(even with sudo^)
    echo   - cargo-flamegraph not properly installed
    echo   - perf not available on your system
    EXIT /B !FLAMEGRAPH_EXIT_CODE!
) ELSE (
    echo [SUCCESS] Flamegraph generated successfully.
)

REM --- Completion Summary ---
echo.
echo ============================================
echo   Profiling Complete
echo ============================================
echo Completed:      %DATE% %TIME%
echo Exit Code:      0
echo.
echo Output files should be in the current directory:
echo   - dhat-heap.json ^(heap profile^)
echo   - flamegraph.svg ^(performance visualization^)
echo ============================================

EXIT /B 0