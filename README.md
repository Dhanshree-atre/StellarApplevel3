# 🌟 StellarVote — Yellow Belt + Level 3 Submission

A live polling dApp built on the Stellar testnet. Connect your Freighter wallet, cast your vote, and watch results update in real time.

---

## 🔗 Links

| Item | Link |
|------|------|
| **Live Demo** | https://stellarvote1.netlify.app/ |
| **Deployed Contract** | CA_REPLACE_WITH_YOUR_REAL_CONTRACT_ID|
| **Stellar Expert** | https://stellar.expert/explorer/testnet/contract/CA_REPLACE_WITH_YOUR_REAL_CONTRACT_ID |

---

## 📸 Screenshots

Level 2 
<img width="1909" height="1119" alt="Screenshot 2026-03-19 224239" src="https://github.com/user-attachments/assets/e46aff03-1234-4100-b063-4c7a6f2919b4" />
<img width="1890" height="1086" alt="Screenshot 2026-03-19 224311" src="https://github.com/user-attachments/assets/ed925296-a157-4946-b608-076ecf0efb01" />
<img width="1917" height="1087" alt="Screenshot 2026-03-19 224323" src="https://github.com/user-attachments/assets/49a9e408-c73b-4efa-b284-dd975fb8bd51" />


Level 3
> <img width="1920" height="1200" alt="Screenshot 2026-03-04 195737" src="https://github.com/user-attachments/assets/3bf23470-ba41-4c78-8ec6-aeeb3168e7e8" />
"C:\Users\ASUS\Videos\Captures\React App - Google Chrome 2026-03-19 19-45-26.mp4"



---

## ✅ Yellow Belt Requirements Checklist

| Requirement | Implementation |
|---|---|
| ✅ 3 error types handled | Wallet not found, User rejected, Wrong network |
| ✅ Contract deployed on testnet | See contract address above |
| ✅ Contract called from frontend | `castVote()` and `getResults()` in `src/contract.js` |
| ✅ Transaction status visible | Pending / Success ✅ / Failed ❌ shown below the poll |
| ✅ Multi-wallet support | Freighter, Albedo, xBull via `@stellar/freighter-api` |
| ✅ Real-time event integration | Live Activity Feed + auto-refresh every 5 seconds |
| ✅ Loading states | Voting button disables + shows "Processing..." |
| ✅ Basic caching | Poll results cached in frontend state |
| ✅ Smart contract testing | `cargo test` verifies poll creation, votes, results |
| ✅ Clean UI improvements | Activity feed + status indicators |

| ✅ Minimum 2+ meaningful commits | See commit history on GitHub |

---

## 🛠️ Setup Instructions

### Prerequisites
- Node.js 18+ and npm
- Rust & Cargo
- [Freighter Wallet](https://freighter.app) Chrome extension
- Freighter set to **Testnet** network
- Soroban CLI installed (`cargo install soroban-cli`)

### Install & Run

```bash
# 1. Clone the repo
git clone https://github.com/YOUR_USERNAME/stellar-poll-app.git
cd stellar-poll-app

# 2. Install dependencies
npm install

# 3. Start development server
npm start
```

App opens at `http://localhost:3000`

### Connect Your Wallet

1. Install [Freighter](https://freighter.app) Chrome extension
2. Open Freighter → click the network name → switch to **Testnet**
3. Get free testnet XLM from [Stellar Friendbot](https://friendbot.stellar.org/?addr=YOUR_ADDRESS)
4. Click the **Freighter** card in the sidebar → click **Approve** in the popup
5. Your balance and address will appear in the sidebar

---

## 🚀 Deploy to Vercel (Live Demo)

```bash
# Install Vercel CLI
npm install -g vercel

# Build and deploy
npm run build
vercel --prod
```

Or connect your GitHub repo at [vercel.com](https://vercel.com) for auto-deploy on every push.

---

## 📁 Project Structure

```
src/
├── App.js          # Main UI — wallet panel, poll card, activity feed
├── wallet.js       # useWallet hook — Freighter connection, balance, send XLM
├── contract.js     # Soroban contract calls — castVote, getResults
└── index.js        # React entry point

contract/
├── live_poll/
│   ├── src/
│   ├── Cargo.toml
```

---

## 🔑 How It Works

### Wallet Connection (`wallet.js`)
Uses `@stellar/freighter-api` directly:
```js
await api.requestAccess();      // Opens Freighter popup → user approves
const key = await api.getPublicKey();  // Returns Stellar address
```

### Contract Calls (`contract.js`)
Calls the deployed Soroban smart contract on testnet:
```js
// Vote
contract.call("vote_yes", Address.fromString(wallet).toScVal())

// Read results
contract.call("get_results")
```

### Real-time Sync

Poll results auto-refresh every 5 seconds

Activity feed shows every vote with address, timestamp, and TX hash

Your vote appears at the top of the feed instantly after confirmation

### 3 Error Types Handled
```
⛔ Rejected     — User dismissed the Freighter approval popup
🌐 Wrong network — Wallet is on Mainnet, not Testnet  
❌ Not found    — Freighter extension is not installed


## Level 3 Enhancements
Loading State & UX

Voting button disables while transaction is processing

Shows Processing... feedback

Prevents double transactions


Basic Caching

Poll results temporarily stored in frontend state

Reduces repeated blockchain calls


Smart Contract Testing

Inside contract folder:

cargo test

Verifies:

Poll creation

Voting logic

Result storage


Clean UI Improvements

Activity feed shows real-time votes

Transaction status (Pending / Success / Failed)

Multiple wallet support integrated
```

### Real-time Sync
- Poll results auto-refresh every **5 seconds**
- Activity feed shows every vote with address, timestamp, and TX hash
- Your vote appears at the top of the feed instantly after confirmation

---

## 🔍 Transaction Verification

After casting a vote, the transaction hash is shown in the status bar.  
Verify it on Stellar Expert:

```
https://stellar.expert/explorer/testnet/tx/YOUR_TX_HASH
```

**Example transaction hash from a contract call:**  
`4cf1d790d2...` *(copy full hash from the app after voting)*

---

## 📦 Depend~encies

| Package | Version | Purpose |
|---|---|---|
| `stellar-sdk` | 12.3.0 | Stellar transactions + Horizon API |
| `@stellar/freighter-api` | 2.1.0 | Freighter wallet connection |
| `react` | 19.x | UI framework |
---

## 🏗️ Smart Contract

The poll contract is written in Rust using the Soroban SDK and deployed to Stellar testnet.

**Contract functions:**
- `vote_yes(voter: Address)` — records a YES vote
- `vote_no(voter: Address)` — records a NO vote  
- `get_results()` — returns `(yes_count, no_count)`

**Deploy your own:**
```bash
# Install Stellar CLI
cargo install stellar-cli

# Build contract
cd contract
cargo build --target wasm32-unknown-unknown --release

# Deploy to testnet
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/poll.wasm \
  --source YOUR_ACCOUNT \
  --network testnet
```  





