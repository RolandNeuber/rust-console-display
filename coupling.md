# Coupling Analysis Report

## Executive Summary

**Health Grade**: 🟡 C (Room for improvement)

**Why this grade**: Driven by 2 high/critical issue(s), led by High Afferent Coupling (4), Hidden Coupling (3), High Efferent Coupling (3); distance is the largest contributor. Volatility/churn contributes through 3 issue(s).

| Metric | Value |
|--------|-------|
| Files Analyzed | 61 |
| Total Modules | 60 |
| Total Couplings | 685 |
| Balance Score | 0.73/1.00 |
| Balanced | 131 (19%) |
| Issues Surfaced | 12 |

**⚠️ Action Required**

- 🟠 **2 High** priority issues should be addressed soon
- 🟡 10 Medium priority issues to review

## 🔧 Refactoring Priorities

### Immediate Actions

**1. 🟠 `112 dependents` → `console-display::widget`**

- **Issue**: High Afferent Coupling - Module console-display::widget is depended on by 112 other components (threshold: 20)
- **Why**: A module that many others depend on is hard to change. Any modification risks breaking dependents.
- **Action**: Introduce trait `WidgetInterface` with methods: // Define stable public API
- **Balance Score**: 0.00

**2. 🟠 `43 dependents` → `console-display::color`**

- **Issue**: High Afferent Coupling - Module console-display::color is depended on by 43 other components (threshold: 20)
- **Why**: A module that many others depend on is hard to change. Any modification risks breaking dependents.
- **Action**: Introduce trait `ColorInterface` with methods: // Define stable public API
- **Balance Score**: 0.28

**3. 🟡 `display::character_display` → `drawing::ellipse`**

- **Issue**: Hidden Coupling - Strong temporal co-change without code dependency (75% ratio, 3 co-changes)
- **Why**: Files frequently change together without an explicit code dependency. This suggests implicit shared knowledge or a missing abstraction.
- **Action**: Extract a shared abstraction or make the dependency explicit
- **Balance Score**: 0.25

**4. 🟡 `display::character_display` → `drawing::line`**

- **Issue**: Hidden Coupling - Strong temporal co-change without code dependency (75% ratio, 3 co-changes)
- **Why**: Files frequently change together without an explicit code dependency. This suggests implicit shared knowledge or a missing abstraction.
- **Action**: Extract a shared abstraction or make the dependency explicit
- **Balance Score**: 0.25

**5. 🟡 `display::character_display` → `drawing::rectangle`**

- **Issue**: Hidden Coupling - Strong temporal co-change without code dependency (75% ratio, 3 co-changes)
- **Why**: Files frequently change together without an explicit code dependency. This suggests implicit shared knowledge or a missing abstraction.
- **Action**: Extract a shared abstraction or make the dependency explicit
- **Balance Score**: 0.25

## Issue Triage

### Structural — act now (12)

- **High Afferent Coupling** `112 dependents` → `console-display::widget`
- **High Afferent Coupling** `43 dependents` → `console-display::color`
- **Hidden Coupling** `display::character_display` → `drawing::ellipse`
- **Hidden Coupling** `display::character_display` → `drawing::line`
- **Hidden Coupling** `display::character_display` → `drawing::rectangle`
- **God Module** `display::character_display` → `6 functions, 1 types, 26 impls`
- **God Module** `display::pixel_display` → `7 functions, 1 types, 24 impls`
- **High Afferent Coupling** `27 dependents` → `console-display::error`
- **High Efferent Coupling** `console-display::display::character_display` → `20 dependencies`
- **High Efferent Coupling** `console-display::display::pixel_display` → `18 dependencies`
- **High Afferent Coupling** `22 dependents` → `console-display::pixel::monochrome`
- **High Efferent Coupling** `console-display::widget::two::overlay` → `16 dependencies`

### Volatility-driven — may settle (0)


> Expected-by-design patterns (entrypoint fan-out, stable central abstractions) are downgraded and omitted here.

## Issues by Category

### High Afferent Coupling (4 instances)

> A module that many others depend on is hard to change. Any modification risks breaking dependents.

