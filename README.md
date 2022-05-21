## error handling
for this example a simple text file logging is used as error handling.
based on severity of the error this instead would have to be escalated to the responsible person/system/department.
logging to the console is not possible due to the requirements to deliver the results to std::out so logging there would interfere with automatic parsing.

## type safety
all transactions are parsed into spezializations that only expose the allowed fields. this way e.g. accessing the amount field with a dispute transaction is impossible.  
special care was given to not expose mutable states where they are not needed to avoid accidental sideeffects.

## robustness
no unsafe transforms or direct unwraps/casts are used in the production part of the code (unwrapping in tests is fine as tests count as failed if a panic occurs).  
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

### assumptions:
- account states do not have to be persisted apart from printing it to std::out.

- chronological order does not matter for deposits/withdrawals. the sum, even after resolving disputes must be the same regardless of ordering.

- resolve + chargeback referencing the same transaction -> file order decides what is applied?  
  -> *assumption*: chronological order matters for disputes. if a transaction is disputed it matters if it is first resolved or charged back. whatever comes first invalidates what comes after.

- do incoming transaction still change funds after an account was frozen or are transactions afterwards rejected?  
  -> *assumption*: per definition, no transactions can be made after an account was frozen so all transactions afterwards will be rejected.

- negative deposit/withdrawal - invalid transaction -> Reject?  
  -> *assumption*: negative values are invalid and will be rejected.

- can someone dispute a dispute? or dispute a resolve/chargeback?  
  -> *assumption*: has a lot of complex scenarios that would have at least to be mentioned in the procedure description and how to handle them correctly -> ignoring transactions of this kind.
