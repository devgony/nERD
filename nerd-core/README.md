# nERD - Terminal Entity Relationship Diagram Tool

A powerful, interactive Terminal User Interface (TUI) application for creating, editing, and visualizing Entity-Relationship Diagrams with bidirectional SQL synchronization.

## Features

### 🎯 Core Capabilities
- **Interactive Diagram Editing**: Create and manipulate ER diagrams directly in your terminal
- **SQL Parser Integration**: Import SQL CREATE TABLE statements and visualize them instantly  
- **Bidirectional Synchronization**: Edit diagrams and generate SQL, or modify SQL and sync back to diagrams
- **Intelligent Layout Engine**: Force-directed graph layout with automatic entity positioning
- **Real-time Visualization**: Live updates as you modify entities and relationships

### 🚀 User Experience
- **Vim-inspired Keybindings**: Intuitive keyboard navigation and commands
- **Multi-mode Interface**: Switch between diagram view, SQL editor, and entity creation
- **Visual Relationship Indicators**: Primary keys (🗝), foreign keys (🔗), and nullable fields (?)
- **Entity Selection & Movement**: Navigate and precisely position entities with arrow keys
- **Comprehensive Help System**: Built-in help screen with all keybindings

### ⚡ Technical Highlights
- **High Performance**: Efficient rendering for schemas with 100+ tables
- **Multiple SQL Dialects**: Support for PostgreSQL, MySQL, SQLite via configurable parsers
- **Schema Validation**: Real-time validation with detailed error reporting
- **Position Preservation**: Maintains entity positions during SQL synchronization
- **Robust Architecture**: Modular design with separation of concerns

## Installation

```bash
# Clone the repository
git clone <repository-url>
cd nerd-core

# Build the application
cargo build --release

# Run the application
cargo run
```

## Quick Start

1. **Launch the application**: `cargo run`
   - The app starts with a pre-loaded e-commerce schema for immediate exploration!
2. **Explore the sample**: Use `Tab`/`Shift+Tab` to select entities, arrow keys to move them
3. **View the SQL**: Press `s` to see the SQL that generated the current diagram
4. **Try editing**: Modify the SQL in editor mode, then `Ctrl+S` to sync changes
5. **Generate SQL**: Press `g` to generate SQL from your current diagram layout
6. **Create entities**: Press `n` to create a new entity
7. **Get help**: Press `?` for complete keybinding reference

## Keybindings

### Navigation & Modes
| Key | Action |
|-----|--------|
| `q` | Quit application |
| `s` | Switch to SQL editor |
| `n` | Create new entity |
| `i` | Import SQL (or switch to editor) |
| `g` | Generate SQL from diagram |
| `r` | Refresh/re-layout diagram |
| `v` | Validate schema |
| `?` | Show help screen |
| `Esc` | Return to diagram view |

### Entity Management
| Key | Action |
|-----|--------|
| `Tab` | Select next entity |
| `Shift+Tab` | Select previous entity |
| `↑↓←→` | Move selected entity |
| `Ctrl+D` / `Del` | Delete selected entity |

### SQL Editor
| Key | Action |
|-----|--------|
| `Ctrl+S` | Sync SQL changes to diagram |
| `Type` | Enter/edit SQL statements |
| `g` | Generate SQL from current diagram |

## Pre-loaded Sample Schema

When you first launch nERD, it comes with a sample e-commerce schema already loaded:

- **users** - Customer accounts with login credentials  
- **categories** - Product categorization
- **products** - Items for sale with foreign key to categories
- **orders** - Customer orders with foreign key to users
- **order_items** - Order details with foreign keys to orders and products

This demonstrates the foreign key relationships:
- `products.category_id` → `categories.id`
- `orders.user_id` → `users.id`  
- `order_items.order_id` → `orders.id`
- `order_items.product_id` → `products.id`

You can immediately start exploring by pressing `Tab` to select entities and using arrow keys to rearrange them!

## Example Usage

### Creating a Simple Blog Schema

1. **Start with SQL**: Press `s` to enter SQL editor
2. **Enter schema**:
   ```sql
   CREATE TABLE authors (
       id INT PRIMARY KEY,
       name VARCHAR(100) NOT NULL,
       email VARCHAR(255) NOT NULL
   );
   
   CREATE TABLE posts (
       id INT PRIMARY KEY,
       title VARCHAR(255) NOT NULL,
       author_id INT NOT NULL,
       content TEXT
   );
   ```
3. **Import**: Press `Ctrl+S` to parse and visualize
4. **Adjust layout**: Use arrow keys to position entities
5. **Export**: Press `g` to generate updated SQL

### Interactive Entity Creation

1. **Create entity**: Press `n` and type "products"
2. **Position**: Use arrow keys to move the entity
3. **Generate SQL**: Press `g` to see the generated CREATE TABLE statement
4. **Refine**: Switch to SQL editor (`s`) to add more columns

## Project Structure

```
nerd-core/
├── src/
│   ├── main.rs           # Application entry point
│   ├── app.rs            # Application state and event handling
│   ├── parser/           # SQL parsing with sqlparser integration
│   ├── layout/           # Force-directed graph layout algorithms
│   ├── render/           # TUI rendering with ratatui
│   ├── models/           # Core data structures
│   └── sync/             # Bidirectional synchronization logic
├── examples/
│   └── sample_schemas/   # Example SQL schemas
└── tests/                # Test suite
```

## Architecture

### Core Components

- **Models**: Entity, Relationship, Schema definitions with serialization support
- **Parser**: SQL parsing using the `sqlparser` crate with dialect support  
- **Layout Engine**: Force-directed positioning with collision detection and constraints
- **Renderer**: TUI rendering with `ratatui` for cross-platform terminal support
- **Sync Engine**: Bidirectional synchronization with change detection and merging
- **App Controller**: Event handling and state management

### Key Algorithms

1. **Force-Directed Layout**: 
   - Spring forces between connected entities (relationships)
   - Repulsion forces between all entities to prevent overlap
   - Simulated annealing for stable convergence
   - Boundary constraints to keep entities within view

2. **Schema Synchronization**:
   - Intelligent change detection comparing entity structures
   - Position-preserving merge that maintains layout state  
   - Conflict resolution for concurrent modifications
   - Validation with detailed error reporting

## Development

### Prerequisites
- Rust 1.70+ 
- Cargo

### Building
```bash
cargo build
```

### Testing
```bash
cargo test
```

### Running Examples
```bash
# Basic usage
cargo run

# With sample schema
cargo run
# Then press 's' and paste contents from examples/sample_schemas/
```

## Roadmap

- [ ] Foreign key relationship detection and visualization
- [ ] Export to various formats (PNG, SVG, PDF)
- [ ] Schema comparison and diff visualization
- [ ] Plugin system for custom SQL dialects
- [ ] Collaborative editing with conflict resolution
- [ ] Database reverse engineering from live connections

## Contributing

Contributions are welcome! Please feel free to submit issues, feature requests, or pull requests.

## License

[Add your license here]

## Acknowledgments

- Built with [ratatui](https://github.com/ratatui-org/ratatui) for TUI framework
- SQL parsing powered by [sqlparser-rs](https://github.com/sqlparser-rs/sqlparser-rs)  
- Graph algorithms inspired by [petgraph](https://github.com/petgraph/petgraph)
- Force-directed layout based on Fruchterman-Reingold algorithm