| Severity | Source | Target | Action |
|----------|--------|--------|--------|
| High | `112 dependents` | `console-display::widget` | Introduce trait `WidgetInterface` with m... |
| High | `43 dependents` | `console-display::color` | Introduce trait `ColorInterface` with me... |
| Medium | `27 dependents` | `console-display::error` | Introduce trait `ErrorInterface` with me... |
| Medium | `22 dependents` | `...lay::pixel::monochrome` | Introduce trait `MonochromeInterface` wi... |

### High Efferent Coupling (3 instances)

> A module depending on too many others is fragile and hard to test. Changes anywhere affect this module.

| Severity | Source | Target | Action |
|----------|--------|--------|--------|
| Medium | `...lay::character_display` | `20 dependencies` | Split into modules: console-display::dis... |
| Medium | `...display::pixel_display` | `18 dependencies` | Split into modules: console-display::dis... |
| Medium | `...::widget::two::overlay` | `16 dependencies` | Split into modules: console-display::wid... |

### Hidden Coupling (3 instances)

> Files frequently change together without an explicit code dependency. This suggests implicit shared knowledge or a missing abstraction.

| Severity | Source | Target | Action |
|----------|--------|--------|--------|
| Medium | `...lay::character_display` | `drawing::ellipse` | Extract a shared abstraction or make the... |
| Medium | `...lay::character_display` | `drawing::line` | Extract a shared abstraction or make the... |
| Medium | `...lay::character_display` | `drawing::rectangle` | Extract a shared abstraction or make the... |

### God Module (2 instances)

> Module has too many responsibilities - too many functions, types, or implementations. Consider splitting into focused, cohesive modules. (SRP violation)

| Severity | Source | Target | Action |
|----------|--------|--------|--------|
| Medium | `...lay::character_display` | `...ons, 1 types, 26 impls` | Split into modules: display::character_d... |
| Medium | `display::pixel_display` | `...ons, 1 types, 24 impls` | Split into modules: display::pixel_displ... |

## Coupling Distribution

### By Integration Strength

| Strength | Count | % | Description |
|----------|-------|---|-------------|
| Contract | 121 | 18% | Depends on traits/interfaces only |
| Model | 369 | 54% | Uses data types/structs |
| Functional | 175 | 26% | Calls specific functions |
| Intrusive | 20 | 3% | Accesses internal details |

### By Distance

| Distance | Count | % |
|----------|-------|---|
| Same Module (close) | 132 | 19% |
| Different Module | 115 | 17% |
| External Crate (far) | 438 | 64% |

### By Volatility (Internal Couplings)

| Volatility | Count | % | Impact on Balance |
|------------|-------|---|-------------------|
| Low (rarely changes) | 87 | 35% | No penalty |
| Medium (sometimes changes) | 160 | 65% | Moderate penalty |
| High (frequently changes) | 0 | 0% | Significant penalty |

### Worst Balanced Couplings

| Source | Target | Strength | Distance | Volatility | Score | Status |
|--------|--------|----------|----------|------------|-------|--------|
| `...der::shader_trait` | `...e-display::widget` | Model | Same Mod | Med | 0.56 | 🟡 Review |
| `...shader::composite` | `...e-display::widget` | Model | Same Mod | Med | 0.56 | 🟡 Review |
| `...::vertical_tiling` | `...e-display::widget` | Model | Same Mod | Med | 0.56 | 🟡 Review |
| `...get::two::overlay` | `...e-display::widget` | Model | Same Mod | Med | 0.56 | 🟡 Review |
| `...horizontal_tiling` | `...e-display::widget` | Model | Same Mod | Med | 0.56 | 🟡 Review |
| `...splay::color::rgb` | `...le-display::color` | Model | Same Mod | Med | 0.56 | 🟡 Review |
| `...::single::padding` | `...e-display::widget` | Model | Same Mod | Med | 0.56 | 🟡 Review |
| `...idget::single::uv` | `...e-display::widget` | Model | Same Mod | Med | 0.56 | 🟡 Review |
| `...::color::terminal` | `...le-display::color` | Model | Same Mod | Med | 0.56 | 🟡 Review |
| `...t::single::border` | `...e-display::widget` | Model | Same Mod | Med | 0.56 | 🟡 Review |
| `...play::color::argb` | `...le-display::color` | Model | Same Mod | Med | 0.56 | 🟡 Review |
| `...gle::shader::blur` | `...e-display::widget` | Model | Same Mod | Med | 0.56 | 🟡 Review |
| `...rawing::rectangle` | `...ay::drawing::line` | Model | Same Mod | Med | 0.56 | 🟡 Review |
| `...:two::alternative` | `...e-display::widget` | Model | Same Mod | Med | 0.56 | 🟡 Review |
| `...y::display_driver` | `...y::display_driver` | Model | Same Mod | Med | 0.56 | 🟡 Review |

