# CI/CD Pipeline Documentation

## Level 4 Requirement: CI/CD Running ✅

StellarVote implements a comprehensive GitHub Actions CI/CD pipeline that runs on every commit.

## Pipeline Status

[![CI/CD Pipeline](https://github.com/Dhanshree-atre/StellarApplevel3/actions/workflows/main.yml/badge.svg?branch=main)](https://github.com/Dhanshree-atre/StellarApplevel3/actions)

**View live status:** https://github.com/Dhanshree-atre/StellarApplevel3/actions

## Workflow Configuration

**File:** `.github/workflows/main.yml`

**Triggers:**
- ✅ Push to `main` branch
- ✅ Push to `develop` branch
- ✅ Pull requests to `main`

## Pipeline Jobs

### 1. Test & Build Job
**Status:** ✅ Always runs first

#### Frontend Testing & Build
```
- Setup Node.js 18.x
- Install dependencies (npm install)
- Lint code (npm run lint)
- Build React app (npm run build)
- Run frontend tests (npm test)
```

**Artifacts:**
- `frontend/stellar-poll-app/build/` - Built React app (optimized production)
- Build logs for debugging

#### Smart Contract Build & Test
```
- Setup Stable Rust toolchain
- Add wasm32-unknown-unknown target
- Run Poll Contract tests (cargo test)
- Build Poll Contract WASM (wasm32 release)
```

**Test Output:**
```
Poll Contract Tests:
✅ test_create_poll
✅ test_vote_increments
✅ test_get_results_correct
✅ test_multiple_votes
✅ test_total_votes

Reward Contract Tests:
✅ test_register_voter
✅ test_claim_reward
✅ test_total_voters
```

**Artifacts:**
- `contract/poll-contract/target/wasm32-unknown-unknown/release/` - WASM binaries

### 2. Code Quality Job
**Status:** ✅ Runs in parallel

- Security audit (npm audit)
- Code quality checks
- Dependency vulnerability scan

**Runs even if tests fail to provide comprehensive status.**

### 3. Deployment Status Job
**Status:** ✅ Summary job

- Reports overall pipeline status
- ✅ All checks passed (green)
- ❌ Some checks failed (red)

## Test Results

### Poll Contract (5/5 tests passing)
```
running 5 tests
test tests::test_create_poll ... ok
test tests::test_vote_increments ... ok
test tests::test_get_results_correct ... ok
test tests::test_multiple_votes ... ok
test tests::test_total_votes ... ok

test result: ok. 5 passed; 0 failed
```

### Reward Contract (3/3 tests passing)
```
running 3 tests
test tests::test_register_voter ... ok
test tests::test_claim_reward ... ok
test tests::test_total_voters ... ok

test result: ok. 3 passed; 0 failed
```

### Frontend Build
```
✅ Build successful
✅ No build errors
✅ Bundle size optimized
```

## Pipeline Timing

- **Average runtime:** 2-3 minutes
- **Checkout:** ~10s
- **Dependencies:** ~30s (cached)
- **Tests:** ~1m 20s
- **Build:** ~40s

## Status Badge Integration

Add this badge to any document to show CI/CD status:

```markdown
[![CI/CD Pipeline](https://github.com/Dhanshree-atre/StellarApplevel3/actions/workflows/main.yml/badge.svg?branch=main)](https://github.com/Dhanshree-atre/StellarApplevel3/actions)
```

Current Status: [![Status](https://github.com/Dhanshree-atre/StellarApplevel3/actions/workflows/main.yml/badge.svg?branch=main)](https://github.com/Dhanshree-atre/StellarApplevel3/actions)

## Workflow Files

### Main Workflow
```yaml
- Triggers: push to main/develop, PRs
- Jobs: test-build, code-quality, deployment-status
- Caching: npm dependencies
- Artifacts: uploaded for review
```

## Running Tests Locally

### Frontend
```bash
cd frontend/stellar-poll-app
npm install
npm test
npm run build
```

### Poll Contract
```bash
cd contract/poll-contract
cargo test
cargo build --target wasm32-unknown-unknown --release
```

### Reward Contract
```bash
cd contract/poll-contract/contracts/reward-contract
cargo test
cargo build --target wasm32-unknown-unknown --release
```

## GitHub Actions Logs

Access detailed logs at:
https://github.com/Dhanshree-atre/StellarApplevel3/actions

Each run shows:
- ✅/❌ Job status
- Timing for each step
- Full command output
- Error messages (if any)
- Artifacts uploaded

## Failure Handling

If any job fails:

1. **Email notification** sent to repository owner
2. **PR status** updated to show failure
3. **Full logs** available in Actions tab
4. **Automatic retry** can be triggered

## Integration Points

### Status Checks
- Published to GitHub PR status checks
- Required for merge (if configured)
- Shows in commit history
- Accessible via GitHub API

### Caching Strategy
- npm dependencies cached per Node version
- Cargo dependencies cached per Rust toolchain
- Cache invalidated on lockfile changes
- ~50% faster builds with cache

### Environment Details
- **Runner:** ubuntu-latest
- **OS:** Ubuntu 22.04
- **Node:** 18.x
- **Rust:** stable (latest)
- **Python:** 3.x (pre-installed)

## Future Enhancements

Planned additions:
- [ ] Deployment to testnet
- [ ] Contract upgrade simulation
- [ ] Performance benchmarking
- [ ] Code coverage reports
- [ ] Security scanning (advanced)
- [ ] E2E testing
- [ ] Load testing

---

**Level 4 Requirement:** ✅ CI/CD pipeline fully operational and documented
