// This code creates a basic server that listens on port 3000. When a request is made to the server, it responds with a "Hello, World!" message. You can run this code by saving it to a file (e.g. server.go) and then running it with the command  go run server.go .

package main
import (
    "fmt"
    "net/http"
)
func main() {
    http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
        fmt.Fprintf(w, "Hello, World!")
    })
    fmt.Println("Starting server on port 3000...")
    http.ListenAndServe(":3000", nil)
}