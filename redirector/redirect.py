#!/usr/bin/env python3

from http.server import HTTPServer, BaseHTTPRequestHandler
import ssl
import logging
import sys
import argparse

is_tls = False # disable for now

def main():
    logger = setup_logger()
    args = parse_args()
    Redirector.target = args.TARGET
    if is_tls:
        httpd.socket = ssl.wrap_socket(
            httpd.socket, certfile="server.crt", keyfile="server.key", server_side=True, ssl_version=ssl.PROTOCOL_TLS)
    else:
        httpd = HTTPServer(("", int(args.PORT)), Redirector)
    logger.info(f"Starting server on port {args.PORT}")
    httpd.serve_forever()

# up here because you might want to modify this

class Redirector(BaseHTTPRequestHandler):
    def __init__(self, arg1, arg2, arg3):
        super().__init__(arg1, arg2, arg3)
        # is terrible, but how else do we get arguments nicely parsed from a function
        # cri
        self.target = None

    def do_GET(self):
        self.send_response(302)
        self.send_header('Location', self.target)
        self.end_headers()
    
    def do_POST(self):
        self.send_response(302)
        self.send_header('Location', self.target)
        self.end_headers()


def setup_logger():
    root = logging.getLogger(__name__)
    root.setLevel(logging.DEBUG)
    handler = logging.StreamHandler(sys.stdout)
    formatter = logging.Formatter('%(asctime)s - %(message)s')
    handler.setFormatter(formatter)
    root.addHandler(handler)
    return root
    
def parse_args():
    parser = argparse.ArgumentParser(description='A simple python redirection server')
    parser.add_argument(
        '-t',
        '--target',
        default="http://localhost:8080",
        dest="TARGET",
        type=str,
        help="the target of redirection",
    )
    parser.add_argument(
        '-p',
        '--port',
        default=8081,
        dest="PORT",
        type=int,
        help="the port to listen on (default: 8081)",
    )
    return parser.parse_args()
    


if __name__ == "__main__":
    main()
