Store Flow
- Main
  - init `network_store` (#305)
  - pass `network_store` to `SubgraphInstanceManager::new` (#416)
- SubgraphInstanceManager
  - calling `start_subgraph_inner`
  - get `writable` store (#173)
  - pass `writable-store` to `inputs` (#361)
  - spawn `SubgraphRunner` (#407)
- SubgraphRunner
  - calling `run`
  - calling `handle_stream_event` (#128)
  - calling `handle_block_process` (#550)
  - calling `process_block` (#145)
- IndexingInputs
- Process


TODO
1. Add `dest: Eithe<Store|Bus>` to subgraph template, parsing, validate etc
2. Read trigger data and pick what `dest` to send data to
