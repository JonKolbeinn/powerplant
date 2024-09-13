import websocket
import json
import time

def on_message(ws, message):
    print(f"Received: {message}")
    try:
        response = json.loads(message)
        if 'Err' in response:
            print(f"Error from server: {response['Err']}")
        elif 'Ok' in response:
            result = response['Ok']
            print(f"POW completed with difficulty: {result['pow']}")
            event = result['event']
            nonce_tag = next((tag for tag in event['tags'] if tag[0] == 'nonce'), None)
            if nonce_tag:
                print(f"Nonce: {nonce_tag[1]}")
            else:
                print("Nonce tag not found in the response")
            print(f"Event details:")
            print(f"  Created at: {event['created_at']}")
            print(f"  Kind: {event['kind']}")
            print(f"  Content: {event['content']}")
            print(f"  Pubkey: {event['pubkey']}")
        else:
            print("Unexpected response format")
    except json.JSONDecodeError:
        print("Received message is not valid JSON")
    except KeyError as e:
        print(f"Expected key not found in response: {e}")

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
