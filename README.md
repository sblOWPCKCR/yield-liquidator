# Yield Protocol Liquidator

Liquidates undercollateralized fyDAI-ETH positions using Uniswap V2 as a capital source.

This liquidator altruistically calls the `Liquidations.liquidate` function for any
position that is underwater, trigerring an auction for that position. It then tries
to participate in the auction by flashloaning funds from Uniswap, if there's enough
profit to be made.

## CLI

```
Usage: ./yield-liquidator [OPTIONS]

Optional arguments:
  -h, --help
  -c, --config CONFIG      path to json file with the contract addresses
  -u, --url URL            the Ethereum node endpoint (HTTP or WS) (default: http://localhost:8545)
  -p, --private-key PRIVATE-KEY
                           path to your private key
  -i, --interval INTERVAL  polling interval (ms) (default: 1000)
  -f, --file FILE          the file to be used for persistence (default: data.json)
  -m, --min-profit MIN-PROFIT
                           the minimum profit per liquidation (default: 0)
```

Your contracts' `--config` file should be in the following format where:
 * `Cauldron` is the address of the Cauldron
 * `Witch` is the address of the Witch
 * `Flash` is the address of the PairFlash
 * `Multicall` is the address of the Multicall (https://github.com/makerdao/multicall)
```
{
  "Witch": "0xCA4c47Ed4E8f8DbD73ecEd82ac0d8999960Ed57b",
  "Flash": "0xB869908891b245E82C8EDb74af02f799b61deC97",
  "Multicall": "0xeefba1e63905ef1d7acba5a8513c70307c1ce441"
}
```

The `--private-key` _must not_ have a `0x` prefix. Set the `interval` to 15s for mainnet.

## Building and Running

```
# Build in release mode
cargo build --release

# Run it with 
./target/release/yield-liquidator \
    --config ./addrs.json \
    --private-key ./private_key \
    --url http://localhost:8545 \
    --interval 7000 \
    --file state.json \
```

## How it Works

On each block:
1. Bumps the gas price of all of our pending transactions
2. Updates our dataset of borrowers debt health & liquidation auctions with the new block's data
3. Trigger the auction for any undercollateralized borrowers
4. Try participating in any auctions which are worth buying

Take this liquidator for a spin by [running it in a test environment](TESTNET.md).
