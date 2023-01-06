# office-hasher
A service to hash strings as random lines from the hit TV show The Office (US).
The hash space is bad and this should never be considered for any serious application.
Also I have never actually watched the show.

## How To Use:
Download the repo, and run a `cargo build`, then:
```bash
cargo run
   Compiling office_hash_server v0.1.0 (<some path>)
    Finished dev [unoptimized + debuginfo] target(s) in 2.09s
     Running `target/debug/office_hash_server`
🔧 Configured for debug.
   >> address: 127.0.0.1
   >> port: 8000
   >> workers: 10
   >> ident: Rocket
   >> limits: bytes = 8KiB, data-form = 2MiB, file = 1MiB, form = 32KiB, json = 1MiB, msgpack = 1MiB, string = 8KiB
   >> temp dir: /var/folders/q4/mbwmnpl960q46q7yl6zl4bh80000gn/T/
   >> http/2: true
   >> keep-alive: 5s
   >> tls: disabled
   >> shutdown: ctrlc = true, force = true, signals = [SIGTERM], grace = 2s, mercy = 3s
   >> log level: normal
   >> cli colors: true
📬 Routes:
   >> (index) GET /
   >> (hash_payload) POST /hash application/json
   >> (random_response) GET /random
📡 Fairings:
   >> Shield (liftoff, response, singleton)
🛡️ Shield:
   >> Permissions-Policy: interest-cohort=()
   >> X-Content-Type-Options: nosniff
   >> X-Frame-Options: SAMEORIGIN
🚀 Rocket has launched from http://127.0.0.1:8000
```
Verify the server is working with:
```bash
$ curl http://127.0.0.1:8000/      
Server is running...
```


Once the server is running, you can start requesting immediately.
### Making a request:
```bash
$ curl -X POST http://127.0.0.1:8000/hash -d '{"payload":"hell yeah!"}' -H 'Content-Type: application/json'
{"character":"Toby","line":"Oh.","episode":6,"season":8}%       
```

If you don't want to generate a payload, you can also get a random quote:
```bash
$ curl http://127.0.0.1:8000/random
{"character":"Ryan","line":"We're never going to get what need from that guy. [still shaking head, pats Kevin on the shoulder and walks out of the kitchen]","episode":2,"season":5
```
