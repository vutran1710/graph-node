Main Tasks
- [x] Define abstract Bus interface/Trait
- [x] Implement specific Bus service (kafka/rabbitmq/etc)
  - [x] enable sending some data
  - [ ] define routing mechanism
- Implement Bus/message-send-request for Wasm
  - [x] customize @graphprotocol/graph-ts
  - [ ] define bus-sender in wasm-host
- Allow wasm to call bus


TODO
1. Add `dest: Eithe<Store|Bus>` to subgraph template, parsing, validate etc
2. Read trigger data and pick what `dest` to send data to
3. Customize `WasmRuntimeHost`.
4. Customize `@graphnodeprotocol/graph-ts`: add `Bus` module
