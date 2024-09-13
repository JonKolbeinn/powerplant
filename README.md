# Powerplant

Powerplant is a high-performance Proof of Work (PoW) server implemented in Rust, designed to work with Nostr events. It provides a WebSocket interface for clients to request PoW calculations and receive the results.

## Features

- WebSocket server for handling PoW requests
- Customizable PoW difficulty
- Efficient PoW calculation algorithm
- Nostr event compatibility
- Configurable server settings
- Python test client included

## Prerequisites

- Rust (latest stable version)
- Python 3.7+ (for running the test client)

## Installation

1. Clone the repository:
   ```
   git clone https://github.com/your-username/powerplant.git
   cd powerplant
   ```

2. Build the Rust server:
   ```
   cargo build --release
   ```

3. Install Python dependencies for the test client:
   ```
   pip install websocket-client
   ```

## Configuration

Server settings can be configured in the `config/default.toml` file. You can adjust the following parameters:

- `server.host`: The host address to bind the server to
- `server.port`: The port number for the WebSocket server
- `server.max_connections`: Maximum number of simultaneous connections
- `pow.default_difficulty`: Default PoW difficulty if not specified in the request
- `pow.max_difficulty`: Maximum allowed PoW difficulty

## Usage

1. Start the Powerplant server:
   ```
   cargo run --release
   ```

2. Use the Python test client to send PoW requests:
   ```
   python test_client.py
   ```

## API

The server accepts WebSocket connections and expects JSON-formatted messages for PoW requests. The request format is as follows:

```json
{
  "event": {
    "created_at": 1234567890,
    "kind": 1,
    "tags": [],
    "content": "Hello, World!",
    "pubkey": "public_key_here"
  },
  "target_pow": 20
}
```

The server will respond with a JSON message containing the updated event (including the calculated nonce) and the achieved PoW difficulty:

```json
{
  "event": {
    "created_at": 1234567890,
    "kind": 1,
    "tags": [["nonce", "calculated_nonce_here"]],
    "content": "Hello, World!",
    "pubkey": "public_key_here"
  },
  "pow": 21
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
