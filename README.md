# SolanaRust

A decentralized social media application built on the Solana blockchain using Rust.

## Features

- **User Management**: Create and update user profiles with names and bios
- **Content Publishing**: Post and share content on the blockchain
- **Social Interactions**: Follow other users and like content
- **Tokenization**: Built-in token system for rewarding content creation and interaction
- **Solana Integration**: Leverages Solana's high performance and low fees

## Technology Stack

- **Rust**: Core programming language
- **Solana SDK**: For blockchain interaction
- **Borsh**: For efficient serialization/deserialization
- **Tokio**: Asynchronous runtime
- **Clap**: Command-line interface

## Project Structure

```
├── src/
│   ├── error.rs      # Error handling
│   ├── lib.rs        # Library entry point
│   ├── main.rs       # Client entry point
│   ├── models.rs     # Data models (User, Content)
│   ├── program.rs    # Program instructions and processing
│   ├── state.rs      # On-chain state management
│   ├── token.rs      # Token functionality
│   └── utils.rs      # Utility functions
├── tests/            # Test code
└── docs/             # Documentation
```

## Architecture

The system follows a client-on-chain program architecture:

- **Client**: Command-line interface for interacting with the Solana network
- **On-chain Program**: Smart contract deployed on Solana that handles all business logic
- **On-chain Storage**: Uses Solana accounts to store user and content data

### Data Models

- **User**: Stores user profile information including name, bio, and follower counts
- **Content**: Represents posts with text content, author information, and engagement metrics
- **Token**: Implements a custom token for the platform with reward mechanisms

## Prerequisites

- Rust and Cargo (latest stable version)
- Solana CLI tools
- Solana account with SOL for transactions

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/SolanaRust.git
   cd SolanaRust
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

### Creating a User

```bash
solana_rust create-user --name "YourUsername" --url https://api.devnet.solana.com
```

### Posting Content

```bash
solana_rust post-content --content "Hello, Solana world!" --url https://api.devnet.solana.com
```

### Getting User Information

```bash
solana_rust get-user --address "YourSolanaAddress" --url https://api.devnet.solana.com
```

## Token System

The platform includes a native token system that:

- Rewards users for creating quality content
- Enables tipping and transactions between users
- Provides governance capabilities for platform decisions

## Development

### Setting Up a Local Development Environment

1. Install Solana tools:
   ```bash
   sh -c "$(curl -sSfL https://release.solana.com/v1.17.0/install)"
   ```

2. Start a local Solana test validator:
   ```bash
   solana-test-validator
   ```

3. Build and deploy the program:
   ```bash
   cargo build-bpf
   solana program deploy target/deploy/solana_rust.so
   ```

## Testing

Run the test suite with:

```bash
cargo test
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
