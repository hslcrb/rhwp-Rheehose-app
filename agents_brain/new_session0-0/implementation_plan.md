# Goal Description
Install dependencies, build the project, and run the web editor as requested by the user.

## Proposed Changes

### [Native/Rust]
Build the Rust core and run tests to ensure the environment is correctly set up.
- Run `cargo build`
- Run `cargo test`

### [WASM]
Since Docker is missing, I will attempt to install `wasm-pack` locally to build the WASM bindings needed for the web editor.
- Install `wasm-pack` via `cargo install wasm-pack` (if possible) or check for alternatives.
- Run `wasm-pack build --target web --out-dir pkg` if `wasm-pack` is available.

### [Web Editor / rhwp-studio]
Install Node.js dependencies and run the Vite development server.
- `cd rhwp-studio`
- `npm install`
- `npm run dev` (or `npx vite`)

## Verification Plan

### Automated Tests
- `cargo test`: Verify that the core logic is working correctly.
- `cd rhwp-studio && npm test`: If there are any frontend tests.

### Manual Verification
- Access the web editor at `http://localhost:7700` and verify it loads correctly.
- Check if the WASM module is correctly loaded and can render a sample HWP file.
