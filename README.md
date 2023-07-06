# ❄️🧊 cryo 🧊❄️

[![Rust](https://github.com/paradigmxyz/cryo/actions/workflows/build_and_test.yml/badge.svg)](https://github.com/paradigmxyz/cryo/actions/workflows/build_and_test.yml)

cryo is the easiest way to extract blockchain data to parquet, csv, or json

## Example Usage

use as `cryo <dataset> [OPTIONS]`

| Example | Command |
| :- | :- |
| Extract all logs from block 16,000,000 to block 17,000,000 | `cryo logs -b 16M:17M` |
| Extract blocks, logs, or traces missing from current directory | `cryo blocks txs traces` |
| Extract to csv instead of parquet | `cryo blocks txs traces --csv` |
| Extract only certain columns | `cryo blocks --include number timestamp` |
| Dry run to view output schemas or expected work | `cryo storage_diffs --dry` |
| Extract all USDC events | `cryo logs --contract 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48` |

`cryo` uses `ETH_RPC_URL` env var as the data source unless `--rpc <url>` is given

## Datasets

cryo can extract the following datasets from EVM nodes:
- `blocks`
- `transactions` (alias = `txs`)
- `logs`
- `call_traces` (alias = `traces`)
- `storage_diffs` (alias for `slot_diffs` + `balance_diff` + `nonce_diffs` + `code_diffs`)
- `slot_diffs`
- `balance_diffs`
- `nonce_diffs`
- `code_diffs`
- `opcode_traces`

## Installation

[wip]

## CLI Options

```
cryo extracts blockchain data to parquet, csv, or json

Usage: cryo [OPTIONS] <DATATYPE>...

Arguments:
  <DATATYPE>...  datatype(s) to collect, one or more of:
                 - blocks
                 - logs
                 - transactions
                 - call_traces
                 - state_diffs
                 - balance_diffs
                 - code_diffs
                 - slot_diffs
                 - nonce_diffs
                 - opcode_traces

Options:
  -h, --help     Print help
  -V, --version  Print version

Content Options:
  -b, --blocks <BLOCKS>              Block numbers, see syntax below [default: 0:latest]
  -a, --align                        Align block chunk boundaries to regular intervals
                                     e.g. (1000, 2000, 3000) instead of (1106, 2106, 3106)
      --reorg-buffer <REORG_BUFFER>  Reorg buffer, save blocks only when they are this old,
                                     can be a number of blocks or a time [default: 0]
  -i, --include-columns [<COLS>...]  Columns to include in output
  -e, --exclude-columns [<COLS>...]  Columns to exclude from output
      --hex                          Use hex string encoding for binary columns
  -s, --sort [<SORT>...]             Columns(s) to sort by

Source Options:
  -r, --rpc <RPC>                    RPC url [default: ETH_RPC_URL env var]
      --network-name <NETWORK_NAME>  Network name [default: use name of eth_getChainId]

Acquisition Options:
  -l, --requests-per-second <limit>  Ratelimit on requests per second
      --max-concurrent-requests <M>  Global number of concurrent requests
      --max-concurrent-chunks <M>    Number of chunks processed concurrently
      --max-concurrent-blocks <M>    Number blocks within a chunk processed concurrently
  -d, --dry                          Dry run, collect no data

Output Options:
  -c, --chunk-size <CHUNK_SIZE>      Number of blocks per file [default: 1000]
      --n-chunks <N_CHUNKS>          Number of files (alternative to --chunk-size)
  -o, --output-dir <OUTPUT_DIR>      Directory for output files [default: .]
      --file-suffix <FILE_SUFFIX>    Suffix attached to end of each filename
      --overwrite                    Overwrite existing files instead of skipping them
      --csv                          Save as csv instead of parquet
      --json                         Save as json instead of parquet
      --row-group-size <GROUP_SIZE>  Number of rows per row group in parquet file
      --n-row-groups <N_ROW_GROUPS>  Number of rows groups in parquet file
      --no-stats                     Do not write statistics to parquet files
      --compression <NAME [#]>...    Set compression algorithm and level [default: lz4]

Dataset-specific Options:
      --contract <CONTRACT>          [logs] filter logs by contract address
      --topic0 <TOPIC0>              [logs] filter logs by topic0 [aliases: event]
      --topic1 <TOPIC1>              [logs] filter logs by topic1
      --topic2 <TOPIC2>              [logs] filter logs by topic2
      --topic3 <TOPIC3>              [logs] filter logs by topic3
      --log-request-size <N_BLOCKS>  [logs] Number of blocks per log request [default: 1]


Block specification syntax
- can use numbers                    --blocks 5000 6000 7000
- can use ranges                     --blocks 12M:13M 15M:16M
- numbers can contain { _ . K M B }  5_000 5K 15M 15.5M
- omiting range end means latest     15.5M: == 15.5M:latest
- omitting range start means 0       :700 == 0:700
- minus on start means minus end     -1000:7000 == 6000:7000
- plus sign on end means plus start  15M:+1000 == 15M:15.001K
```