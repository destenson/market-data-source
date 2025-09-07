@echo off
REM Windows batch script to generate CHANGELOG.md

echo Generating CHANGELOG.md from git history...
python "%~dp0generate-changelog.py" %*

if %ERRORLEVEL% NEQ 0 (
    echo.
    echo Error: Failed to generate changelog.
    echo Make sure Python is installed and you're in a git repository.
    exit /b 1
)

echo.
echo Changelog generation complete!