# Build/Test/Lint Commands for lamb Scheme Interpreter

## Essential Commands
- Build: `cargo build`
- Run: `cargo run`
- Test all: `cargo test`
- Test specific: `cargo test test_parse_number` 
- Test with pattern: `cargo test parse`
- Lint: `cargo clippy`
- Format: `cargo fmt`

## Code Style Guidelines

### Naming Conventions
- Functions: snake_case (e.g., `eval_expression`, `parse_number`)
- Types/Enums: PascalCase (e.g., `BuiltinOp`, `Atom`, `Expr`)
- Constants: SCREAMING_SNAKE_CASE (e.g., `PROGRAM_NAME`)

### Imports Organization
- Group imports by crate
- Use nested paths for related imports from same crate
- Standard library imports use qualified paths (std::io)
- External crates first, then internal modules

### Error Handling
- Use `anyhow` crate for error handling
- Error creation with `anyhow::anyhow!()` macro
- Error propagation with `?` operator
- Include meaningful debug context in errors (`{val:#?}`)

### Testing
- Use `#[test]` for standard tests
- Use `rstest` with `#[case]` for parameterized tests
- Test module: `mod tests` within implementation files
- Use descriptive test function names