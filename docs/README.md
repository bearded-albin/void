# VOID Documentation

Comprehensive guides for developing and using the VOID energy lattice simulator.

## For Developers

### [DEVELOPMENT.md](DEVELOPMENT.md)
**Complete implementation roadmap with phase-by-phase instructions**

- Module-by-module implementation order
- Detailed TODO lists with code examples
- Testing strategies
- Performance optimization tips
- CI/CD guidelines

**Start here if you're implementing the library from scratch.**

### [PHYSICS_REFERENCE.md](PHYSICS_REFERENCE.md)
**Mathematical foundations and equations**

- Complete physics model specification
- Evolution equations (local and spatial)
- Conservation laws
- Oscillation mode analysis
- Pattern metric formulas
- Numerical methods

**Reference this when implementing physics modules.**

### [../TODO.md](../TODO.md)
**Master checklist with 200+ actionable items**

- Organized by phases and modules
- Includes milestones and deadlines
- Track your progress

**Use this as your daily todo list.**

## For Users

### Getting Started

See [GETTING_STARTED.md](GETTING_STARTED.md) for:
- Installation instructions
- First simulation walkthrough
- TUI dashboard tutorial
- Common pitfalls and solutions

### API Documentation

Generate API docs locally:
```bash
cargo doc --open --no-deps
```

## Repository Structure

```
docs/
├── README.md              # This file
├── DEVELOPMENT.md         # Implementation guide
├── PHYSICS_REFERENCE.md   # Equations and formulas
└── GETTING_STARTED.md     # User tutorial (coming soon)
```

## Contributing

See [DEVELOPMENT.md](DEVELOPMENT.md) for contribution guidelines.

## Questions?

Open an issue on GitHub or reach out to the maintainers.
