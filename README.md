# ftl-tool-think

[![Release](https://github.com/fastertools/ftl-tool-think/workflows/Release/badge.svg)](https://github.com/fastertools/ftl-tool-think/actions)
[![Rust Version](https://img.shields.io/badge/rust-1.86%2B-orange)](https://forge.rust-lang.org/releases.html)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue)](LICENSE)
[![GHCR](https://img.shields.io/badge/ghcr.io-ftl--tool--think-blue)](https://github.com/fastertools/ftl-tool-think/pkgs/container/ftl-tool-think)

An **[FTL](https://github.com/fastertools/ftl-cli)** tool for structured reasoning and dynamic problem-solving, powered by [MindKit](https://github.com/DevOpsDali/mindkit).

## Overview

The `think` tool provides AI systems with a structured reasoning framework for complex problem-solving. It allows for dynamic reasoning that can branch, revise, and evolve as understanding deepens.

## Features

- **Structured Reasoning**: Break down complex problems into manageable analytical steps
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
ftl add tools add ghcr.io/fastertools/ftl-tool-think:latest
```

2. Or manually add to your `ftl.toml`:
```toml
[[tools]]
name = "think"
source = "ghcr.io/fastertools/ftl-tool-think:latest"
```

3. Use the tool in your FTL application:
```rust
use ftl_think::structured_reasoning;

// Your code using the structured reasoning tool
```

## Tool Interface

The `structured_reasoning` function accepts the following parameters:

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

### Basic Structured Reasoning
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

## Development

### Building from Source

```bash
# Clone the repository
git clone https://github.com/fastertools/ftl-tool-think.git
cd ftl-tool-think

# Build the tool
ftl build

# Start the tool
ftl up
```

### Contributing

Please feel free to get in touch if you're interested in contributing.

## Inspiration / Acknowledgements

The tool is built on **[MindKit](https://github.com/DevOpsDali/mindkit)** mcp agnostic sequential thinking library

This tool was inspired by [@modelcontextprotocol/server-sequential-thinking](https://www.npmjs.com/package/@modelcontextprotocol/server-sequential-thinking), extending the concept into a structured reasoning framework built as a Rust-based FTL tool.

## License

Apache-2.0