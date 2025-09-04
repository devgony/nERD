# nERD Examples

This directory contains examples demonstrating how to draw ASCII entity-relationship diagrams similar to those used in the nERD library.

## Running Examples

To run any example:

```bash
cargo run --example <example_name>
```

## Available Examples

### 1. Simple Tables (`simple_tables.rs`)
Demonstrates basic ASCII table drawings and simple relationships.

```bash
cargo run --example simple_tables
```

Shows:
- Simple two-table diagrams
- Tables with foreign key relationships
- Complex database schemas with multiple relationships

### 2. Draw Entities (`draw_entities.rs`)
Uses the actual `into_entities` function to parse SQL DDL and draw tables.

```bash
cargo run --example draw_entities
```

Shows:
- Parsing SQL CREATE TABLE statements
- Converting schemas to entities with calculated positions
- Drawing entities on an ASCII grid

### 3. Foreign Key Lines (`foreign_key_lines.rs`)
Demonstrates various ways to draw relationship lines between tables.

```bash
cargo run --example foreign_key_lines
```

Shows:
- One-to-many relationships
- Many-to-many relationships with junction tables
- Self-referencing relationships
- Multiple foreign keys from one table
- Box drawing characters reference

### 4. Grid Layout (`grid_layout.rs`)
Explains the grid layout algorithm used to position tables.

```bash
cargo run --example grid_layout
```

Shows:
- Layout calculation parameters
- How tables are positioned in a grid
- Handling variable table heights
- Visual grid representation

## Box Drawing Characters Reference

The examples use Unicode box drawing characters:

```
┌─────────┐  Corners: ┌ ┐ └ ┘
│ Content │  Lines: ─ │
├─────────┤  T-joints: ├ ┤ ┬ ┴
└─────────┘  Crosses: ┼
```

For foreign key relationships:
- `→` Right arrow
- `←` Left arrow
- `├` Left junction (foreign key from)
- `┤` Right junction (foreign key to)

## Creating Your Own Diagrams

To create your own ASCII diagrams:

1. Define your table structure with columns
2. Calculate positions using the grid layout algorithm
3. Draw boxes using the Unicode box characters
4. Add relationship lines between related tables
5. Use arrows or junction characters to indicate foreign keys

The grid layout algorithm from `engine.rs`:
- Layout size: 100 units
- Margin: 1 unit from edges
- Entity width: 12 characters
- Gap between entities: 3 characters
- Entities per row: floor((layout_size - 2*margin) / (entity_width + gap))