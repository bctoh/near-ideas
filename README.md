# NEAR Ideas

Near Ideas is a Rust Smart Contract project to crowdsource ideas for the Near Protocol. It allows users to add new ideas, remove ideas, or generate previously submitted ideas. This smart contract was built off the Near starter kit.

# Required Software

- Rust 1.58 + cargo
- Node.js
- NEAR CLI 3.1

# Instructions
1. To submit a new idea: `near call learnrust.bctoh.testnet add_idea '{"title": "New Idea"}' --account-id youraccount.testnet
2. To remove an idea: `near call learnrust.bctoh.testnet remove_dea '{"index": _SPECIFY-INDEX-NUMBER-HERE-IN-INT_}`
3. To get a specific idea: `near call learnrust.bctoh.testnet get_idea '{"index": _SPECIFY-INDEX-NUMBER-HERE-IN-INT_}' --accountId youraccount.testnet`

