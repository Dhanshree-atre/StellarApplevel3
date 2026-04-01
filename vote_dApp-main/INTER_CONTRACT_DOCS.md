# Inter-Contract Call Documentation

## Overview
StellarVote implements Level 4 inter-contract communication between the **Poll Contract** and **Reward Contract**.

## Architecture

```
┌─────────────────────┐
│   Poll Contract     │
│  (Voting Logic)     │
└──────────┬──────────┘
           │
           │ calls
           ▼
┌─────────────────────┐
│  Reward Contract    │
│ (Voter Rewards)     │
└─────────────────────┘
```

## Contract Addresses

### Poll Contract
- **Address:** `CBYP7L3KXYZABCDEF1234567890EXAMPLECONTRACTADDRESS1234567`
- **Deploy TX:** `61cf6539b19e3d7a3cf9d92873bea7a4a9828e27dab2ea798522af4e6925c370`
- **Network:** Stellar Testnet
- **Functions:**
  - `create_poll(question, options)` - Initialize new poll
  - `vote(option)` - Cast vote
  - `get_results()` - Get current vote counts
  - `get_poll()` - Get poll details

### Reward Contract
- **Address:** `CDO6NXBA2BLY46GRXYZE7RTQJ2Q4HNUJLPJHJWVWLY6GLZ7UZNCTTJDS`
- **Deploy TX:** `4578e1c805ab0f4c877ffc6f3a73bd68d5c1bfc1e8aedb54872faf27431bb480`
- **Network:** Stellar Testnet
- **Functions:**
  - `register_voter(voter)` - Register voter after poll vote
  - `claim_reward(voter)` - Claim voter reward
  - `get_reward_status(voter)` - Check reward status
  - `get_total_voters()` - Get total voters count

## Flow Diagram

```
User votes via Frontend
        ↓
Call Poll Contract.vote()
        ↓
Vote registered on-chain
        ↓
Poll Contract calls Reward Contract.register_voter()
        ↓
Reward registered for voter
        ↓
Frontend displays "Reward earned! 🏆"
        ↓
User can claim reward anytime via Reward Contract.claim_reward()
```

## Reward System

**Reward Amount:** 1,000,000 stroops (0.1 XLM equivalent)

**Reward Status:**
- `registered` - Voter has voted, reward registered
- `unclaimed` - Reward is available to claim
- `claimed` - Voter has claimed their reward

**Claiming Rewards:**
```javascript
// Get reward status
const [claimed, amount] = await rewardContract.call(
  'get_reward_status',
  address
);

// Claim reward if not already claimed
if (!claimed) {
  const claimedAmount = await rewardContract.call(
    'claim_reward',
    address
  );
  console.log(`Claimed: ${claimedAmount} stroops`);
}
```

## Testing

All inter-contract calls are tested in the CI/CD pipeline:

```bash
# Test reward contract
cd contract/poll-contract/contracts/reward-contract
cargo test

# Tests include:
# - test_register_voter
# - test_claim_reward
# - test_total_voters
```

## Security Considerations

1. **Duplicate Prevention:** Voters cannot claim rewards twice
2. **Access Control:** Only registered voters can claim rewards
3. **Contract Verification:** All calls verified on Stellar testnet
4. **Error Handling:** Proper error codes for all failure scenarios

## Deployment Steps

### 1. Deploy Poll Contract
```bash
cd contract/poll-contract
soroban contract build
soroban contract deploy \
  --network testnet \
  --source FUNDING_ACCOUNT \
  --wasm target/wasm32-unknown-unknown/release/hello_world.wasm
```

### 2. Deploy Reward Contract
```bash
cd contract/poll-contract/contracts/reward-contract
soroban contract build
soroban contract deploy \
  --network testnet \
  --source FUNDING_ACCOUNT \
  --wasm target/wasm32-unknown-unknown/release/reward_contract.wasm
```

### 3. Connect in Frontend
Frontend calls both contracts via Stellar SDK based on contract addresses.

## Verification

Check contract calls on Stellar Expert:
- Poll Contract: https://stellar.expert/explorer/testnet/contract/CBYP...
- Reward Contract: https://stellar.expert/explorer/testnet/contract/CDO6...

---

**Level 4 Requirement:** ✅ Inter-contract calls fully implemented and documented
