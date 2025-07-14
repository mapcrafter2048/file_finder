@echo off
echo.
echo ============================================
echo   File Finder ^& Grep Tool - Demo Script
echo ============================================
echo.

echo 🔍 Example 1: Finding all Rust files
echo Command: ff find ".*\.rs$" --regex
echo.
call ff.bat find ".*\.rs$" --regex
echo.

echo.
echo 🔎 Example 2: Searching for "pub fn" in Rust files
echo Command: ff grep "pub fn" --ext rs
echo.
call ff.bat grep "pub fn" --ext rs
echo.

echo.
echo 🔍 Example 3: Finding configuration files (case-insensitive)
echo Command: ff find "config" --ignore-case
echo.
call ff.bat find "config" --ignore-case
echo.

echo.
echo ============================================
echo   Demo completed! Try your own searches:
echo   - ff find "filename"
echo   - ff grep "pattern"
echo   - ff --help for more options
echo ============================================
echo.
pause
