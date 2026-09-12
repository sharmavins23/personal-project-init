@echo off

@REM Set up directory environment variables for the PPI project.
set "PPI_SRC=D:\Code\personal\personal-project-init"
set "EXE=%PPI_SRC%\target\release\ppi.exe"

@REM If the project is not yet built, build it.
if not exist "%EXE%" (
    echo Building PPI (first run)...
    cargo build --release --manifest-path "%PPI_SRC%\Cargo.toml"

    @REM If the build fails, exit the process.
    if errorlevel 1 exit /b %errorlevel%
)

@REM Run the executable.
"%EXE%" %*