# Contributing to Siragugal Film Studio

Thank you for your interest in contributing to **Siragugal Film Studio**! 🎬

---

## 1. Development Process & Mandatory Standards

Before contributing, please review our core governance documents:
- [CONSTITUTION.md](file:///D:/SiragugalFilmStudio/CONSTITUTION.md): Frozen Architecture v1.2.0.
- [ENGINEERING_FOUNDATION.md](file:///D:/SiragugalFilmStudio/docs/governance/ENGINEERING_FOUNDATION.md): Naming, coding style, git strategy, and Definition of Done.

---

## 2. Pull Request Workflow

1. **Fork & Branch**: Create a feature branch off `develop` using Conventional Commits naming (`feature/mXX-description` or `fix/issue-id`).
2. **License Header**: Ensure all new source files contain the mandatory Apache/MIT copyright header.
3. **Commit Messages**: Follow Conventional Commits (`feat: ...`, `fix: ...`, `docs: ...`).
4. **Validation**: Ensure `pnpm lint`, `pnpm format`, `cargo check --workspace`, and `cargo test --workspace` pass with zero errors.
5. **PR Checklist**: Complete all items in `.github/PULL_REQUEST_TEMPLATE.md`.
