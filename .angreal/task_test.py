"""Test-related Angreal tasks for Mimir project"""
import angreal
import subprocess
from pathlib import Path
import sys
import json

# Get the project root directory
PROJECT_ROOT = Path(angreal.get_root()).parent
FRONTEND_DIR = PROJECT_ROOT / "crates" / "mimir-dm" / "frontend"

# Create a test command group
test = angreal.command_group(name="test", about="Testing commands for the Mimir workspace")

@test
@angreal.command(name="all", about="Run all tests (Rust + Frontend)")
def test_all():
    """Run all tests across the entire project"""
    print("=" * 60)
    print("Running all tests (Rust + Frontend)...")
    print("=" * 60)
    
    failures = []
    
    # Run Rust tests
    print("\n📦 Running Rust tests...")
    result = subprocess.run(
        ["cargo", "test", "--workspace", "--lib", "--bins", "--tests"],
        cwd=PROJECT_ROOT,
        capture_output=False
    )
    if result.returncode != 0:
        failures.append("Rust tests")
    
    # Run Frontend tests
    print("\n🎨 Running Frontend tests...")
    result = subprocess.run(
        ["npm", "test", "--", "--run"],
        cwd=FRONTEND_DIR,
        capture_output=False
    )
    if result.returncode != 0:
        failures.append("Frontend tests")
    
    # Summary
    print("\n" + "=" * 60)
    if failures:
        print(f"❌ Test failures in: {', '.join(failures)}")
        return 1
    else:
        print("✅ All tests passed!")
        return 0

@test
@angreal.command(name="unit", about="Run unit tests only (fast)")
def test_unit():
    """Run only unit tests - fast feedback loop"""
    print("Running unit tests...")
    
    # Run Rust unit tests (exclude integration tests)
    result = subprocess.run(
        ["cargo", "test", "--workspace", "--lib"],
        cwd=PROJECT_ROOT,
        capture_output=False
    )
    return result.returncode

@test
@angreal.command(name="frontend", about="Run Vue/Vitest tests")
@angreal.argument(name="watch", short="w", takes_value=False, help="Run in watch mode")
def test_frontend(watch: bool = False):
    """Run frontend tests with Vitest"""
    print("Running frontend tests...")
    
    if not FRONTEND_DIR.exists():
        print(f"Frontend directory not found: {FRONTEND_DIR}")
        return 1
    
    # Check if node_modules exists
    if not (FRONTEND_DIR / "node_modules").exists():
        print("Installing frontend dependencies...")
        subprocess.run(["npm", "install"], cwd=FRONTEND_DIR)
    
    # Run tests
    cmd = ["npm", "test"]
    if not watch:
        cmd.extend(["--", "--run"])
    
    result = subprocess.run(cmd, cwd=FRONTEND_DIR, capture_output=False)
    return result.returncode

@test
@angreal.command(name="backend", about="Run Rust tests")
@angreal.argument(name="crate", short="c", help="Test specific crate")
def test_backend(crate: str = None):
    """Run Rust backend tests"""
    print("Running backend tests...")
    
    cmd = ["cargo", "test"]
    if crate:
        cmd.extend(["-p", crate])
        print(f"Testing crate: {crate}")
    else:
        cmd.append("--workspace")
        print("Testing all crates...")
    
    result = subprocess.run(cmd, cwd=PROJECT_ROOT, capture_output=False)
    return result.returncode

@test
@angreal.command(name="integration", about="Run integration tests")
def test_integration():
    """Run integration tests (includes external dependencies)"""
    print("Running integration tests...")
    
    # Run LLM integration tests if script exists
    llm_test_script = PROJECT_ROOT / "crates" / "mimir-dm-llm" / "run_integration_tests.sh"
    if llm_test_script.exists():
        print("Running LLM integration tests...")
        subprocess.run(["bash", str(llm_test_script)], cwd=llm_test_script.parent)
    
    # Run Rust integration tests (tests marked with #[ignore])
    print("Running Rust integration tests...")
    result = subprocess.run(
        ["cargo", "test", "--workspace", "--", "--ignored"],
        cwd=PROJECT_ROOT,
        capture_output=False
    )
    return result.returncode

@test
@angreal.command(name="e2e", about="Run end-to-end tests")
def test_e2e():
    """Run end-to-end tests (full application flow)"""
    print("Running end-to-end tests...")
    
    # For now, we'll run the Tauri app tests if they exist
    # In the future, this could include Playwright or similar e2e tests
    
    # Check if Tauri tests exist
    tauri_test_dir = PROJECT_ROOT / "crates" / "mimir-dm" / "tests"
    if tauri_test_dir.exists():
        print("Running Tauri application tests...")
        result = subprocess.run(
            ["cargo", "test", "-p", "mimir-dm", "--test", "*"],
            cwd=PROJECT_ROOT,
            capture_output=False
        )
        return result.returncode
    else:
        print("No e2e tests found yet")
        return 0

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
        return 1
    
    return 0