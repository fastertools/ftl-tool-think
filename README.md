# ftl-tool-think

[![Release](https://github.com/fastertools/ftl-tool-think/workflows/Release/badge.svg)](https://github.com/fastertools/ftl-tool-think/actions)
[![Rust Version](https://img.shields.io/badge/rust-1.86%2B-orange)](https://forge.rust-lang.org/releases.html)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue)](LICENSE)
[![GHCR](https://img.shields.io/badge/ghcr.io-ftl--tool--think-blue)](https://github.com/fastertools/ftl-tool-think/pkgs/container/ftl-tool-think)

An **[FTL](https://github.com/fastertools/ftl-cli)** tool for structured reasoning and dynamic problem-solving, powered by [MindKit](https://github.com/DevOpsDali/mindkit), for AI Agents.

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
- Rust 1.89+ (for local development)

## Usage

### Running Locally

Clone and run the tool locally for development or testing:

```bash
# Clone the repository
gh repo clone fastertools/ftl-tool-think
cd ftl-tool-think

# Start the FTL development server
ftl up
```

The tool will be available in your local environment for use in your applications.

## Tool Interface

The `structured_reasoning` function accepts a `ThinkInput` structure with the following fields:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `thought` | `String` | Yes | The current reasoning step |
| `next_thought_needed` | `bool` | Yes | Whether more reasoning is required |
| `thought_number` | `u32` | Yes | Current reasoning step number (must be > 0) |
| `total_thoughts` | `u32` | Yes | Estimated total reasoning steps (must be > 0) |
| `thought_type` | `Option<String>` | No | Reasoning type: analytical, critical, synthesis, or validation |
| `is_revision` | `Option<bool>` | No | Whether this revises a previous reasoning step |
| `revises_thought` | `Option<u32>` | No | Which reasoning step number is being revised |
| `branch_from_thought` | `Option<u32>` | No | Branching point for alternate paths |
| `branch_id` | `Option<String>` | No | Identifier for the branch |
| `needs_more_thoughts` | `Option<bool>` | No | Indicates expansion beyond initial reasoning estimate |
| `custom_lens` | `Option<String>` | No | Domain-specific analytical lens |
| `confidence` | `Option<f32>` | No | Confidence level (0.0-1.0) |

## Examples

### Basic Structured Reasoning
```json
{
  "thought": "Let me break down this authentication problem step by step",
  "next_thought_needed": true,
  "thought_number": 1,
  "total_thoughts": 3
}
```

### Critical Analysis with Low Confidence
```json
{
  "thought": "This approach might have security implications I haven't considered",
  "next_thought_needed": true,
  "thought_number": 2,
  "total_thoughts": 4,
  "thought_type": "critical",
  "confidence": 0.6
}
```

### Revising a Previous Thought
```json
{
  "thought": "Actually, I need to reconsider my assumption about the database schema",
  "next_thought_needed": true,
  "thought_number": 3,
  "total_thoughts": 5,
  "is_revision": true,
  "revises_thought": 1
}
```

## Full Documentation

This tool wraps the [MindKit](https://github.com/DevOpsDali/mindkit) library. For comprehensive documentation, advanced features, and detailed examples, see:

- **[MindKit Documentation](https://docs.rs/mindkit)** - Complete API reference and usage guide
- **[MindKit GitHub](https://github.com/DevOpsDali/mindkit)** - Source code and latest updates

The interface documented above matches MindKit's `ThinkInput` structure. For information about reasoning types, visual indicators, advanced branching, custom lenses, and implementation patterns, refer to the upstream MindKit documentation.

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

## Acknowledgements

This tool is built on **[MindKit](https://github.com/DevOpsDali/mindkit)**, a framework-agnostic Rust library for structured reasoning and analytical thinking patterns. MindKit provides the core reasoning engine, while this tool provides FTL integration.

The MindKit library was inspired by [@modelcontextprotocol/server-sequential-thinking](https://www.npmjs.com/package/@modelcontextprotocol/server-sequential-thinking), extending the concept into a comprehensive structured reasoning framework.

## License

Apache-2.0