*Showing 15 of 107 couplings*

## Module Statistics

| Module | Trait Impl | Inherent Impl | Internal Deps | External Deps |
|--------|------------|---------------|---------------|---------------|
| `display::character_display` | 24 | 2 | 31 | 2 |
| `display::pixel_display` | 22 | 2 | 30 | 3 |
| `widget::single::uv` | 11 | 1 | 24 | 3 |
| `widget::two::overlay` | 5 | 2 | 19 | 1 |
| `pixel::monochrome` | 6 | 6 | 20 | 0 |
| `widget::two::alternative` | 5 | 2 | 16 | 1 |
| `widget::single::double_buffer` | 7 | 1 | 15 | 1 |
| `widget::two::vertical_tiling` | 5 | 2 | 13 | 1 |
| `widget::two::horizontal_tiling` | 3 | 2 | 13 | 1 |
| `...traits::dynamic::set_pixels` | 1 | 0 | 13 | 1 |
| `widget::single::padding` | 3 | 1 | 12 | 1 |
| `drawing::ellipse` | 2 | 0 | 9 | 3 |
| `drawing::rectangle` | 4 | 0 | 10 | 2 |
| `...traits::dynamic::get_pixels` | 1 | 0 | 11 | 1 |
| `pixel::character` | 3 | 1 | 8 | 2 |
| `display::display_driver` | 1 | 1 | 8 | 2 |
| `pixel::color` | 5 | 0 | 10 | 0 |
| `color::terminal` | 7 | 0 | 10 | 0 |
| `display::traits::constant` | 0 | 0 | 5 | 4 |
| `color::argb` | 6 | 1 | 9 | 0 |

*Showing top 20 of 60 modules*

## Volatility Analysis

No high volatility files detected (threshold: >10 changes).

## Temporal Coupling (Co-Change Analysis)

Files that frequently change together in git commits, indicating implicit coupling
beyond what code structure reveals.

### Strong Temporal Coupling (>50% co-change ratio)

⚠️ These pairs may share implicit knowledge (business logic, assumptions, data formats).

| File A | File B | Co-changes | Ratio |
|--------|--------|------------|-------|
| `src/drawing.rs` | `src/drawing/line.rs` | 4 | 100% |
| `src/drawing/ellipse.rs` | `src/drawing/rectangle.rs` | 4 | 100% |
| `src/drawing/line.rs` | `src/drawing/rectangle.rs` | 4 | 100% |
| `src/drawing.rs` | `src/drawing/ellipse.rs` | 4 | 100% |
| `src/widget/two_widget/alternative_widget.rs` | `src/widget/two_widget/vertical_tiling_widget.rs` | 4 | 100% |

*... and 70 more (use --all)*

## Circular Dependencies

✅ No circular dependencies detected.

## Balance Guidelines

The goal is **balanced coupling**, not zero coupling.

### Ideal Patterns ✅

| Pattern | Example | Why It Works |
|---------|---------|--------------|
| Strong + Close | `impl` blocks in same module | Cohesion within boundaries |
| Weak + Far | Trait impl for external crate | Loose coupling across boundaries |

### Problematic Patterns ❌

| Pattern | Problem | Solution |
|---------|---------|----------|
| Strong + Far | Global complexity | Introduce adapter or move closer |
| Strong + Volatile | Cascading changes | Add stable interface |
| Intrusive + Cross-boundary | Encapsulation violation | Extract trait API |

## Not Analyzed (blind spots)

Run-specific notes:
- 26 source file(s) failed to parse and were skipped; coupling inside them is unknown.

ℹ 4 structural blind spots not analyzed — see --blind-spots (or --json).

