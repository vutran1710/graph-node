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
- [ ] Update config to pass bus uri from env
- [ ] Routing/Categorizing analysis
- [ ] Define Bus Types/Config that our graphnode supports in a separate Rust package
- [ ] Define Bus Types/Config that our graphnode supports in a separate Typescript package
