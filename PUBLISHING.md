# Publishing to PyPI Guide

## Prerequisites

1. Create accounts on:
   - [PyPI](https://pypi.org/account/register/) (production)
   - [TestPyPI](https://test.pypi.org/account/register/) (testing)

2. Install required tools:

**On macOS (recommended - using pipx):**
```bash
brew install pipx
pipx install build
pipx install twine
```

**Or using pip with virtual environment:**
```bash
python3 -m venv ~/venv-publish
source ~/venv-publish/bin/activate
pip install build twine
```

**Or using pip with --break-system-packages (not recommended):**
```bash
pip install --break-system-packages build twine
```

## Step 1: Update Version

Update the version in both:
- `setup.py` (line 11)
- `pyproject.toml` (line 6)

## Step 2: Build the Package

```bash
# Clean previous builds
rm -rf dist/ build/ *.egg-info

# Build the package
python -m build
```

This creates:
- `dist/terminal-theatre-X.X.X.tar.gz` (source distribution)
- `dist/terminal_theatre-X.X.X-py3-none-any.whl` (wheel distribution)

## Step 3: Test on TestPyPI (Optional but Recommended)

```bash
# Upload to TestPyPI
twine upload --repository testpypi dist/*

# Test installation
pip install --index-url https://test.pypi.org/simple/ terminal-theatre
```

## Step 4: Upload to PyPI

```bash
# Upload to production PyPI
twine upload dist/*
```

You'll be prompted for your PyPI username and password.

## Step 5: Verify Installation

```bash
# Install from PyPI
pip install terminal-theatre

# Run the game
terminal-theatre
```

## Using API Tokens (Recommended)

Instead of username/password, use API tokens:

1. Go to PyPI Account Settings → API tokens
2. Create a new token with scope "Entire account" or specific project
3. Create `~/.pypirc`:

```ini
[pypi]
username = __token__
password = pypi-AgEIcHlwaS5vcmc...your-token-here

[testpypi]
username = __token__
password = pypi-AgENdGVzdC5weXBpLm9yZw...your-token-here
```

Then simply run:
```bash
twine upload dist/*
```

## Updating the Package

1. Make your changes
2. Update version number (use semantic versioning)
3. Rebuild and upload:

```bash
rm -rf dist/ build/ *.egg-info
python -m build
twine upload dist/*
```

## Version Numbering

Follow [Semantic Versioning](https://semver.org/):
- `1.0.0` - Initial release
- `1.0.1` - Patch (bug fixes)
- `1.1.0` - Minor (new features, backwards compatible)
- `2.0.0` - Major (breaking changes)

## Common Issues

**Issue**: `twine: command not found`
**Solution**: `pip install twine`

**Issue**: "File already exists on PyPI"
**Solution**: Increment version number

**Issue**: Authentication failed
**Solution**: Check credentials or use API token

## Quick Reference

```bash
# One-time setup
pip install build twine

# Every release
rm -rf dist/ build/ *.egg-info
python -m build
twine upload dist/*
```

## Users Will Install Like This

```bash
pip install terminal-theatre
terminal-theatre
```

That's it! 🎭
