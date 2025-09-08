# TUI ERD Application Development Plan

## Overview
Build a Terminal User Interface (TUI) Entity-Relationship Diagram application in Rust with bidirectional synchronization between SQL code and visual diagram representation.

## Core Architecture
- **Dual-mode interface**: SQL editor view and ERD diagram view with real-time sync
- **Event-driven architecture** for handling user input and state changes
- **Separation of concerns**: Parser, Layout Engine, Renderer, and State Manager

## Development Phases

### Phase 1: Foundation
1. **Set up Rust project** with dependencies:
   - `ratatui` for TUI framework
   - `crossterm` for terminal manipulation
   - `sqlparser` for SQL parsing
   - `petgraph` for graph algorithms

2. **Core data structures**:
   - Entity (table representation)
   - Relationship (foreign key connections)
   - Layout coordinates and dimensions
   - Application state management

### Phase 2: SQL Processing
3. **SQL Parser implementation**:
   - Parse CREATE TABLE statements
   - Extract foreign key relationships
   - Support common SQL dialects (PostgreSQL, MySQL, SQLite)
   - Handle complex data types and constraints

### Phase 3: Layout Engine
4. **Graph layout algorithm**:
   - Force-directed layout for initial positioning
   - Crossing minimization using heuristics (Sugiyama algorithm variant)
   - Dynamic repositioning when entities are added/removed
   - Collision detection and resolution

### Phase 4: Rendering
5. **TUI rendering system**:
   - Entity boxes with columns and types
   - Relationship lines (1:1, 1:N, N:M) with crow's foot notation
   - Unicode box drawing characters
   - Highlight active elements
   - Color coding for different relationship types

### Phase 5: Interactivity
6. **Diagram editing features**:
   - Drag entities with keyboard (arrow keys or hjkl)
   - Add/remove tables
   - Modify relationships
   - Edit column properties inline
   - Undo/redo functionality

7. **SQL generation**:
   - Generate CREATE TABLE statements from diagram
   - Maintain foreign key constraints
   - Format SQL output with proper indentation
   - Support for indexes and constraints

### Phase 6: Synchronization
8. **Bidirectional sync**:
   - SQL changes update diagram immediately
   - Diagram changes regenerate SQL
   - Conflict resolution strategy
   - Debouncing for performance

### Phase 7: Polish
9. **User experience**:
   - Keyboard shortcuts (vim-like navigation)
   - Help menu (? key)
   - Status bar with current mode
   - Error handling and validation
   - Search functionality (/ key)

10. **Persistence**:
    - Save/load SQL files
    - Export diagram as SQL
    - Project file format for layout preservation
    - Auto-save functionality

## Technical Considerations

### Performance
- Efficient rendering for large schemas (100+ tables)
- Lazy rendering for off-screen entities
- Caching of layout calculations

### Testing Strategy
- Unit tests for SQL parser
- Integration tests for bidirectional sync
- Property-based testing for layout algorithm
- Snapshot tests for rendering output

### Key Algorithms
1. **Crossing Minimization**:
   - Layer assignment (Longest Path)
   - Crossing reduction (Barycenter method)
   - Coordinate assignment (Brandes & Köpf)

2. **Force-Directed Layout**:
   - Spring forces between connected nodes
   - Repulsion forces between all nodes
   - Simulated annealing for convergence

### File Structure
```
nerd-core/
├── src/
│   ├── main.rs           # Application entry point
│   ├── app.rs            # Application state and event handling
│   ├── parser/           # SQL parsing module
│   ├── layout/           # Graph layout algorithms
│   ├── render/           # TUI rendering
│   ├── models/           # Data structures
│   ├── sync/             # Bidirectional synchronization
│   └── ui/               # UI components
├── tests/
│   ├── parser_tests.rs
│   ├── layout_tests.rs
│   └── integration_tests.rs
└── examples/
    └── sample_schemas/   # Example SQL schemas
```

## Implementation Order
1. Basic project setup and dependencies
2. SQL parser for CREATE TABLE statements
3. Core data models (Entity, Relationship)
4. Simple TUI with entity rendering
5. Basic layout algorithm
6. Relationship line rendering
7. Keyboard navigation
8. SQL to diagram conversion
9. Diagram to SQL generation
10. Advanced layout optimization
11. Interactive editing features
12. Polish and error handling

## Success Criteria
- [ ] Can parse standard SQL CREATE TABLE statements
- [ ] Renders entities with clear visual hierarchy
- [ ] Relationship lines don't cross (or minimal crossings)
- [ ] Real-time bidirectional synchronization works smoothly
- [ ] Keyboard navigation is intuitive and responsive
- [ ] Can handle schemas with 50+ tables efficiently
- [ ] Generated SQL is valid and preserves all constraints
- [ ] Application is stable with no crashes on invalid input