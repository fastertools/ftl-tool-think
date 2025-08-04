# ftl-tool-think

An FTL tool for sequential thinking and structured reasoning, powered by [MindKit](https://github.com/DevOpsDali/mindkit).

## Overview

The `think` tool provides AI systems with a structured approach to problem-solving through sequential thinking. It allows for dynamic reasoning that can branch, revise, and evolve as understanding deepens.

## Features

- **Sequential Thinking**: Break down complex problems into manageable thought steps
- **Thought Types**: Support for analytical, critical, synthesis, and validation thinking modes
- **Dynamic Revision**: Ability to revise previous thoughts as new insights emerge
- **Branching Logic**: Explore alternative reasoning paths from any thought
- **Confidence Tracking**: Express uncertainty with confidence levels (0.0-1.0)
- **Custom Lenses**: Apply domain-specific analysis (e.g., "security", "performance", "Rust best practices")

## Prerequisites

- [FTL CLI](https://github.com/TBD54566975/ftl) installed
- Docker (for running with FTL)
- Rust 1.86+ (for local development)

## Usage

### Option 1: Run Locally

Clone and run the tool locally for development or testing:

```bash
# Clone the repository
git clone https://github.com/fastertools/ftl-tool-think.git
cd ftl-tool-think

# Start the FTL development server
ftl up
```

The tool will be available in your local FTL environment for use in your applications.

### Option 2: Use as a Remote Dependency

Add this tool to your FTL project as a remote source from GitHub Container Registry (GHCR):

1. In your FTL project, add the tool:
```bash
ftl add ghcr.io/fastertools/ftl-tool-think:latest
```

2. Or manually add to your `ftl.toml`:
```toml
[[tools]]
name = "think"
source = "ghcr.io/fastertools/ftl-tool-think:latest"
```

3. Use the tool in your FTL application:
```rust
use ftl_think::cogitate;

// Your code using the thinking tool
```

## Tool Interface

The `cogitate` function accepts the following parameters:

- `thought`: Your current thinking step
- `nextThoughtNeeded`: Whether more thinking is required
- `thoughtNumber`: Current position in the thought sequence
- `totalThoughts`: Estimated total thoughts needed (adjustable)
- `thoughtType` (optional): Type of reasoning - "analytical", "critical", "synthesis", or "validation"
- `confidence` (optional): Confidence level (0.0-1.0)
- `customLens` (optional): Domain-specific focus like "security" or "performance"
- `isRevision` (optional): Whether this revises a previous thought
- `revisesThought` (optional): Which thought number is being revised
- `branchFromThought` (optional): Starting point for branching logic
- `branchId` (optional): Identifier for the current branch

## Examples

### Basic Sequential Thinking
```json
{
  "thought": "Let me break down this authentication problem step by step",
  "nextThoughtNeeded": true,
  "thoughtNumber": 1,
  "totalThoughts": 3
}
```

### Critical Analysis with Low Confidence
```json
{
  "thought": "This approach might have security implications I haven't considered",
  "nextThoughtNeeded": true,
  "thoughtNumber": 2,
  "totalThoughts": 4,
  "thoughtType": "critical",
  "confidence": 0.6
}
```

### Revising a Previous Thought
```json
{
  "thought": "Actually, I need to reconsider my assumption about the database schema",
  "nextThoughtNeeded": true,
  "thoughtNumber": 3,
  "totalThoughts": 5,
  "isRevision": true,
  "revisesThought": 1
}
```

## Publishing to GHCR

To publish this tool to GitHub Container Registry for others to use:

1. Ensure you're authenticated with GitHub:
```bash
# Login to GitHub Container Registry
echo $GITHUB_TOKEN | docker login ghcr.io -u YOUR_GITHUB_USERNAME --password-stdin
```

2. Build the tool:
```bash
ftl build
```

3. Tag and push to GHCR:
```bash
# Tag the image
docker tag ftl-tool-think:latest ghcr.io/YOUR_GITHUB_USERNAME/ftl-tool-think:latest

# Push to GHCR
docker push ghcr.io/YOUR_GITHUB_USERNAME/ftl-tool-think:latest
```

4. Make the package public (optional):
   - Go to your GitHub profile → Packages
   - Find the `ftl-tool-think` package
   - Click on "Package settings"
   - Change visibility to "Public"

## Development

### Building from Source

```bash
# Clone the repository
git clone https://github.com/fastertools/ftl-tool-think.git
cd ftl-tool-think

# Build the tool
cargo build --release

# Run tests
cargo test
```

### Contributing

Please feel free to get in touch if you're interested in contributing.

## Architecture

The tool is built on:
- **[MindKit](https://github.com/DevOpsDali/mindkit)**: Generic sequential thinking library
- **FTL SDK**: For MCP server integration
- **WebAssembly**: Compiled to WASM for secure, sandboxed execution

## Inspiration

This tool was inspired by [@modelcontextprotocol/server-sequential-thinking](https://www.npmjs.com/package/@modelcontextprotocol/server-sequential-thinking), extending the concept into a Rust-based FTL tool.

## License

Apache-2.0