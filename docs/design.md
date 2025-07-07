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
```css
--primary-light: #E0F2FE;  /* Calm backgrounds, data visualization */
--primary:       #0891B2;  /* Active states, primary actions */
--primary-dark:  #164E63;  /* Emphasis, dark theme foundation */
```

#### Accent: Emerald
```css
--accent-light: #CFFAFE;   /* Success states, completion indicators */
--accent:       #10B981;   /* Progress, highlights, positive feedback */
--accent-dark:  #065F46;   /* Dark theme accent variations */
```

#### Neutrals: Table Mountain Grays
A carefully calibrated neutral palette supporting both light and dark themes, ensuring optimal contrast for numerical data and extended viewing sessions.

```css
/* Light Theme */
--surface-0:      #F8FAFC;  /* Page background */
--surface-1:      #FFFFFF;  /* Card background */
--surface-2:      #E2E8F0;  /* Borders, dividers */
--surface-3:      #94A3B8;  /* Inactive elements */
--text-primary:   #2D3748;  /* Main content text */
--text-secondary: #475569;  /* Secondary text, labels */
--text-tertiary:  #94A3B8;  /* Muted text, placeholders */

/* Dark Theme */
--dark-surface-0:      #0F172A;  /* Page background */
--dark-surface-1:      #1E293B;  /* Card background */
--dark-surface-2:      #334155;  /* Borders, dividers */
--dark-surface-3:      #475569;  /* Inactive elements */
--dark-text-primary:   #E2E8F0;  /* Main content text */
--dark-text-secondary: #94A3B8;  /* Secondary text */
--dark-text-tertiary:  #64748B;  /* Muted text */
```

#### Semantic Colors
```css
--success: #059669;  /* Successful operations, converged solutions */
--warning: #D97706;  /* Cautions, non-critical issues */
--error:   #DC2626;  /* Errors, failed simulations, critical alerts */
```

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

## Component Architecture

### Atomic Design Structure

The design system follows atomic design principles, organizing components from simple to complex:

```
src/components/
├── atoms/          # Fundamental building blocks
├── molecules/      # Simple functional combinations  
├── organisms/      # Complex functional components
├── templates/      # Page layout structures
└── pages/          # Complete page implementations
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