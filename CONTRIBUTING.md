# Contributing to Terminal Theatre

Thank you for your interest in contributing to Terminal Theatre! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

We are committed to providing a welcoming and inclusive environment for all contributors. Please treat all community members with respect and kindness.

## How to Contribute

### Reporting Bugs

If you find a bug, please create an issue on GitHub with:
- A clear title and description
- Steps to reproduce the issue
- Expected behavior vs actual behavior
- Your environment details (OS, Python version, etc.)

### Suggesting Features

Feature suggestions are welcome! Please:
- Check existing issues to avoid duplicates
- Clearly describe the feature and its benefits
- Provide examples or use cases
- Consider implementation challenges

### Submitting Code Changes

1. **Fork the repository** and clone it locally
2. **Create a new branch** for your feature/fix:
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. **Make your changes** following the coding style guidelines below
4. **Write or update tests** for your changes
5. **Commit with clear messages**:
   ```bash
   git commit -m "Add feature: description of changes"
   ```
6. **Push to your fork** and create a Pull Request

## Coding Guidelines

### Python Style

- Follow PEP 8 conventions
- Use meaningful variable and function names
- Keep functions focused and modular
- Add docstrings to classes and functions
- Maximum line length: 100 characters

### Testing

- Write unit tests for new features
- Ensure all tests pass before submitting a PR:
  ```bash
  python -m pytest
  ```
- Maintain or improve test coverage

### Documentation

- Update README.md if adding major features
- Add docstrings to new functions and classes
- Comment complex logic
- Update QUICKSTART.md if affecting gameplay

## Development Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/Mirzabaig313/TERMINAL-THEATRE.git
   cd TERMINAL-THEATRE
   ```

2. Create a virtual environment:
   ```bash
   python3 -m venv .venv
   source .venv/bin/activate  # On Windows: .venv\Scripts\activate
   ```

3. Install dependencies:
   ```bash
   pip install -r requirements.txt
   ```

4. Run tests:
   ```bash
   python -m pytest
   ```

## Project Structure

```
TERMINAL-THEATRE/
├── engine/          # Core game engine
├── stories/         # Story content files
├── main.py          # Entry point
├── requirements.txt # Dependencies
├── test_*.py        # Test files
└── docs/            # Documentation (not pushed)
```

## Game Engine Architecture

The engine is built on several core modules:

- **game.py** - Main game state and logic
- **story.py** - Story structure and branching
- **renderer.py** - Terminal output rendering
- **animation.py** - Text animation effects
- **colors.py** - Color management
- **input_handler.py** - User input processing
- **save_manager.py** - Save/load functionality

## Adding New Stories

To add a new story:

1. Create a new Python file in `stories/`
2. Implement the story class inheriting from `Story`
3. Define scenes with branching paths
4. Add tests in a corresponding test file
5. Update the story selector in `main.py`

## Pull Request Process

1. Update documentation as needed
2. Add entry to changelog if applicable
3. Ensure CI/CD passes
4. Request review from maintainers
5. Address feedback promptly

## License

By contributing, you agree that your contributions will be licensed under the GNU Affero General Public License v3.0.

## Questions?

Feel free to open an issue for questions or discussions about the project.

---

Happy contributing!
