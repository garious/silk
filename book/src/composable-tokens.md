# Composable Tokens

## Problem

Programs cannot modify the accounts owned by other programs. So, for example, only clients can transfer
the ERC20-like tokens from the Token program. If an on-chain program wanted to transfer tokens, it would
need to embed a copy the Token implementation into its own program, such that it would be the owner of
those accounts.

Let's say two users Carol and Dan want to bet on the outcome of a game of tic tac toe between two other
users, Alice and Bob. Each moves 10 Acme tokens into an escrow account, which will then award all 20 tokens
to Carol if Alice wins, to Dan if Bob wins, and otherwise returns the tokens if the game ends in a tie.

The simplest solution is to implement the token, tic tac toe, and the smart contract all within the same
on-chain program. By using the same program, the Solana runtime will permit the program to modify any account,
be it a token account, tic tac toe game state, or the contract. But how to keep things moduluar. If the
token program and tic tac toe programs already exist, how can we create a third program that triggers
transfers between token accounts? That is, how can one program cause modifications to accounts owned by
another? How does a program communicate to another? How does a program convince another that a modification
is authorized by its owner?

First, we need to examine how the token program is currently convinced to modify an account. Next, we
need to consider different ways ownership could be delegated. Last, we need to consider how to put the
pieces together.

## Token transfers

Currently, a token is created by generating a public key from a private key that a user holds. When
that user wants to transfer tokens, they simply sign the transaction containing the transfer instruction.
That signature proves the user holds the private key and therefore is the authorized account holder.
The token program is the owner of all token accounts. The signature acts as a proof that convinces
the token program that an instruction is authorized to modify a particular account.

For another program to modify a token account, we must convince the token program that an account
holder authorized the action. The simplest solution is to authorize a transfer based on the contents
of a particular account. In the case of tac tac toe, Carol would move 10 tokens into a new account,
which transfers its tokens to Dan if the token program is provided an account with tic tac toe
account data indicating that Alice won the game. Dan would also do the same. The instructions from
both Carol and Dan are put into the same message and signed by each. The token program would see
the two signatures and authorize the transfers from each of their accounts into the escrow accounts.
At that point, either Carol or Dan (presumably the winner), would send a second transaction, causing
the token program to release the funds.

## Changes to Budget

Budget currently has witnesses for a signature and a timestamp. It should also offer a witness for
the hash of account data. If that witness was added, one could implement the tic tac toe bet using
lamports, but not tokens. To use tokens, the same feature must be added to the token program.
