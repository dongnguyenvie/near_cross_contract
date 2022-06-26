```shell
RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/*.wasm ./res/
near dev-deploy --wasmFile ./res/service_contract.wasm

near deploy \
    --wasmFile out/service_contract.wasm \
    --initFunction "new" \
    --initArgs '{}' \
    --accountId dev-1656233957922-76941241592891
```
```
Starting deployment. Account id: dev-1656233957922-76941241592891, node: https://rpc.testnet.near.org, helper: https://helper.testnet.near.org, file: ./res/service_contract.wasm
Transaction Id H6FJYTuW967jtphJjML8r8Nb3dBRRZdJaqLp49MFtPHA
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/H6FJYTuW967jtphJjML8r8Nb3dBRRZdJaqLp49MFtPHA
Done deploying to dev-1656233957922-76941241592891
```

```
near call dev-1656233957922-76941241592891 say_hello  --accountId nolannguyen.testnet             
Scheduling a call: dev-1656233957922-76941241592891.say_hello()
Doing account.functionCall()
Receipt: J4bnZa9SKETbSfMLHvGCvRNXYqhvGxq2LAqWAdyeCg17
        Log [dev-1656233957922-76941241592891]: processor=dev-1656233957922-76941241592891, account_id=nolannguyen.testnet
Transaction Id C3GpZ6syBX8sFkXmR9z9LnNEbNhFJ5y3mda6iovyehdu
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/C3GpZ6syBX8sFkXmR9z9LnNEbNhFJ5y3mda6iovyehdu
'Hello nolannguyen.testnet!'
```