# nERD

Interactive terminal entity-relationship diagrams built with Rust and Ratatui.

## Run

From the repository root, with Rust and Cargo installed:

```bash
cargo run --locked
```

The application opens an interactive diagram with five e-commerce tables and four foreign-key relationships. The startup SQL is embedded from `examples/sample_schemas/ecommerce.sql`; no example flag or external database is required.

There is one Cargo package, `nerd-core`, at the repository root. The old root prototype and print-only Rust examples have been removed. The library remains available as `nerd_core`.

## Controls

### Diagram

| Key | Action |
| --- | --- |
| `Tab` / `Shift+Tab` | Select the next / previous entity |
| Arrow keys | Move the selected entity |
| `Ctrl+D` | Delete the selected entity and its relationships |
| `s` | Open the SQL editor |
| `i` | Import the current SQL and recalculate layout |
| `g` | Generate SQL from the diagram and open it in the editor |
| `n` | Create a table with an `id` primary key |
| `r` | Recalculate layout |
| `?` | Show help |
| `q` | Quit |

### SQL editor

The editor opens in normal mode. Press `i` to insert text. In normal mode, `h/j/k/l` or arrow keys move the cursor, `w/b` move by word, `0/$` move to line boundaries, `x` deletes a character, and `d` deletes the current line.

- `Ctrl+S`: parse and apply SQL changes, then return to the diagram on success.
- `Esc`: leave insert mode; from normal mode, return to the diagram.
- `Ctrl+Q`: quit from normal mode.

In help, `Esc` or `q` returns to the diagram. In the table creator, type a name and press `Enter`, or cancel with `Esc`.

## SQL samples

`examples/sample_schemas/` contains:

- `ecommerce.sql`: the built-in startup sample.
- `blog.sql`: authors, posts, tags, and comments.
- `library_with_fk.sql`: books, authors, publishers, members, and loans.

Each sample declares its foreign keys in `CREATE TABLE` statements. To try another sample, replace the current SQL in the editor and press `Ctrl+S`. There is no command-line SQL file loader.

## Current limitations

- SQL import handles `CREATE TABLE`, not `ALTER TABLE`. Generated foreign keys use `ALTER TABLE`, so exporting and re-importing SQL is not yet lossless.
- The editor's cursor operations are not Unicode-safe; non-ASCII editing can panic.
- SQL parse errors are not displayed in the UI. Invalid SQL leaves the previous diagram unchanged.
- Foreign-key routing and overlapping table placement can require manual adjustment.

## Development

```bash
cargo fmt --check
cargo test --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo build --locked --release
```

- `src/main.rs`: terminal lifecycle and screen composition, using the library.
- `src/app.rs`: interactive state and key handling.
- `src/models/`, `src/parser/`, `src/layout/`, `src/render/`, `src/sync/`: schema data, SQL parsing, layout, rendering, and SQL synchronization.
- `tests/`: foreign-key parsing regressions and rendered-diagram snapshots.

## License

Apache-2.0. See `LICENSE`.
