# Fynbos Technologies Design System

**Version 1.1** | *Modern CFD/CAE Interface Design Language*

## Table of Contents

1. [Brand Identity](#brand-identity)
2. [Design Principles](#design-principles)
3. [Visual Language](#visual-language)
4. [Component Architecture](#component-architecture)
5. [Layout Philosophy](#layout-philosophy)
6. [CSS Architecture](#css-architecture)

---

## Brand Identity

### Company Overview
**Fynbos Technologies** provides modern, compute-agnostic CFD and CAE solutions with an ecosystem of open numerical libraries optimized for GPU computing and remote compute capabilities.

### Brand Positioning
- **Modern**: Cutting-edge computational methods with contemporary interface design
- **Efficient**: Performance-first approach prioritizing engineering productivity
- **Mathematical Elegance**: Clean, precise, systematic visual relationships
- **Cape Town Heritage**: Sophisticated, natural, grounded in South African innovation

### Target Audience
- CFD Engineers and computational specialists requiring precision tools
- Conservative engineering professionals who value reliability and clarity
- Researchers needing high-performance computing with minimal cognitive overhead
- Teams requiring remote compute simulation tools with collaborative workflows

---

## Design Principles

### 1. Mathematical Precision
Visual systems should reflect the mathematical rigor of CFD work. Consistent spacing grids, predictable proportions, and systematic visual relationships create interfaces that feel as precise as the calculations they support.

### 2. Technical Clarity
Engineering workflows demand clear information hierarchy and minimal cognitive load. Every visual decision should prioritize the rapid comprehension of complex numerical data and simulation states.

### 3. Understated Elegance
Professional sophistication without flashiness. The interface should feel like a precision instrument—trustworthy, refined, and built to support long engineering work sessions without fatigue.

### 4. Performance First
Fast, responsive interactions optimized for desktop workflows. The design system should enable efficient information density while maintaining clarity across large displays and complex datasets.

---

## Visual Language

### Color Philosophy

#### Primary: Table Bay Blue
The foundation of the interface, representing the dependable precision of engineering work. A sophisticated blue that feels both modern and trustworthy, suitable for long CFD work sessions.

- **Light variant** `#E0F2FE`: Calm backgrounds and subtle data visualization elements
- **Standard** `#0891B2`: Active states, primary actions, and key interface elements  
- **Dark variant** `#164E63`: Emphasis and high-contrast elements

#### Accent: Emerald
A controlled accent that provides positive feedback and progress indication. Inspired by Cape Town's natural landscape, this green supports the brand's South African heritage while maintaining professional restraint.

- **Light variant** `#CFFAFE`: Success states and gentle completion indicators
- **Standard** `#10B981`: Progress highlights and positive user feedback
- **Dark variant** `#065F46`: Accent elements in dark themes

#### Neutrals: Table Mountain Grays
A carefully balanced neutral system supporting both light and dark themes. These grays ensure optimal contrast for numerical data presentation and reduce eye strain during extended engineering sessions.

**Surface Colors:**
- **Lightest** `#FFFFFF`: Primary page backgrounds providing calm visual foundation
- **Light** `#F8FAFC`: Card and panel backgrounds for content organization
- **Medium** `#E2E8F0`: Borders, dividers, and subtle separations
- **Inactive** `#94A3B8`: Inactive elements and disabled states

**Dark Theme Surfaces:**
- **Darkest** `#0F172A`: Dark theme page backgrounds
- **Dark** `#1E293B`: Dark theme card and panel backgrounds
- **Medium** `#334155`: Dark theme borders and dividers
- **Inactive** `#475569`: Dark theme inactive elements

**Content Colors:**
- **Primary** `#2D3748` / `#E2E8F0`: Main text and critical information requiring immediate attention
- **Secondary** `#475569` / `#94A3B8`: Supporting text, labels, and contextual information  
- **Tertiary** `#94A3B8` / `#64748B`: Muted text, placeholders, and background information

#### Semantic Colors
Purpose-driven colors that communicate system states clearly:
- **Success** `#059669`: Converged solutions, successful operations, positive outcomes
- **Warning** `#D97706`: Non-critical issues, cautions requiring attention
- **Error** `#DC2626`: Failed simulations, critical alerts, blocking issues

### Color Relationships
The color system creates clear information hierarchy while maintaining visual harmony. Primary blues establish authority and trust, accent greens provide positive reinforcement, and the neutral system ensures content legibility across all viewing conditions.

### Theme Adaptability
The color system seamlessly transitions between light and dark themes, maintaining contrast ratios and semantic meaning across all viewing preferences. Dark themes use inverted surface relationships while preserving the core brand colors and their functional meanings.

### Typography Strategy

#### Font Selection
- **Primary**: Modern sans-serif optimized for engineering interfaces
- **Technical**: Monospace for numerical data, code, and precise measurements

#### Information Hierarchy
Clear typographic scales supporting complex engineering data presentation:
- **Headlines**: Establishing context and major sections
- **Content**: Readable body text for documentation and descriptions  
- **Data**: Monospace presentation for numerical values and technical specifications
- **Metadata**: Supporting information and status indicators

### Spacing Philosophy
Mathematical 8px grid system ensuring visual harmony and systematic layouts. Spacing decisions should feel predictable and create natural groupings of related information.

---

## Business Logic Architecture

For business logic implementation, we follow Domain Driven Design (DDD) principles to ensure a clear separation of concerns and maintainable code structure:

### Domain-Driven Design Layers

- **Domain Layer** (`src/domain/`): Core business logic and entities independent of external concerns
  - Contains business rules, domain entities, and value objects
  - No dependencies on external frameworks or infrastructure
  - Represents the heart of the CFD/CAE workflow logic

- **Application Layer** (`src/application/`): Use cases and application services coordinating domain operations
  - Orchestrates domain objects to fulfill specific use cases
  - Handles cross-cutting concerns like transactions and security
  - Provides the interface between presentation and domain layers

- **Infrastructure Layer** (`src/infrastructure/`): External concerns like API clients, data persistence, and external services
  - Implements interfaces defined by the domain layer
  - Handles communication with external systems and APIs
  - Contains adapters for third-party services and frameworks

- **Presentation Layer** (`src/components/`): UI components and user interaction handling
  - Implements the atomic design component hierarchy
  - Handles user interface concerns and user experience
  - Translates user actions into application layer commands

This layered architecture ensures that business rules remain isolated and testable while providing clear boundaries between different architectural concerns. The domain layer forms the core, with dependencies flowing inward, maintaining the independence of business logic from technical implementation details.

## Component UI Architecture

### Atomic Design Structure

The design system follows atomic design principles for UI organization, organizing components from simple to complex:


```
src/components/
├── atoms/          # Fundamental building blocks
├── molecules/      # Simple functional combinations  
├── organisms/      # Complex functional components
├── templates/      # Page layout structures
```

### Component Characteristics

#### Atoms
Fundamental elements that embody core design principles:
- Typography elements ensuring consistent text presentation
- Control elements providing reliable interaction patterns
- Data elements displaying numerical information with appropriate precision
- Indicator elements communicating system and simulation states

#### Molecules  
Functional combinations addressing specific CFD/CAE workflows:
- Parameter input groups streamlining simulation configuration
- Status displays providing clear feedback on operation states
- Metric presentations organizing numerical results effectively

#### Organisms
Comprehensive components supporting complete engineering tasks:
- Simulation management for end-to-end workflow support
- Results analysis with comprehensive data exploration tools
- Workspace organization ensuring efficient information architecture

---

## Layout Philosophy

### Desktop-First Strategy
Primary focus on desktop browser experiences with progressive simplification:

- **Desktop**: Maximum information density, multiple simultaneous workflows, complex data visualization
- **Tablet**: Reduced complexity, essential workflows maintained
- **Mobile**: Minimal interface for monitoring and basic status checking only

### Information Architecture
Engineering workflows require careful attention to information hierarchy:

1. **Critical Status**: Simulation health, convergence, errors
2. **Primary Data**: Current values, progress, key metrics  
3. **Secondary Information**: Historical data, configuration details
4. **Contextual Actions**: Workflow-specific tools and options

### Spatial Relationships
Consistent spacing and grouping patterns that support engineering thinking:
- Related parameters grouped spatially
- Clear separation between different simulation contexts
- Progressive disclosure for complex configuration options

---

## CSS Architecture

### Framework Choice: Tailwind CSS

**Decision:** Use Tailwind CSS for all component styling.

**Rationale:** Eliminates CSS hierarchy/specificity issues. Atomic utilities make refactoring simple via find/replace. Built-in design system prevents inconsistent spacing and colors.

**Conventions:**
- Use Tailwind utilities directly in component classes
- Prefer utility classes over custom CSS
- Use `@layer components` only for complex reusable patterns
- Configure brand colors in tailwind.config.js

```rust
// Example: Button component with Tailwind utilities
view! {
    <button class="bg-blue-500 hover:bg-blue-600 px-4 py-2 text-white rounded">
        "Click me"
    </button>
}
```