#!/usr/bin/env python3

from http.server import HTTPServer, BaseHTTPRequestHandler
import ssl

target = f"https://localhost:8081/?owo=owo"
port = 8081


class Redirect(BaseHTTPRequestHandler):
    def do_POST(self):
        self.send_response(302)
        self.send_header('Location', target)
        self.end_headers()


httpd = HTTPServer(("", int(port)), Redirect)
# httpd.socket = ssl.wrap_socket(
#     httpd.socket, certfile="server.crt", keyfile="server.key", server_side=True, ssl_version=ssl.PROTOCOL_TLS)
print("Starting...")
httpd.serve_forever()
