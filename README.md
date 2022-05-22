## error handling
for this example a simple text file logging is used as error handling.
based on severity of the error this instead would have to be escalated to the responsible person/system/department.
logging to the console is not possible due to the requirements to deliver the results to std::out so logging there would interfere with automatic parsing.

## type safety
all transactions are parsed into spezializations that only expose the allowed fields. this way e.g. accessing the amount field with a dispute transaction is impossible.  
special care was given to not expose mutable states where they are not needed to avoid accidental side effects.  
complex behaviors are always hidden behind at least one layer of abstraction
another important aspect is the "single source of truth". states must only ever defined by one set of data for one specific point in time.  
an example would be the "try_into" implementations for the transaction types, which take ownership of the parsed transaction and transforms it into its specialization without ever allowing to access both.
another example is the "Funds" struct that makes sure that the total is always calculated instead of directly set to avoid impossible states.

## robustness
no unsafe transforms or direct unwraps/casts are used in the production part of the code (unwrapping in tests is fine as tests count as failed if a panic occurs).  
the libraries that have been used are all staples of the rust community.  
there are several unit tests making sure that the program behaves correctly, even under pressure.  
disclaimer: i had to remove the larger test cases to keep the repository at a reasonable size.

## efficiency
the parsing has a line-by-line approach and should thus not load the whole file into memory.  
deposits and withdrawals have to be kept in memory after parsing because they can be referenced by dispute transactions later on.  
in order to be able to throw away dispute/resolve/chargeback transactions after applying their data, deposits and withdrawals are stored with flags to show their dispute status instead.  
there are opportunities to avoid copies still in the code, but they are mostly done this way to keep the code safe and/or readable.

## parallelization
opportunities to multithread the program are seemingly sparse.  
because the order of the transactions overall matters (at least thats what i assume) parsing the file out of order would need the file to be in memory completely to do a re-ordering after the parsing.  
pre-sorting the transactions to each account and doing the calculations for each account balance in a threadpool would in theory be possible but, because of all the batching and distribution effort, this is not really worth it, especially considering that multiple instances of the program would run on a server which then again means that the cores are exhausted anyway.

## clean code
in order to stay on the stable branch of rust i had to refrain from using some features that would increase the overall readability e.g. [let..else](https://rust-lang.github.io/rfcs/3137-let-else.html) with early returns.  
overall the files are a little bit longer than i would usually prefer but i tried to comply with the standards the Rust community is setting which is leaving connected structs and functions mostly in the same file.  
i am adaptable in that regard and will tune my style towards the context of the team i am working with.  
my assumption is that this piece of code is going to be evaluated by experts and it was also written under that assumption. that means i did not comment on things i feel are obvious as the code should speak for itself in most cases.


### assumptions:
- account states do not have to be persisted apart from printing it to std::out.

- chronological order does not matter for deposits/withdrawals. the sum, even after resolving disputes must be the same regardless of ordering.

- chronological order does matter for disputes. if a transaction is disputed it matters if it is first resolved or charged back. whatever comes first invalidates what comes after.

- a frozen account does not allow ANY other transactions, including other chargebacks. this further assumes that chargebacks are not inevitable facts coming from another party, otherwise they would still have to be deducted after an account was frozen.

- negative values for deposits and withdrawals are invalid and will be rejected.
