import http.server
import socketserver

class Server:
    port = 8080
    handler = ''

    def __init__(self):
        Handler = self.http.server.SimpleHTTPRequestHandler
        http = socketserver.TCPServer(("", self.port), Handler)


# # Import libraries
# import http.server
# import socketserver

# # Defining PORT number
# PORT = 8080

# # Creating handle
# Handler = http.server.SimpleHTTPRequestHandler

# # Creating TCPServer
# http = socketserver.TCPServer(("", PORT), Handler)

# # Getting logs
# print("serving at port", PORT)
# http.serve_forever()
