"""Test-related Angreal tasks for Mimir project

Test Organization:
- Unit tests: Located in src/ files with #[cfg(test)] modules, run with --lib
- Integration tests: Located in tests/ directories, run with --test
- Frontend tests: Located in frontend/, run with npm test

Commands:
- angreal test: Run all tests (backend + frontend)
- angreal test --watch: Run tests in watch mode
- angreal test --coverage: Generate coverage reports
"""
import angreal
import subprocess
from pathlib import Path
import sys

# Get the project root directory
PROJECT_ROOT = Path(angreal.get_root()).parent
FRONTEND_DIR = PROJECT_ROOT / "crates" / "mimir-dm" / "frontend"

# Create a test command group
test = angreal.command_group(name="test", about="Testing commands for the Mimir workspace")

@test
@angreal.argument(name="watch", short="w", takes_value=False, help="Run tests in watch mode")
@angreal.argument(name="backend_only", short="b", takes_value=False, help="Run backend tests only")
@angreal.argument(name="frontend_only", short="f", takes_value=False, help="Run frontend tests only")
def test_main(watch: bool = False, backend_only: bool = False, frontend_only: bool = False):
    """Run tests (all, backend only, or frontend only)"""
    failures = []

    # Run backend tests unless frontend-only
    if not frontend_only:
        print("\nRunning Rust tests (unit + integration)...")
        result = subprocess.run(
            ["cargo", "test", "--workspace"],
            cwd=PROJECT_ROOT,
            capture_output=False
        )
        if result.returncode != 0:
            failures.append("Backend tests")

    # Run frontend tests unless backend-only
    if not backend_only:
        print("\nRunning frontend tests...")
        if not FRONTEND_DIR.exists():
            print(f"Frontend directory not found: {FRONTEND_DIR}")
            sys.exit(1)

        if not (FRONTEND_DIR / "node_modules").exists():
            print("Installing frontend dependencies...")
            subprocess.run(["npm", "install"], cwd=FRONTEND_DIR)

        cmd = ["npm", "test"]
        if not watch:
            cmd.extend(["--", "--run"])

        result = subprocess.run(cmd, cwd=FRONTEND_DIR, capture_output=False)
        if result.returncode != 0:
            failures.append("Frontend tests")

    # Summary
    if failures:
        print(f"\nTest failures in: {', '.join(failures)}")
        sys.exit(1)
    else:
        print("\nAll tests passed!")

@test
@angreal.command(name="coverage", about="Run tests with coverage reporting")
@angreal.argument(name="backend_only", short="b", takes_value=False, help="Run backend coverage only")
@angreal.argument(name="frontend_only", short="f", takes_value=False, help="Run frontend coverage only")
@angreal.argument(name="open", short="o", takes_value=False, help="Open coverage reports in browser")
def test_coverage(backend_only: bool = False, frontend_only: bool = False, open: bool = False):
    """Run tests with code coverage for both backend and frontend"""
    print("Running tests with coverage...")
    
    failures = []
    
    # Run backend coverage unless frontend-only
    if not frontend_only:
        print("\n[Backend Coverage (Rust)]")
        
        # Check if cargo-tarpaulin is installed
        check_result = subprocess.run(
            ["cargo", "tarpaulin", "--version"],
            capture_output=True,
            cwd=PROJECT_ROOT
        )
        
        if check_result.returncode != 0:
            print("cargo-tarpaulin not found. Installing...")
            subprocess.run(["cargo", "install", "cargo-tarpaulin"], cwd=PROJECT_ROOT)
        
        # Run tarpaulin with HTML output
        cmd = [
            "cargo", "tarpaulin",
            "--workspace",
            "--out", "Html",
            "--output-dir", "target/coverage",
            "--exclude-files", "*/tests/*", "*/examples/*", "*/build.rs",
            "--timeout", "120"
        ]
        
        result = subprocess.run(cmd, cwd=PROJECT_ROOT, capture_output=False)
        if result.returncode != 0:
            failures.append("Backend coverage")
        else:
            print("Backend coverage report: target/coverage/tarpaulin-report.html")
            if open:
                subprocess.run(["open", "target/coverage/tarpaulin-report.html"], cwd=PROJECT_ROOT)
    
    # Run frontend coverage unless backend-only
    if not backend_only:
        print("\n[Frontend Coverage (Vue/TypeScript)]")
        
        # Check if coverage package is installed
        check_result = subprocess.run(
            ["npm", "list", "@vitest/coverage-v8"],
            capture_output=True,
            cwd=FRONTEND_DIR
        )
        
        if check_result.returncode != 0:
            print("Installing @vitest/coverage-v8...")
            subprocess.run(["npm", "install", "--save-dev", "@vitest/coverage-v8"], cwd=FRONTEND_DIR)
        
        # Run frontend coverage
        result = subprocess.run(
            ["npm", "run", "test:coverage"],
            cwd=FRONTEND_DIR,
            capture_output=False
        )
        if result.returncode != 0:
            failures.append("Frontend coverage")
        else:
            print("Frontend coverage report: crates/mimir-dm/frontend/coverage/index.html")
            if open:
                subprocess.run(["open", "crates/mimir-dm/frontend/coverage/index.html"], cwd=PROJECT_ROOT)
    
    # Summary
    if not failures:
        print("\nAll coverage reports generated successfully!")
        if not open:
            print("Use --open flag to view reports in browser")
    else:
        print(f"\nFailed: {', '.join(failures)}")
        sys.exit(1)