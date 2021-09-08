## Testing

In this guide, you will:
1. Deploy the yield contracts
2. Run the liquidator 
3. See the liquidator trigger the liquidation 
4. After some time, see the liquidator participate in the auction

### Deploy the contracts

First we must clone the contracts and install the deps:

```
git clone git@github.com:sblOWPCKCR/vault-v2.git
git checkout liquidation
yarn
```

In one terminal, run hardhat node with mainnet fork: `yarn hardhat node --network hardhat`

In another terminal, deploy the contracts: `yarn hardhat run --network localhost scripts/deploy.ts`
It deploys Yield-V2 and prints out 3 important pieces of information:
* block number at the time of deployment
* owner's address and private key
* a json snippet with addresses of the deployed contracts

Store the private key in a file (/tmp/pk) without the '0x' prefix, store the json snippet in another file (config.json)

### Run the liquidator

In a new terminal, navigate back to the `yield-liquidator` directory and run:
```
RUST_BACKTRACE=1 RUST_LOG=yield_liquidator=trace cargo run -- --chain_id 31337 -c config.json -p /tmp/pk -s BLOCK_NUMBER_AT_TIME_OF_DEPLOYMENT
```

You should see logs appear like below:

```
Sep 12 15:32:28.477  INFO yield_liquidator: Starting Yield-v2 Liquidator.
Sep 12 15:32:28.482  INFO yield_liquidator: Profits will be sent to 0xf364fdfe5706c4c274851765c00716ebad06eb6a
Sep 12 15:32:28.482  INFO yield_liquidator: Node: http://localhost:8545
Sep 12 15:32:28.482  INFO yield_liquidator: Cauldron: 0xf065bb3cf180501bb29e18439cb89a41355c766a
Sep 12 15:32:28.482  INFO yield_liquidator: Witch: 0x768f51adefc0c11c1be34551cfc139845a1a900b
Sep 12 15:32:28.482  INFO yield_liquidator: Multicall: Some(0xeefba1e63905ef1d7acba5a8513c70307c1ce441)
Sep 12 15:32:28.482  INFO yield_liquidator: FlashLiquidator 0xf5a8491d0b8e16fea73f38e2e40e9f6e41222791
Sep 12 15:32:28.482  INFO yield_liquidator: Persistent data will be stored at: "data.json"
...
Sep 12 15:32:29.520 DEBUG eloop{block=13206687}:monitoring: yield_liquidator::borrowers: New vaults: 1
...
Sep 12 15:32:29.602 DEBUG eloop{block=13206687}: yield_liquidator::keeper: trigger liquidations block_number=13206687
...
Sep 12 15:32:29.602  INFO eloop{block=13206687}: yield_liquidator::liquidations: found undercollateralized vault. triggering liquidation vault_id=[237, 153, 226, 176, 234, 143, 239, 206, 190, 162, 13, 60] details=Vault { is_collateralized: false, under_auction: false, level: -0.5, ink: [69, 84, 72, 0, 0, 0], art: [240, 197, 170, 61, 0, 223] }
...
Sep 12 15:32:30.818 DEBUG eloop{block=13206688}: yield_liquidator::liquidations: new auction vault_id=[237, 153, 226, 176, 234, 143, 239, 206, 190, 162, 13, 60] auction=Auction { started: 1631486819, debt: 1000000000000000000, collateral: 1000000000000000000, debt_id: [68, 65, 73, 0, 0, 0], collateral_id: [69, 84, 72, 0, 0, 0], debt_address: 0x6b175474e89094c44da98b954eedeac495271d0f, collateral_address: 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2 }
...
Sep 12 15:32:36.673 TRACE eloop{block=13206688}:buying{vault_id=[237, 153, 226, 176, 234, 143, 239, 206, 190, 162, 13, 60] auction=Auction { started: 1631486819, debt: 1000000000000000000, collateral: 1000000000000000000, debt_id: [68, 65, 73, 0, 0, 0], collateral_id: [69, 84, 72, 0, 0, 0], debt_address: 0x6b175474e89094c44da98b954eedeac495271d0f, collateral_address: 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2 }}: yield_liquidator::liquidations: Submitted buy order tx_hash=PendingTransaction { tx_hash: 0xb0bb6c659cb4abe41a53c9b68b519ec8737c40781c9ee5de758f1003b94fd362, confirmations: 1, state: PendingTxState { state: "InitialDelay" } }
...
Sep 12 15:32:36.681 TRACE eloop{block=13206689}: yield_liquidator::liquidations: confirmed tx_hash=0xb0bb6c659cb4abe41a53c9b68b519ec8737c40781c9ee5de758f1003b94fd362 gas_used=363216 user=[237, 153, 226, 176, 234, 143, 239, 206, 190, 162, 13, 60] status="success" tx_type="auctions"
```

Here we started with a vault that was already undercollaterized. If you don't want that, bump up the oracle price before launching the bot:
```
seth send SPOT_SOURCE_ADDRESS "set(uint)" "1000000000000000000"
```

And this takes the vault uunder water:
```
seth send SPOT_SOURCE_ADDRESS "set(uint)" "500000000000000000"
```