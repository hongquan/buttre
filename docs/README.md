# buttre Documentation

> Complete documentation for developers and contributors

**Last Updated**: 2026-01-08

---

## Quick Start for Developers

**New to buttre?** Read these in order:

1. **[README.md](../README.md)** - Project overview and quick start
2. **[ARCHITECTURE.md](ARCHITECTURE.md)** - System architecture and design
3. **[CODING_GUIDE.md](CODING_GUIDE.md)** - How to write code in this project
4. **[ROADMAP.md](ROADMAP.md)** - Project roadmap and future plans

---

## Documentation Structure

```
docs/
├── README.md                      # This file
├── ARCHITECTURE.md                # ⭐ System architecture (comprehensive)
├── CODING_GUIDE.md                # ⭐ Coding standards and patterns
├── ROADMAP.md                     # ⭐ Project roadmap and timeline
├── PIPELINE_ARCHITECTURE.md       # Detailed 7-stage pipeline docs
├── VIETNAMESE_ACCENT.md           # Vietnamese orthography specification
├── MANUAL_TESTING_GUIDE.md        # How to manually test TSF DLL
├── FFI_SAFETY_GUIDE.md            # FFI safety patterns (for macOS/Linux)
├── agent.md                       # AI agent configuration
├── nom/                           # Hán Nôm documentation
│   ├── NOM_DATABASE_OPTIMIZATION.md
│   ├── NOM_DEVELOPER_GUIDE.md
│   ├── NOM_INPUT_ANALYSIS.md
│   └── NOM_REFERENCE_ANALYSIS.md
└── archive/                       # Historical docs (for reference)
    ├── 4_IME_COMPARISON.md
    ├── COMPLETE_CRATE_ANALYSIS.md
    ├── DEPENDENCY_ANALYSIS.md
    ├── GONHANH_VS_buttre_COMPARISON.md
    ├── PLATFORM_CRATE_ANALYSIS.md
    ├── PHASE2_PROGRESS.md
    ├── TSF_IMPLEMENTATION_PROGRESS.md
    ├── CONFIG_DRIVEN_ARCHITECTURE.md
    ├── FOLDER_STRUCTURE_RECOMMENDATION.md
    ├── LINUX_IMPLEMENTATION_PLAN.md
    ├── MACOS_IMPLEMENTATION_PLAN.md
    ├── WINDOWS_MIGRATION_PLAN.md
    └── STATE_MANAGEMENT_IN_3_CRATES.md
```

---

## Core Documentation

### [ARCHITECTURE.md](ARCHITECTURE.md)

**What**: Complete architectural overview of buttre

**Contains**:
- System overview and high-level architecture
- Crate structure (buttre-engine, buttre-core, buttre-platform, buttre-test)
- 7-stage processing pipeline architecture
- State management and data flow
- Platform integration (Windows TSF, macOS, Linux)
- Design principles

**When to read**:
- ⭐ **FIRST** - Before contributing code
- When you need to understand the big picture
- When planning new features

---

### [CODING_GUIDE.md](CODING_GUIDE.md)

**What**: Coding standards and patterns extracted from actual codebase

**Contains**:
- Project setup and workspace structure
- Rust coding standards (error handling, documentation, naming)
- Common patterns (Pipeline Stage, Action Enum, Configuration)
- Testing guidelines (unit tests, integration tests)
- Error handling best practices
- Performance guidelines
- How to add new features (step-by-step)

**When to read**:
- ⭐ **BEFORE** writing code
- When you're unsure about coding style
- Before submitting a PR

---

### [ROADMAP.md](ROADMAP.md)

**What**: Strategic plan for buttre development

**Contains**:
- Current status and completed features
- Phase-by-phase roadmap (Q1-Q4 2026, 2027)
- Platform priorities (Windows, macOS, Linux)
- Hán Nôm support plan
- Advanced features under consideration
- Technical debt and known issues
- Timeline and deliverables

