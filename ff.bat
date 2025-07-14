@echo off
REM File Finder & Grep Tool - Windows Batch Script
REM This script allows you to run the tool from anywhere

cd /d "%~dp0"
.\target\release\file_finder.exe %*
