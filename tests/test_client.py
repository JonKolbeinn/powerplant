import websocket
import json
import time

def on_message(ws, message):
    print(f"Received: {message}")
    response = json.loads(message)
    print(f"POW completed with difficulty: {response['pow']}")
    print(f"Nonce: {response['event']['tags'][-1][1]}")

def on_error(ws, error):
    print(f"Error: {error}")

def on_close(ws, close_status_code, close_msg):
    print("Connection closed")

def on_open(ws):
    print("Connection opened")
    pow_request = {
        "event": {
            "created_at": int(time.time()),
            "kind": 1,
            "tags": [],
            "content": "Hello, World!",
            "pubkey": "test_pubkey"
        },
        "target_pow": 20
    }
    ws.send(json.dumps(pow_request))
    print("POW request sent")

if __name__ == "__main__":
    websocket.enableTrace(True)
    ws = websocket.WebSocketApp("ws://localhost:8080",
                                on_open=on_open,
                                on_message=on_message,
                                on_error=on_error,
                                on_close=on_close)

    ws.run_forever()
