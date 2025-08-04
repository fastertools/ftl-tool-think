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

## Installation

```bash
# Clone the repository
git clone https://github.com/fastertools/ftl-tool-think.git
cd ftl-tool-think

# Build the tool
make build

# Run with FTL
ftl serve
```

## Usage

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

## Development

### Prerequisites
- Rust 1.86+
- FTL SDK
- Make

### Building
```bash
make build
```

### Testing
```bash
make test
```

## Architecture

The tool is built on:
- **[MindKit](https://github.com/DevOpsDali/mindkit)**: Generic sequential thinking library
- **FTL SDK**: For MCP server integration
- **WebAssembly**: Compiled to WASM for secure, sandboxed execution

## Inspiration

This tool was inspired by [@modelcontextprotocol/server-sequential-thinking](https://www.npmjs.com/package/@modelcontextprotocol/server-sequential-thinking), extending the concept into a Rust-based FTL tool.

## License

Apache-2.0