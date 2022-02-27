import http.server, socketserver

Handler = http.server.SimpleHTTPRequestHandler
Handler.extensions_map[".wasm"] = "application/wasm"
port = 8889
with socketserver.TCPServer(("", port), Handler) as d:
    print("serving at port", port)
    try:
        d.serve_forever()
    except KeyboardInterrupt:
        print("\nKeyboard interrupt received, exiting.")
    finally:
        d.server_close()
