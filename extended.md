Main Tasks
- [x] Define abstract Bus interface/Trait
- [x] Implement specific Bus service (kafka/rabbitmq/etc)
  - [x] enable sending some data
  - [ ] define routing mechanism
- Implement Bus/message-send-request for Wasm
  - [x] customize @graphprotocol/graph-ts
  - [x] define bus-sender in wasm-host
- [x] Allow wasm to call bus




TODO
1. Routing/Categorizing analysis
2. Define Bus Types/Config that our graphnode supports in a separate Rust package
3. Define Bus Types/Config that our graphnode supports in a separate Typescript package
