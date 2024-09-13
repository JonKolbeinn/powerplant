# Tests for Powerplant POW Server

   This directory contains tests for the Powerplant POW Server.

   ## Python Test Client

   The `test_client.py` script is a WebSocket client that tests the POW server functionality.

   To run the test client:

   1. Ensure the Powerplant server is running.
   2. Install the required Python library:
      ```
      pip install websocket-client
      ```
   3. Run the test client:
      ```
      python test_client.py
      ```

   This will connect to the server, send a POW request, and display the response.
   