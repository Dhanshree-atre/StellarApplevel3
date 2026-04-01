# 🌟 StellarVote — Advanced Level 3 Submission

[![CI Pipeline](https://github.com/Dhanshree-atre/StellarApplevel3/actions/workflows/main.yml/badge.svg)](https://github.com/Dhanshree-atre/StellarApplevel3/actions)

A production-ready polling dApp built on the Stellar testnet. Connect your Freighter wallet, cast your vote, and watch results update in real time with transaction transparency.

**Features:**
- ✅ Soroban smart contract voting system  
- ✅ Multi-wallet support (Freighter, Albedo, xBull)  
- ✅ Real-time activity feed with transaction hashes  
- ✅ Transaction status tracking  
- ✅ Mobile responsive UI  
- ✅ Full CI/CD pipeline with automated testing  
- ✅ Error handling (wallet detection, network validation)  

---

## 🔗 Deployment & Contract

| Item | Link |
|------|------|
| **Live Demo** | https://stellarvote1.netlify.app/ |
| **GitHub Repository** | https://github.com/Dhanshree-atre/StellarApplevel3 |
| **CI/CD Pipeline** | [![Tests](https://github.com/Dhanshree-atre/StellarApplevel3/actions/workflows/main.yml/badge.svg?branch=main)](https://github.com/Dhanshree-atre/StellarApplevel3/actions) |
| **Stellar Testnet** | Network: Testnet • Horizon: https://horizon-testnet.stellar.org |
| **Soroban Contract** | Built with Rust/Soroban SDK • Source: `/contract/poll-contract` |

---

## � Mobile Responsive View

The application is fully responsive and works on all devices:

| Desktop | Mobile |
|---------|--------|
| <img width="1920" height="1200" alt="Desktop View" src="https://github.com/user-attachments/assets/3bf23470-ba41-4c78-8ec6-aeeb3168e7e8" /> | Responsive design adapts to all screen sizes |

**Mobile Features:**
- Touch-friendly voting buttons
- Optimized activity feed display
- Responsive wallet sidebar
- Full functionality on phones and tablets

---

## 📋 Submission Requirements Checklist

### Production-Ready Features ✅

| Requirement | Implementation | Evidence |
|---|---|---|
| **Inter-contract Calls** | N/A (Single poll contract) | Poll contract handles votes independently |
| **Custom Token/Pool** | N/A (Uses native XLM) | Transaction fee: 1 stroopVote tracking with XLM|
| **Smart Contract** | Soroban poll contract (Rust) | `/contract/poll-contract/contracts/hello-world/src/lib.rs` |
| **Contract Functions** | `create_poll()`, `vote()`, `get_results()` | Full voting lifecycle |
| **Frontend Integration** | Stellar SDK + Freighter integration | `/frontend/stellar-poll-app/src/contract.js` |
| **Transaction Tracking** | Real-time transaction hashes | Activity feed with transaction IDs |
| **Error Handling** | 5+ error types | Wallet validation, network detection, tx rejection |
| **Mobile Responsive** | React-based responsive design | Tested on desktop and mobile |
| **CI/CD Pipeline** | GitHub Actions | [![CI Status](https://github.com/Dhanshree-atre/StellarApplevel3/actions/workflows/main.yml/badge.svg?branch=main)](https://github.com/Dhanshree-atre/StellarApplevel3/actions) |
| **Testing** | Soroban contract tests | `cargo test` pipeline |
| **Live Deployment** | Netlify | https://stellarvote1.netlify.app/ |
| **Meaningful Commits** | 11+ commits | ✅ Exceeds 8+ requirement |

###🏗️ Architecture

```
stellar-poll-app/
├── contract/                          # Soroban smart contract
│   └── poll-contract/
│       ├── contracts/hello-world/
│       │   └── src/
│       │       ├── lib.rs            # Poll contract implementation
│       │       └── test.rs           # Unit tests
│       └── Cargo.toml
├── frontend/                          # React web application
│   └── stellar-poll-app/
│       ├── src/
│       │   ├── App.js                # Main voting interface
│       │   ├── contract.js           # Stellar SDK integration
│       │   ├── wallet.js             # Freighter wallet logic
│       │   └── ...
│       ├── public/
│       └── package.json
└── .github/
    └── workflows/
        └── main.yml                  # CI/CD pipeline
```

### Smart Contract Details

**Poll Contract** (`Soroban/Rust`)
- `create_poll()` - Initialize poll with options
- `vote()` - Cast vote for option
- `get_poll()` - Retrieve poll details
- `get_results()` - Get current vote counts
- Error handling for invalid options & duplicate polls

---

## 🛠️ Development Setup

### Prerequisites
- **Node.js**: 18+ with npm
- **Rust**: Latest stable (`rustup`)
- **Soroban CLI**: `cargo install soroban-cli`
- **Freighter Wallet**: Chrome extension (https://freighter.app)

### Local Installation

```bash
# 1. Clone repository
git clone https://github.com/Dhanshree-atre/StellarApplevel3.git
cd StellarApplevel3

# 2. Install frontend dependencies
cd frontend/stellar-poll-app
npm install

# 3. Start development server
npm start
```

Access at `http://localhost:3000`

### Smart Contract Development

```bash
# Navigate to contract directory
cd contract/poll-contract

# Run tests
cargo test

# Build WASM (for deployment)
cargo build --target wasm32-unknown-unknown --release
```

### Wallet Setup

1. **Install Freighter**: https://freighter.app
2. **Switch to Testnet**: Click network selector → Testnet
3. **Get testnet XLM**: https://friendbot.stellar.org/?addr=YOUR_ADDRESS
4. **Connect in app**: Click Freighter button → Approve wallet connection

---

## 🚀 Deployment

### Frontend (Netlify)

```bash
npm run build
# Deploy build/ folder to Netlify
```

**Live at:** https://stellarvote1.netlify.app/

### Smart Contract (Soroban Testnet)

```bash
cd contract/poll-contract
soroban contract build
soroban contract deploy --network testnet
```

---

## 📊 CI/CD Pipeline

GitHub Actions automatically runs on every push to `main`:

- ✅ Installs Node and Rust dependencies
- ✅ Builds React frontend (`npm run build`)
- ✅ Runs Soroban contract tests (`cargo test`)
- ✅ Reports results

**View pipeline status:** [Actions](https://github.com/Dhanshree-atre/StellarApplevel3/actions)

---

## 🔐 Technical Stack

| Layer | Technology | Purpose |
|-------|-----------|---------|
| **Smart Contract** | Rust/Soroban SDK | Decentralized poll voting |
| **Frontend** | React + Stellar.js | Web UI & wallet integration |
| **Blockchain** | Stellar Testnet | Testnet transactions |
| **Wallet** | Freighter API | Transaction signing |
| **CI/CD** | GitHub Actions | Automated testing & builds |
| **Deployment** | Netlify | Continuous frontend deployment |

---
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





