# Anchor Token Swap  

A decentralized token swap mechanism built on Solana using the Anchor framework.  

## Overview  

This program allows users to create and take token swap offers securely. It leverages Solana's token program and Anchor for program management.  

### Features  
- Users can create swap offers by locking their tokens in a vault.
- Offers specify the desired token type and amount.
- Counterparties can fulfill offers, completing the swap.  

## How It Works  

1. **Make Offer**  
   - User A locks `token_a` in a vault.
   - Specifies that they want `token_b` in return.  

2. **Take Offer**  
   - User B transfers `token_b` to User A.
   - Receives `token_a` from the vault.  

3. **Vault**  
   - Acts as a temporary holding area for tokens.
   - Ensures security until the swap is completed.  

## Prerequisites  

- [Anchor CLI](https://book.anchor-lang.com/chapter_3/installation.html)  
- Rust and Solana CLI installed  

## Installation  

1. Clone the repository:  
   ```bash
   git clone https://github.com/kunals12/anchor-token-swap.git
   cd anchor-token-swap
   ```

2. Install dependancies
    ```
    anchor build
    ```

3. Deploy the program
    ```
    anchor deploy
    ```

## Program Architecture
### Instructions
- make_offer:
Locks the user's tokens in the vault and registers an offer.

- take_offer:
Fulfills an existing offer by transferring the requested tokens and releasing the locked tokens.

### Accounts
- Offer Account: Stores details about the offer (e.g., token types, amounts, and status).

- Vault Account: Holds tokens securely until the offer is fulfilled.

- User Accounts: Source and destination accounts for the tokens involved in the swap.