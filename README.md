Stellar Split Bill DApp
Stellar Split Bill DApp - Blockchain-Based Decentralized Expense Sharing System

Project Description
Stellar Split Bill DApp is a decentralized smart contract solution built on the Stellar blockchain using the Soroban SDK. It provides a secure, transparent, and automated platform for managing shared expenses and group bills directly on the blockchain. The contract ensures that financial data is stored immutably and calculations (such as cost per person) are handled automatically by the smart contract, eliminating disputes and reliance on centralized split-bill applications.

The system allows users to create bills, specify the total amount and number of participants, and automatically calculate the exact split amount. Each bill is uniquely identified and stored within the contract's instance storage, ensuring fairness, data persistence, and reliability.

Project Vision
Our vision is to revolutionize group finance and expense sharing in the digital age by:

Decentralizing Debt Tracking: Moving expense management from centralized servers to a global, distributed blockchain.

Ensuring Fairness: Automating split calculations through smart contracts so that no one pays more or less than their fair share.

Guaranteeing Immutability: Providing a permanent, tamper-proof record of group expenses that cannot be secretly altered.

Building Trustless Systems: Creating a platform where financial transparency is guaranteed by code, eliminating interpersonal disputes over money.

We envision a future where decentralized applications seamlessly handle group economics, empowering communities and friends with frictionless, transparent financial tools.

Key Features
1. Automated Bill Splitting
Create shared bills with a single function call.

Input the description, total amount, and number of participants.

The smart contract automatically calculates the exact amount per person.

Automated ID generation for unique bill identification.

2. Transparent Expense Retrieval
Fetch all active bills in a single call.

Structured data representation containing total amounts, participant counts, and individual dues.

Real-time synchronization with the blockchain state.

3. Secure Settlement & Deletion
Remove specific bills using their unique IDs once settled.

Clean and efficient storage management.

Immediate update of the bill list after deletion.

4. Transparency and Security
View all expense activities on the public blockchain.

Immutable records of bill creation to prevent unauthorized modifications.

Built-in validation (e.g., preventing zero-participant errors).

5. Stellar Network Integration
Leverages the high speed and incredibly low cost of the Stellar network.

Built using the modern, Rust-based Soroban Smart Contract SDK.

Scalable architecture ready to be integrated with real token transfers.

Contract Details
Contract Address: CBLU4IUASQ4WUMOXBFLZRSBBLILGOH33GS4LUPKFBCCCMJCDQNMF7G2M

(Screenshot has been removed)

Future Scope
Short-Term Enhancements
Direct Token Settlement: Integration with Stellar assets (e.g., USDC, XLM) to allow users to pay their share directly through the smart contract.

Partial Payments: Track who has paid and who hasn't for a specific bill.

Custom Split Ratios: Allow splitting by percentage or specific amounts rather than just equal division.

Receipt Uploads: Link IPFS hashes to attach digital receipts or images to specific bills.

Medium-Term Development
Group Management: Create permanent groups for roommates or travel buddies to track ongoing balances.

Multi-Signature Approvals: Require all participants to approve the bill before it becomes finalized on-chain.

Notification System: Off-chain bridge to alert users when they are added to a new bill or when a settlement is due.

Inter-Contract Integration: Allow other smart contracts (like escrow or treasury contracts) to interact with the split bill contract.

Long-Term Vision
Cross-Border Fiat Settlements: Leverage Stellar's anchors to settle bills in multiple different fiat currencies seamlessly.

DeFi Yield Integration: Stake pooled funds in yield-generating protocols while waiting for group bills to be fully settled.

Decentralized UI Hosting: Host the frontend on IPFS or similar decentralized platforms.

Identity Management: Integration with decentralized identity (DID) systems to tag real usernames instead of wallet addresses.

Enterprise Features
Corporate Expense Accounts: Adapt the system for corporate reimbursement and department-level budget splitting.

Immutable Audit Logging: Create time-locked logs for accounting and tax purposes.

Automated Reporting: Exportable on-chain financial histories for enterprises.

Technical Requirements
Soroban SDK

Rust programming language

Stellar blockchain network

Getting Started
Deploy the smart contract to Stellar's Soroban network and interact with it using the three main functions:

create_bill(description: String, total_amount: u64, participants_count: u32) - Create a new bill; the contract will auto-calculate the split.

get_bills() - Retrieve all active bills from the contract.

delete_bill(id: u64) - Remove a specific bill by its ID once it has been settled.

Stellar Split Bill DApp - Transparent, Trustless, and Fair Expense Sharing on the Blockchain.