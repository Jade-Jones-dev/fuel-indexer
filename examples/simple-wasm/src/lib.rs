extern crate alloc;
use fuel_indexer_macros::{graphql_schema, handler};
use fuels_abigen_macro::wasm_abigen;

graphql_schema!("test_namespace", "schema/schema.graphql");
wasm_abigen!(no_name, "examples/simple-wasm/contracts/my_struct.json");

#[handler]
fn function_one(event: SomeEvent, event2: AnotherEvent) {
    let SomeEvent { id, account } = event;
    let AnotherEvent { hash, .. } = event2;

    let t1 = Thing1 {
        id,
        account: Address::from(account),
    };
    t1.save();

    let t2 = Thing2 {
        id,
        account: Address::from(account),
        hash: Bytes32::from(hash),
    };

    t2.save();
}

#[handler]
fn function_two(event: AnotherEvent) {
    let AnotherEvent { id, hash, .. } = event;

    let Thing1 { account, .. } = Thing1::load(id).expect("No object with that ID");

    let t2 = Thing2 {
        id,
        account,
        hash: Bytes32::from(hash),
    };

    t2.save();
}
