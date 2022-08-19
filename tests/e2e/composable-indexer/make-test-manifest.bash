#!/bin/bash

echo """
namespace: composability_test
graphql_schema: composable-indexer-lib/schema/schema.graphql
wasm_module: composable-indexer-lib/composable_indexer_lib.wasm
handlers:
  - event: LogData
    handler: function_one
""" > manifest.yaml
