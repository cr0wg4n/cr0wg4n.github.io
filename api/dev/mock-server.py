#!/usr/bin/env python3
"""A fake comments API for trying the site locally with `hugo server`. Nothing is stored on disk.

    python3 api/dev/mock-server.py            # http://127.0.0.1:8787
    cd blog && HUGO_PARAMS_COMMENTS_APIURL=http://127.0.0.1:8787 hugo server -D

It answers like the real API (same routes and JSON), keeps counters and comments in memory, and shows one sample
comment containing HTML so you can see that it is displayed as plain text. A comment you send is accepted (202) but
stays pending, exactly like the real one: it never appears in the list.
"""
import json
import sys
import time
from http.server import BaseHTTPRequestHandler, HTTPServer

PORT = int(sys.argv[1]) if len(sys.argv) > 1 else 8787
counts = {"like": 3, "love": 1, "idea": 0, "party": 0}
comments = [{
    "id": "1",
    "name": "Ana <b>",
    "body": "Muy bueno!\nSegunda línea <script>alert(1)</script>",
    "created": int(time.time() * 1000) - 86_400_000,
}]


class Handler(BaseHTTPRequestHandler):
    def _send(self, status, data):
        body = json.dumps(data).encode()
        self.send_response(status)
        self.send_header("content-type", "application/json")
        self.send_header("access-control-allow-origin", "*")
        self.send_header("access-control-allow-headers", "content-type")
        self.end_headers()
        self.wfile.write(body)

    def do_OPTIONS(self):
        self._send(204, {})

    def do_GET(self):
        if self.path.startswith("/reactions"):
            self._send(200, {"reactions": counts})
        elif self.path.startswith("/comments"):
            self._send(200, {"comments": comments})
        else:
            self._send(404, {"error": "not found"})

    def do_POST(self):
        length = int(self.headers.get("content-length", 0))
        body = json.loads(self.rfile.read(length) or b"{}")
        if self.path.startswith("/reactions") and body.get("kind") in counts:
            counts[body["kind"]] += 1
            self._send(200, {"added": True, "reactions": counts})
        elif self.path.startswith("/comments"):
            print("comment received:", body)
            self._send(202, {"status": "pending"})
        else:
            self._send(400, {"error": "invalid request"})


if __name__ == "__main__":
    print(f"mock API on http://127.0.0.1:{PORT} (Ctrl+C to stop)")
    HTTPServer(("127.0.0.1", PORT), Handler).serve_forever()