**When to read**:
- When you want to contribute (find what's planned)
- When proposing new features
- To understand project direction

---

## Specialized Documentation

### [PIPELINE_ARCHITECTURE.md](PIPELINE_ARCHITECTURE.md)

**What**: Detailed documentation of the 7-stage processing pipeline

**Contains**:
- Stage-by-stage breakdown
- Flow control and decision trees
- State management in TypingContext
- Performance optimizations
- Real examples (typing "người")

**When to read**:
- When working on the engine (buttre-engine)
- When debugging input processing
- When adding new stages

---

### [VIETNAMESE_ACCENT.md](VIETNAMESE_ACCENT.md)

**What**: Vietnamese orthography specification

**Contains**:
- Phase 1: Character transformation (mũ, râu, trăng)
- Phase 2: Parser & normalization (initial, vowel core, final)
- Phase 3: Anchor logic (tone placement rules)
- Priority rules for tone positioning
- Test cases

**When to read**:
- When working on Vietnamese input logic
- When fixing tone positioning bugs
- When validating orthography rules

---

### [MANUAL_TESTING_GUIDE.md](MANUAL_TESTING_GUIDE.md)

**What**: How to manually test the Windows TSF DLL

**Contains**:
- Build output locations
- Registration commands
- Testing in Notepad/Word/browsers
- Common issues and solutions

**When to read**:
- When testing Windows TSF changes
- When debugging TSF integration
- Before releasing Windows builds

---

### [FFI_SAFETY_GUIDE.md](FFI_SAFETY_GUIDE.md)

**What**: FFI safety patterns for platform integration

**Contains**:
- Achieving zero unsafe in FFI
- Objective-C ↔ Rust FFI patterns (for macOS)
- Using windows-rs safely
- Best practices for platform bindings

**When to read**:
- When working on macOS/Linux platform integration
- When adding unsafe code
- When reviewing FFI code

---

### [agent.md](agent.md)

**What**: AI agent configuration for Claude Code

**Contains**:
- Project metadata (tech stack, crate structure)
- Code quality rules
- Workflow phases
- Constraints and conventions
- Current focus and status
- Rust coding rules (mandatory)

**When to read**:
- When using AI agents to assist with development
- To understand project conventions quickly
- For AI context setup

---

## Specialized Topics

### Hán Nôm Documentation ([nom/](nom/))

Documentation specific to Hán Nôm (chữ Nôm) input support.

**Files**:
- **[NOM_DEVELOPER_GUIDE.md](nom/NOM_DEVELOPER_GUIDE.md)** - Developer guide for Nôm input
- **[NOM_DATABASE_OPTIMIZATION.md](nom/NOM_DATABASE_OPTIMIZATION.md)** - Database design and optimization
- **[NOM_INPUT_ANALYSIS.md](nom/NOM_INPUT_ANALYSIS.md)** - Input method analysis
- **[NOM_REFERENCE_ANALYSIS.md](nom/NOM_REFERENCE_ANALYSIS.md)** - Analysis of weasel-hannom-keyboard

**When to read**: When working on Phase 4 (Hán Nôm support)

---

### Archive ([archive/](archive/))

Historical documentation kept for reference. These docs may be outdated but contain valuable context.

**Analysis & Comparisons**:
- [4_IME_COMPARISON.md](archive/4_IME_COMPARISON.md) - Comparison of 4 Vietnamese IMEs
- [GONHANH_VS_buttre_COMPARISON.md](archive/GONHANH_VS_buttre_COMPARISON.md) - GoNhanh pipeline comparison
- [DEPENDENCY_ANALYSIS.md](archive/DEPENDENCY_ANALYSIS.md) - GoNhanh zero-dependency analysis
- [COMPLETE_CRATE_ANALYSIS.md](archive/COMPLETE_CRATE_ANALYSIS.md) - 3-crate refactor analysis
- [PLATFORM_CRATE_ANALYSIS.md](archive/PLATFORM_CRATE_ANALYSIS.md) - buttre-platform crate analysis

**Progress Reports** (outdated):
- [PHASE2_PROGRESS.md](archive/PHASE2_PROGRESS.md) - Phase 2 progress (2025-12-19)
- [TSF_IMPLEMENTATION_PROGRESS.md](archive/TSF_IMPLEMENTATION_PROGRESS.md) - TSF progress (Dec 2024)

**Plans** (now consolidated into ROADMAP.md):
- [LINUX_IMPLEMENTATION_PLAN.md](archive/LINUX_IMPLEMENTATION_PLAN.md) - Linux/IBus implementation
- [MACOS_IMPLEMENTATION_PLAN.md](archive/MACOS_IMPLEMENTATION_PLAN.md) - macOS/IMKit implementation
- [WINDOWS_MIGRATION_PLAN.md](archive/WINDOWS_MIGRATION_PLAN.md) - Windows backend migration
- [CONFIG_DRIVEN_ARCHITECTURE.md](archive/CONFIG_DRIVEN_ARCHITECTURE.md) - Architecture redesign
- [FOLDER_STRUCTURE_RECOMMENDATION.md](archive/FOLDER_STRUCTURE_RECOMMENDATION.md) - Folder structure
- [STATE_MANAGEMENT_IN_3_CRATES.md](archive/STATE_MANAGEMENT_IN_3_CRATES.md) - State management design

**When to read**: For historical context or to understand past decisions

---

## Documentation Maintenance

### When to Update Documentation

**ARCHITECTURE.md**: Update when:
- Adding new crates
- Changing crate responsibilities
- Modifying pipeline architecture
- Adding new platforms

**CODING_GUIDE.md**: Update when:
- Establishing new coding patterns
- Changing naming conventions
- Adding new testing guidelines
- Discovering anti-patterns

**ROADMAP.md**: Update when:
- Completing phases
- Adjusting timeline
- Adding/removing features
- Reprioritizing platforms

**PIPELINE_ARCHITECTURE.md**: Update when:
- Adding/removing stages
- Changing stage responsibilities
- Modifying flow control

**VIETNAMESE_ACCENT.md**: Update when:
- Fixing orthography bugs
- Adding new rules
- Clarifying specifications

### Documentation Standards

**Format**: All docs use GitHub-flavored Markdown

**Style**:
- Use clear, concise language
- Include code examples from actual codebase
- Provide "When to read" guidance
- Keep docs up-to-date with code

**File Naming**:
- Use `UPPERCASE_WITH_UNDERSCORES.md` for major docs
- Use `lowercase-with-hyphens.md` for subdirectories

**Structure**:
- Start with brief description
- Include table of contents for long docs
- Use headers for navigation
- Add "Last Updated" date

---

## Contributing to Documentation

Documentation improvements are highly valued!

**How to contribute**:

1. **Fix typos/errors**: Submit PR directly
2. **Clarify existing docs**: Submit PR with explanation
3. **Add new sections**: Discuss in issue first, then PR
4. **Add new docs**: Discuss in issue first (avoid duplication)

**Good documentation contributions**:
- Fix outdated information
- Add missing examples
- Clarify confusing sections
- Add diagrams/visuals
- Improve navigation
- Add "when to read" guidance

**Documentation PR checklist**:
- [ ] Information is accurate (verified against code)
- [ ] Examples are from actual codebase
- [ ] Formatting is consistent
- [ ] Links are working
- [ ] "Last Updated" date is current
- [ ] No spelling/grammar errors

---

## Quick Reference Card

| Task | Read This |
|------|-----------|
| I want to understand buttre's architecture | [ARCHITECTURE.md](ARCHITECTURE.md) |
| I want to write code for buttre | [CODING_GUIDE.md](CODING_GUIDE.md) |
| I want to contribute a feature | [ROADMAP.md](ROADMAP.md) |
| I'm working on the engine | [PIPELINE_ARCHITECTURE.md](PIPELINE_ARCHITECTURE.md) |
| I'm fixing tone positioning | [VIETNAMESE_ACCENT.md](VIETNAMESE_ACCENT.md) |
| I'm testing Windows TSF | [MANUAL_TESTING_GUIDE.md](MANUAL_TESTING_GUIDE.md) |
| I'm adding macOS/Linux support | [FFI_SAFETY_GUIDE.md](FFI_SAFETY_GUIDE.md) |
| I'm working on Hán Nôm | [nom/NOM_DEVELOPER_GUIDE.md](nom/NOM_DEVELOPER_GUIDE.md) |
| I want historical context | [archive/](archive/) |

---

## Questions?

- **General questions**: [GitHub Discussions](https://github.com/vi-group/buttre/discussions)
- **Bug reports**: [GitHub Issues](https://github.com/vi-group/buttre/issues)
- **Documentation issues**: [GitHub Issues](https://github.com/vi-group/buttre/issues) (label: documentation)

---

**Last Updated**: 2026-01-08

_Thank you for reading the documentation! Your attention to detail helps make buttre better._
