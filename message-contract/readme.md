```shell
RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/*.wasm ./res/
near dev-deploy --wasmFile ./res/message_contract.wasm
```
```
Transaction Id E2EKsm9BKV5BUpuSkPrPxigN5xbLdW4U12YHeFWKpLVg
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/E2EKsm9BKV5BUpuSkPrPxigN5xbLdW4U12YHeFWKpLVg
Done deploying to dev-1656233421007-65579532520509
```