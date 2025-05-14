from http.server import HTTPServer, BaseHTTPRequestHandler

class SimpleHTTPRequestHandler(BaseHTTPRequestHandler):

    def do_GET(self):
        self.send_response(200)
        self.end_headers()
        self.wfile.write(b'Hello, World!')

def start_server():
    print("Starting server")
    httpd = HTTPServer(('', 8000), SimpleHTTPRequestHandler)
    httpd.serve_forever()
