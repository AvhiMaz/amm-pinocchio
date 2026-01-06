# AMM Pinocchio

A lightweight Automated Market Maker (AMM) implementation for Solana built using the Pinocchio framework. This program implements a constant product AMM with LP tokens, swaps, and liquidity management.

## Features

- Constant product AMM (x * y = k)
- LP token minting and burning
- Token swaps with configurable fees
- Add/remove liquidity
- Built with Pinocchio for optimal performance

## AMM Formulas

### Swap Formula

The swap uses the constant product formula with fees:

```
amount_in_with_fee = amount_in * (10000 - fee_rate) / 10000
amount_out = (reserve_out * amount_in_with_fee) / (reserve_in + amount_in_with_fee)
```

Where:
- `fee_rate` is in basis points (1 basis point = 0.01%, 10000 basis points = 100%)
- Fees are deducted from the input amount before calculating output

### Add Liquidity Formula

**Initial liquidity:**
```
lp_tokens = sqrt(amount_a * amount_b)
```

**Subsequent liquidity:**
```
lp_a = (amount_a * total_lp_supply) / reserve_a
lp_b = (amount_b * total_lp_supply) / reserve_b
lp_tokens = min(lp_a, lp_b)
```

### Withdraw Liquidity Formula

```
amount_a_out = (lp_amount * reserve_a) / total_lp_supply
amount_b_out = (lp_amount * reserve_b) / total_lp_supply
```

## Instructions

The program supports four main instructions:

1. **Initialize** - Create a new liquidity pool
   - Creates pool PDA and LP mint
   - Sets fee rate and vaults
   - Derives pool authority

2. **Add Liquidity** - Add tokens to the pool
   - Transfers tokens from user to vaults
   - Mints LP tokens proportional to contribution
   - Updates pool reserves

3. **Swap** - Exchange one token for another
   - Transfers input token from user to vault
   - Transfers output token from vault to user
   - Applies fee and updates reserves

4. **Withdraw** - Remove liquidity from pool
   - Burns LP tokens
   - Transfers proportional amounts of both tokens to user
   - Updates pool reserves

## Build Commands

### Build the program
```bash
make build
# or
cargo build-sbf
```

### Check code
```bash
make check
# or
cargo check
```

### Run clippy
```bash
make clippy
# or
cargo clippy --all-targets --all-features -- -D warnings
```

### Format code
```bash
make fmt
# or
cargo fmt && cargo +nightly fmt --all
```

## Testing

Run all tests:
```bash
make test
# or
cargo test
```

## Benchmarking

Run all benchmarks:
```bash
make bench
# or
cargo bench --bench initializer_ix_bench && \
cargo bench --bench add_liquidity_ix_bench && \
cargo bench --bench swap_ix_bench && \
cargo bench --bench withdraw_ix_bench
```

Run specific benchmark:
```bash
cargo bench --bench swap_ix_bench
cargo bench --bench add_liquidity_ix_bench
cargo bench --bench withdraw_ix_bench
cargo bench --bench initializer_ix_bench
```

## Run Everything

Run all checks, tests, and build:
```bash
make all
```

This runs: format, check, clippy, build, test, and bench in sequence.

## Project Structure

```
.
├── src/
│   ├── constants.rs       # Program constants and seeds
│   ├── entrypoint.rs      # Program entrypoint
│   ├── helper.rs          # Helper functions (sqrt)
│   ├── lib.rs             # Library root
│   ├── instructions/      # Instruction processors
│   │   ├── add_liquidity.rs
│   │   ├── initializer.rs
│   │   ├── swap.rs
│   │   ├── withdraw.rs
│   │   ├── validators.rs
│   │   └── utils.rs
│   └── states/
│       └── pool.rs        # Pool state definition
├── tests/                 # Integration tests
├── bench/                 # Performance benchmarks
├── client/                # TypeScript client library
└── Cargo.toml
```

## Pool State

The pool account stores:
- `authority` - Pool PDA authority
- `token_a` - First token mint
- `token_b` - Second token mint
- `lp_mint` - LP token mint
- `vault_a` - Token A vault
- `vault_b` - Token B vault
- `reserve_a` - Token A reserve amount
- `reserve_b` - Token B reserve amount
- `fee_rate` - Fee in basis points
- `bump` - Pool PDA bump seed
- `lp_mint_bump` - LP mint PDA bump seed

## Seeds

- Pool PDA: `["pool", token_a, token_b, bump]`
- LP Mint PDA: `["lp_mint", pool, bump]`

## Dependencies

- `pinocchio` (0.9.2) - Lightweight Solana program framework
- `pinocchio-system` (0.4.0) - System program instructions
- `pinocchio-token` (0.4.0) - Token program instructions
- `bytemuck` (1.14) - Zero-cost byte manipulation

## Development Dependencies

- `mollusk-svm` (0.9.0) - Solana VM testing framework
- `mollusk-svm-bencher` (0.9.0) - Benchmarking utilities
- `solana-sdk` (3.0.0) - Solana SDK
- `spl-token` (9.0.0) - SPL Token library

## Build and Maintain 

Avhi ([Web](avhi.in) | [X](x.com/avhidotsol))
