# 🏦 Solana Lending Protocol v2

This project is a lightweight, composable **lending protocol** on Solana, built with
the [Anchor framework](https://book.anchor-lang.com/).
It enables users to deposit and borrow assets (e.g., SOL and USDC), tracks shares for proportional ownership and
interest accrual, and supports liquidation safety via LTV enforcement.

---

## ✨ Features

- ✅ SOL & USDC support
- ✅ Deposit & withdraw with interest accounting
- ✅ Borrow with LTV enforcement
- ✅ PDA-based user/bank/tokens
- ✅ Price feed integration (e.g., via Pyth)

---

## 📦 Program Details

- **Languages:** Rust + Anchor
- **Token Interface:** SPL (via `anchor_spl::token_interface`)

---

## 🧠 Concepts

### 🧮 Share-Based Accounting

Deposits and borrowings are represented via *shares*:

- Prevents rounding errors
- Tracks interest-accruing balances proportionally

### 💸 Interest Accrual

Interest is applied based on:

- `interest_rate` (per second)
- `last_updated` timestamps for user and bank
- Continuous compounding using natural exponential (`e^rt`)

### 💥 Liquidation Safety

Each `Bank` enforces:

- **max_ltv**: Maximum loan-to-value ratio
- **liquidation_threshold**: Triggers undercollateralization
- **liquidation_bonus**: Incentive for liquidators
- *(Liquidation logic is a future milestone)*

---

## 🧾 Instructions

### 🏛 `init_bank`

Initializes a lending `Bank` for a given mint.

```rust
pub fn init_bank(ctx: Context<InitBank>, liquidation_threshold: u64, max_ltv: u64) -> Result<()>
```

### 👤 init_user

Initializes a user account for tracking deposits and borrowings.

```rust
pub fn init_user(ctx: Context<InitUser>, usdc_address: Pubkey) -> Result<()>
```

### 💰 deposit

Transfers tokens to the protocol and mints proportional deposit shares.

```rust
pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()>
```

### 🪙 withdraw

Withdraws tokens and burns corresponding deposit shares. Accrued interest is accounted for.

```rust
pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()>
```

### 📤 borrow

Borrows tokens based on available collateral and price feed data.

```rust
pub fn borrow(ctx: Context<Borrow>, amount: u64) -> Result<()>
```

## 📚 Accounts

### 🧾 User

Stores the deposit and loan state per user across multiple assets.


```text
| Field           | Description                              |
| --------------- | ---------------------------------------- |
| `owner`         | Wallet owner                             |
| `deposited_sol` | SOL deposited (lamports)                 |
| `borrowed_usdc` | USDC borrowed                            |
| `usdc_address`  | User's associated token account for USDC |
| `last_updated`  | Used for interest and health checks      |
```


### 🏦 Bank

Represents an asset pool managed by the protocol.

```text
| Field                   | Description                       |
| ----------------------- | --------------------------------- |
| `mint_address`          | Token mint (e.g., SOL or USDC)    |
| `total_deposits`        | Total value deposited             |
| `total_deposits_shares` | Total shares issued               |
| `max_ltv`               | Max allowed loan-to-value         |
| `interest_rate`         | Interest rate for borrowed tokens |
```

### ⚠️ Errors

Code Message

```text
| Code                     | Message                             |
| ------------------------ | ----------------------------------- |
| `OverLTV`                | Cannot borrow: exceeds max LTV      |
| `UnderCollateralized`    | Position is unsafe                  |
| `InsufficientFunds`      | Cannot withdraw more than deposited |
| `OverRepay`              | Trying to repay more than borrowed  |
| `OverBorrowableAmount`   | Borrow amount exceeds limit         |
| `NotUndercollateralized` | User is not liquidatable            |

```

## 🧪 Development

📥 Install Anchor

```bash
cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked
```

## 🔨 Build & Deploy

```bash
anchor build
```

```bash
anchor deploy
```






