#!/usr/bin/env python3
"""Local reverse proxy that fetches akasha.cv for akasha-service.

Cloudflare answers requests from this host with a JS challenge when the TLS
fingerprint looks like a generic HTTP client. A browser fingerprint gets a
normal response, so the Rust service points its base URL here instead of at
akasha.cv directly.

Only GET requests under /api/ are forwarded, and the listener is bound to
loopback so the proxy can never be reached from outside the host.

Environment:
    PORT                  listen port (default 8097)
    UPSTREAM_ORIGIN       default https://akasha.cv
    IMPERSONATE           curl_cffi browser profile (default chrome)
    UPSTREAM_TIMEOUT_SECS default 30
"""

import json
import os
import sys
import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

from curl_cffi import requests

ORIGIN = os.environ.get("UPSTREAM_ORIGIN", "https://akasha.cv").rstrip("/")
IMPERSONATE = os.environ.get("IMPERSONATE", "chrome")
PORT = int(os.environ.get("PORT", "8097"))
TIMEOUT = float(os.environ.get("UPSTREAM_TIMEOUT_SECS", "30"))

_local = threading.local()


def session() -> requests.Session:
    """One session per thread so connections are not shared across threads."""
    current = getattr(_local, "session", None)
    if current is None:
        current = requests.Session(impersonate=IMPERSONATE)
        _local.session = current
    return current


class Handler(BaseHTTPRequestHandler):
    protocol_version = "HTTP/1.1"

    def do_GET(self) -> None:  # noqa: N802 - http.server naming
        if self.path == "/health":
            self._send_json(200, {"status": "ok", "upstream": ORIGIN})
            return

        if not self.path.startswith("/api/"):
            self._send_json(
                404, {"error": "not_found", "message": "only /api/* is proxied"}
            )
            return

        try:
            upstream = session().get(
                ORIGIN + self.path,
                timeout=TIMEOUT,
                headers={"Accept": "application/json"},
            )
        except Exception as error:  # noqa: BLE001 - report any transport failure
            self._send_json(
                502, {"error": "upstream_unavailable", "message": str(error)}
            )
            return

        body = upstream.content
        self.send_response(upstream.status_code)
        self.send_header(
            "Content-Type", upstream.headers.get("content-type", "application/json")
        )
        self.send_header("Content-Length", str(len(body)))
        self.send_header("Cache-Control", "no-store")
        self.end_headers()
        self.wfile.write(body)

    def _send_json(self, status: int, payload: dict) -> None:
        body = json.dumps(payload).encode()
        self.send_response(status)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def log_message(self, format: str, *args) -> None:  # noqa: A002
        sys.stderr.write("%s %s\n" % (self.address_string(), format % args))


def main() -> None:
    server = ThreadingHTTPServer(("127.0.0.1", PORT), Handler)
    print(
        f"akasha-fetch-proxy listening on 127.0.0.1:{PORT} -> {ORIGIN} ({IMPERSONATE})",
        flush=True,
    )
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        server.server_close()


if __name__ == "__main__":
    main()
