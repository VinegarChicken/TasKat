@echo off
echo Building PromptFile with Security Improvements...
echo.

echo Checking Rust installation...
rustc --version >nul 2>&1
if errorlevel 1 (
    echo ERROR: Rust is not installed or not in PATH
    echo Please install Rust from https://rustup.rs/
    pause
    exit /b 1
)

echo Rust found. Building project...
echo.

cargo build --release
if errorlevel 1 (
    echo.
    echo BUILD FAILED!
    echo Check the error messages above.
    pause
    exit /b 1
)

echo.
echo ✅ Build successful!
echo.
echo Executable location: target\release\promptfile.exe
echo.
echo Next steps:
echo 1. Set your Gemini API key: setx GEMINI_API_KEY "your-api-key-here"
echo 2. Install context menu: .\target\release\promptfile.exe install
echo 3. Test with: .\target\release\promptfile.exe prompt "C:\some-folder"
echo.
pause