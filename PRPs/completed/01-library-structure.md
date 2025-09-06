# PRP: Convert to Library Structure

## Objective
Convert the current binary crate to a library crate with proper module organization and public API exposure.

## Context
The project currently has a main.rs binary file with "Hello World". It needs to become a library that exposes market data generation functionality. The Cargo.toml already has feature flags for future API emulation capabilities.

## Success Criteria
- Library compiles with `cargo build --lib`
- Public API is accessible from external crates
- Examples can import and use the library
- Documentation builds with `cargo doc`

## Implementation Tasks
1. Create src/lib.rs as the library entry point
2. Move main.rs to examples/basic.rs
3. Set up module structure with proper visibility
4. Define the root module exports
5. Update Cargo.toml if needed for library metadata
6. Add basic library documentation comments

## Module Structure to Create
- lib.rs (root module, public exports)
- mod.rs files for submodules as needed
- Public API surface clearly defined

## Validation Gates
```bash
# Build as library
cargo build --lib

# Check documentation
cargo doc --no-deps

# Run example
cargo run --example basic

# Check that library can be imported
cargo check --lib
```

## References
- Rust book on library crates: https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
- Cargo book on project layout: https://doc.rust-lang.org/cargo/guide/project-layout.html

## Dependencies
None - this is the foundational structure

## Notes
- Use `pub` keyword for public API items
- Use `pub(crate)` for internal visibility
- Document public items with /// comments
- Consider re-exporting commonly used types at root

## Confidence Score: 9/10
Simple structural change with clear Rust conventions to follow.