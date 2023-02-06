package main

import "os"
import "log"
import "net/http"

// serve folder as fileserver
func main() {
    addr := os.Args[1]
    port := os.Args[2]
    dir := os.Args[3]
    err := http.ListenAndServe(addr + ":" + port, http.FileServer(http.Dir(dir)))
    if err != nil {
        log.Fatal(err)
    }
}
