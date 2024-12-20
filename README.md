# Billy: Your Bitcoin Node Assistant 🧚‍♂️

Billy is a friendly command-line interface that helps you interact with your Bitcoin node using natural language. Instead of remembering complex RPC commands, simply ask Billy questions in plain English, and he'll handle the technical details for you.

## Features

- 🗣️ Natural language interface to your Bitcoin node
- 🔍 Automatic conversion of questions to Bitcoin RPC commands
- 📊 Smart summarization of complex JSON responses
- 🔒 Secure connection to your local Bitcoin node
- 🤖 Powered by Ollama for natural language processing

## Prerequisites

- Rust (latest stable version)
- A running Bitcoin Core node
- Ollama server (default: running on localhost:11434)

## Installation

1. Clone the repository:

2. Create a `.env` file in the project root with your Bitcoin node credentials:

```
BITCOIN_RPC_USER=your_rpc_user
BITCOIN_RPC_PASS=your_rpc_password
OLLAMA_SERVER=http://localhost:11434
OLLAMA_MODEL=qwen2.5
```

3. Build and run Billy:

bash
cargo build --release
cargo run --release

```
Bitcoin Node Chat Interface
Type 'exit' to quit
Ask questions about your node...
> What's the current block height?
Executing command: getblockcount
Current block height: 824892
> How many peers am I connected to?
Executing command: getconnectioncount
Response: 8
```
## Usage

Once running, Billy provides an interactive prompt where you can ask questions about your node:


## Example Questions

You can ask Billy questions like:
- "What's the current block height?"
- "How many peers am I connected to?"
- "Show me information about the latest block"
- "What's my wallet balance?"
- "Show me my node's network info"

## Configuration

Billy uses the following default settings:
- Bitcoin node connection: `http://127.0.0.1:8332`
- Ollama server: `http://localhost:11434`
- Language model: `qwen2.5` (configurable via OLLAMA_MODEL env var)

These can be configured through environment variables or the `.env` file.


## How it Works

Billy uses Ollama's language model to:
1. Convert your natural language questions into Bitcoin RPC commands
2. Translate complex JSON responses into human-readable summaries

The application references the following code for core functionality:
- Main interface: `src/main.rs`
- Ollama integration: `src/ollama.rs`

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

GNU Affero General Public License v3.0

## Support

If you encounter any issues or have questions, please open an issue on the GitHub repository.