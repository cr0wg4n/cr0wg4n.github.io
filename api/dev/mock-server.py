#!/usr/bin/env python3
"""A fake comments API for trying the site locally with `hugo server`. Nothing is stored on disk.

    python3 api/dev/mock-server.py            # http://127.0.0.1:8787
    cd blog && HUGO_PARAMS_COMMENTS_APIURL=http://127.0.0.1:8787 hugo server -D

It answers like the real API (same routes and JSON): counters and comments are kept in memory per post, so the English
and the Spanish page of a post share them. A comment you send is published at once (201) and shows up in the list.
Start it with `--hold` to answer like the API does during a flood: comments are accepted (202) but stay hidden.
"""
import json
import sys
import time
from http.server import BaseHTTPRequestHandler, HTTPServer
from urllib.parse import parse_qs, urlparse

args = [a for a in sys.argv[1:] if not a.startswith("--")]
PORT = int(args[0]) if args else 8787
HOLD = "--hold" in sys.argv
KINDS = ("like", "love", "idea", "party")

counts = {}    # post key -> {kind: n}
comments = {}  # post key -> [comment]


def post_key(raw):
    """'/posts/x', '/posts/x/' and '/es/posts/x/' are all the post 'posts/x'."""
    raw = (raw or "").strip().strip("/")
    if raw.startswith("es/"):
        raw = raw[3:]
    return raw if raw.startswith("posts/") and "/" not in raw[6:] and raw[6:] else None


def sample(key):
    if key not in counts:
        counts[key] = {"like": 3, "love": 1, "idea": 0, "party": 0}
        comments[key] = [{
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
        url = urlparse(self.path)
        q = parse_qs(url.query)
        key = post_key((q.get("key") or q.get("path") or [""])[0])
        if not key:
            return self._send(400, {"error": "invalid page"})
        sample(key)
        if url.path.startswith("/reactions"):
            self._send(200, {"reactions": counts[key]})
        elif url.path.startswith("/comments"):
            self._send(200, {"comments": comments[key]})
        else:
            self._send(404, {"error": "not found"})

    def do_POST(self):
        length = int(self.headers.get("content-length", 0))
        body = json.loads(self.rfile.read(length) or b"{}")
        key = post_key(body.get("key") or body.get("path"))
        if not key:
            return self._send(400, {"error": "invalid page"})
        sample(key)
        if self.path.startswith("/reactions") and body.get("kind") in KINDS:
            counts[key][body["kind"]] += 1
            self._send(200, {"added": True, "reactions": counts[key]})
        elif self.path.startswith("/comments"):
            print("comment received:", {k: v for k, v in body.items() if k != "ts"})
            comment = {"id": str(len(comments[key]) + 1), "name": body.get("name", ""), "body": body.get("body", ""),
                       "created": int(time.time() * 1000)}
            if HOLD:
                self._send(202, {"status": "pending"})
            else:
                comments[key].append(comment)
                self._send(201, {"status": "published", "comment": comment})
        else:
            self._send(400, {"error": "invalid request"})


if __name__ == "__main__":
    print(f"mock API on http://127.0.0.1:{PORT}{' (holding new comments)' if HOLD else ''} (Ctrl+C to stop)")
    HTTPServer(("127.0.0.1", PORT), Handler).serve_forever()
