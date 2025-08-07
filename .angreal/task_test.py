"""Test-related Angreal tasks for Mimir project"""
import angreal
import subprocess
from pathlib import Path

# Get the project root directory
PROJECT_ROOT = Path(angreal.get_root()).parent

# Create a test command group
test = angreal.command_group(name="test", about="Testing commands for the Rust workspace")

@test
@angreal.command(name="all", about="Run all tests across the workspace")
def test_all():
    """Run all tests in the workspace with cargo test"""
    print("Running all tests across the workspace...")
    result = subprocess.run(
        ["cargo", "test", "--workspace", "--lib", "--bins", "--tests"],
        cwd=PROJECT_ROOT,
        capture_output=False
    )
    return result.returncode

@test
@angreal.command(name="crate", about="Run tests for a specific crate")
@angreal.argument(name="crate", help="Name of the crate to test (e.g., mimir-dm-core)")
def test_crate(crate: str):
    """Run tests for a specific crate"""
    print(f"Running tests for crate: {crate}")
    result = subprocess.run(
        ["cargo", "test", "-p", crate],
        cwd=PROJECT_ROOT,
        capture_output=False
    )
    return result.returncode

@test
@angreal.command(name="integration", about="Run integration tests")
def test_integration():
    """Run integration tests (includes external dependencies like Ollama)"""
    print("Running integration tests...")
    # First run the LLM integration tests
    llm_test_script = PROJECT_ROOT / "crates" / "mimir-dm-llm" / "run_integration_tests.sh"
    if llm_test_script.exists():
        print("Running LLM integration tests...")
        subprocess.run(["bash", str(llm_test_script)], cwd=llm_test_script.parent)
    
    # Run other integration tests (now fully isolated with in-memory databases)
    print("Running integration tests...")
    result = subprocess.run(
        ["cargo", "test", "--workspace", "--", "--ignored"],
        cwd=PROJECT_ROOT,
        capture_output=False
    )
    return result.returncode

@test
@angreal.command(name="coverage", about="Run tests with coverage reporting")
@angreal.argument(name="html", short="h", takes_value=False, help="Generate HTML coverage report")
def test_coverage(html: bool = False):
    """Run tests with code coverage using cargo-tarpaulin"""
    print("Running tests with coverage...")
    
    # Check if cargo-tarpaulin is installed
    check_result = subprocess.run(
        ["cargo", "tarpaulin", "--version"],
        capture_output=True,
        cwd=PROJECT_ROOT
    )
    
    if check_result.returncode != 0:
        print("cargo-tarpaulin not found. Installing...")
        subprocess.run(["cargo", "install", "cargo-tarpaulin"], cwd=PROJECT_ROOT)
    
    cmd = ["cargo", "tarpaulin", "--workspace"]
    if html:
        cmd.extend(["--out", "Html"])
        print("HTML coverage report will be generated in target/tarpaulin/")
    
    result = subprocess.run(cmd, cwd=PROJECT_ROOT, capture_output=False)
    return result.returncode

@test
@angreal.command(name="watch", about="Run tests in watch mode")
@angreal.argument(name="crate", short="c", help="Watch tests for a specific crate")
def test_watch(crate: str = None):
    """Run tests in watch mode using cargo-watch"""
    print("Starting test watcher...")
    
    # Check if cargo-watch is installed
    check_result = subprocess.run(
        ["cargo", "watch", "--version"],
        capture_output=True,
        cwd=PROJECT_ROOT
    )
    
    if check_result.returncode != 0:
        print("cargo-watch not found. Installing...")
        subprocess.run(["cargo", "install", "cargo-watch"], cwd=PROJECT_ROOT)
    
    cmd = ["cargo", "watch", "-x"]
    if crate:
        cmd.append(f"test -p {crate}")
    else:
        cmd.append("test --workspace")
    
    result = subprocess.run(cmd, cwd=PROJECT_ROOT, capture_output=False)
    return result.returncode