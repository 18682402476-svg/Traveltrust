# Scenic Review DApp on Solana

A decentralized scenic spot review platform built on Solana blockchain with AI-powered content moderation.

## Overview

This project implements a Web3 application where users can:
- Submit reviews for scenic spots on-chain
- Earn token rewards for accepted reviews
- Purchase and redeem NFT coupons for scenic visits
- View AI-generated summaries of scenic reviews

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Frontend (React)                       │
│                   app/ (Vite + React)                       │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                   Solana Blockchain                          │
│         programs/scenic_review/ (Anchor Contract)           │
│                                                              │
│  - ScenicSpots    - Reviews    - Coupons    - User Levels   │
└─────────────────────────────────────────────────────────────┘
                              ▲
                              │ Events (ReviewSubmitted, etc.)
┌─────────────────────────────────────────────────────────────┐
│                   Oracle Service (Python)                    │
│                  oracle_service/                             │
│                                                              │
│  - Listens for ReviewSubmitted events                        │
│  - AI Moderation (VolcEngine API)                           │
│  - Submits confirmations (Accept/Reject)                     │
└─────────────────────────────────────────────────────────────┘
```

## Project Structure

```
scenic_review/
├── app/                          # React Frontend
│   ├── App.jsx                   # Main application component
│   ├── main.jsx                  # Entry point
│   ├── index.html                # HTML template
│   ├── package.json              # Frontend dependencies
│   ├── vite.config.js            # Vite configuration
│   └── idl.json                  # Anchor IDL for frontend
│
├── programs/scenic_review/       # Solana Smart Contract
│   ├── src/
│   │   └── lib.rs                # Anchor contract source
│   ├── Cargo.toml                # Rust dependencies
│   └── target/                   # Build artifacts
│
├── oracle_service/               # Oracle Service (Python)
│   ├── main.py                   # Main entry point
│   ├── ai_client.py              # AI API client (VolcEngine)
│   ├── solana_utils.py           # Solana utility functions
│   ├── business_logic.py         # Business logic handlers
│   ├── config.py                 # Configuration management
│   ├── requirements.txt          # Python dependencies
│   ├── .env                      # Environment variables
│   └── oracle-keypair.json       # Oracle wallet (sensitive)
│
├── scripts/                      # Utility Scripts
│   ├── init_all.ts               # Initialize contract accounts
│   ├── update_coupon_templates.js # Update coupon parameters
│   ├── buy_and_redeem_coupon.js   # Coupon purchase & redemption
│   ├── idl.json                  # Contract IDL
│   └── user_keypair.json         # Test user wallet
│
├── migrations/                   # Deployment Scripts
│   └── deploy.js                 # Anchor deployment
│
├── Anchor.toml                   # Anchor project configuration
├── Cargo.toml                    # Rust workspace configuration
└── package.json                  # Project-wide dependencies
```

## Smart Contract

### Core Features

| Feature | Description |
|---------|-------------|
| **Scenic Spots** | Create and manage scenic spot accounts with ratings |
| **Reviews** | Submit on-chain reviews with content and ratings |
| **AI Moderation** | Oracle-powered review approval/rejection |
| **Rewards** | Token rewards for accepted reviews (5 SOL per review) |
| **User Levels** | 3-tier user level system (upgrade costs: 20/30/40 SOL) |
| **Coupons** | NFT coupons with expiration and redemption |
| **Treasury** | Central treasury for funds management |

### Key Accounts

| Account | PDA Seed | Purpose |
|---------|----------|---------|
| Config | `scenic_config` | Global configuration |
| User | `user_account` + user_pubkey | User data & level |
| ScenicSpot | `scenic_spot` + scenic_id | Scenic spot info |
| Review | `review` + scenic_id + review_id | Review data |
| CouponTemplate | `coupon_template` + coupon_id | Coupon definition |
| UserCoupon | `user_coupon` + user + coupon_id | User's coupons |

### Program ID

```
Devnet: 2QWH1NEeJyo8RB4hGymgsj9jwKNCjumKr46yim7bhk9x
```

## Oracle Service

The Oracle service listens for `ReviewSubmitted` events on-chain and performs AI-powered content moderation.

### AI Integration

- **Provider**: Volcano Engine (字节跳动火山引擎)
- **Models**:
  - Audit Model: `bot-20251225155259-nhb82`
  - Summary Model: `bot-20251225152358-mwhdd`

### Workflow

1. Listen for `ReviewSubmitted` event
2. Fetch review content from blockchain
3. Send to AI moderation API
4. Submit confirmation transaction (Accept/Reject)
5. Update review status and distribute rewards

### Running the Oracle

```bash
cd oracle_service
pip install -r requirements.txt
cp .env.example .env  # Configure your environment
python main.py
```

## Scripts

### Initialize Contract

```bash
cd scenic_review
npm install
npx ts-node scripts/init_all.ts
```

### Update Coupon Templates

```bash
node scripts/update_coupon_templates.js
```

### Check Account Status

```bash
npx ts-node scripts/check_init_status.ts
```

## Configuration

### Environment Variables (oracle_service/.env)

| Variable | Description |
|----------|-------------|
| `SOLANA_RPC_URL` | Solana RPC endpoint |
| `ORACLE_PRIVATE_KEY` | Oracle wallet private key |
| `PROGRAM_ID` | Deployed program ID |
| `TOKEN_MINT` | SPL Token mint address |
| `VOLC_AI_API_KEY` | Volcano Engine API key |
| `VOLC_AI_API_URL` | Volcano Engine API endpoint |
| `PINATA_JWT` | Pinata JWT for IPFS (optional) |

## Development

### Prerequisites

- Node.js 18+
- Rust 1.70+
- Solana CLI
- Anchor CLI 0.29.0+
- Python 3.10+

### Build Contract

```bash
cd programs/scenic_review
anchor build
```

### Deploy to Devnet

```bash
anchor deploy --provider.cluster devnet
```

### Run Frontend

```bash
cd app
npm install
npm run dev
```

## Security Notes

- **Never commit private keys** - Use `.env` files and add them to `.gitignore`
- **Oracle wallet** requires sufficient SOL balance for transaction fees
- **AI moderation** helps filter inappropriate content but is not foolproof

## License

MIT
