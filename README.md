# Extended UTXO Framework for Substrate Runtimes

The Extended UTXO model is a storage model that is optimized for providing a partial ordering over
state transitions and detecting transitions that conflict with one another.

This repository presents the idea and (aspirational) provides an implementation for writing
blockchain runtimes in the Substrate framework.

## EUTXO model

The Extended UTXO storage model is a single large set. Pieces of state are either in the set or not.
State transitions (called transactions henceforth in the blockchain tradition) can interact with
that state in three fundamental ways:

* Produce new UTXOs
* Consume existing UTXOs
* Peek at (read, but don't consume) existing UTXOs

Notice there is no native mutability of individual pieces of state during their existence. They can
only be produced, peeked at, and consumed. This is not limiting however as mutability can be
expressed in terms of a consume followed by a produce of the new data. Expressive developer-friendly
smart contracting languages that _do_ have mutability can be constructed on this foundation as we
will see in the example.

Peek is not necessary for completeness as it can be expressed as a consume followed by a
produce, however it greatly increases concurrent throughput as we will see in the examples.

TODO This needs to be fleshed out a lot. I did a rough draft of it a few years ago
https://drive.google.com/file/d/11KiQND-tPFxqaYihbs6JIM8fNgVkGDai/view?usp=sharing

### Currency Example

The term UTXO (unspent transaction output) comes from Bitcoin, the first blockchain to use it for
fungible token transactions. There is already a concrete example of this in Substrate at

https://github.com/substrate-developer-hub/utxo-workshop/blob/master/runtime/src/utxo.rs

### Biological Reproduction Example

Consider the sexual reproduction of mamals. A simple `reproduce` transaction would peek at a mother,
and peek at a father to get some genes from each. It would produce a new offspring. A simple `die`
transaction would comsume a single UTXO person.

A slightly more complex example could separate the process into two steps. `conceive` peeks at a
father, consumes a fertile mother, and produces a pregnant mother. Then `give_birth` consumes a
pregnant mother and produces a fertile mother (identical to the one before conception) and an
offspring.

### Smart Contract Example

Concurrent programming models such as Robin Milner's Pi Calculus are designed with a concurrency
top of mind. This makes them a natural candidate for expression in the EUTOX model. This is the
example that is most thoroughly fleshed out in my rough draft [same link as intro]
(https://drive.google.com/file/d/11KiQND-tPFxqaYihbs6JIM8fNgVkGDai/view?usp=sharing).

#### What about my favorite VM

RAM-register state machines like the EVM or the JVM can be implemented in this model where each
storage location is mapped to a UTXO. This mapping may be considered a strength for programmers who
want their existing codebases to #JustWork, And some concurrency benefits can be gained this way but
for most real-world contracts this approach does not fully utilize the concurrent potential of
EUTXOs.

### More Concurrency Primitives

The introduction discussed the importance of `peek` for concurrency. Concurrency can be further
improved by introducing more concurrent primitives. For example, consider an addition cell. The cell
may be added to multiple times, and no ordering is necessary between the two as long as no peeks or
consumes happen on it in between.

Each additional primitive comes with additional complexity in terms of resolving life-lines. This
repo makes no attempt to add such primitives, and focuses entirely on peek.

## Substrate Implementation

This is so WIP it can't even be described. I'll either be coding a pallet that exposes EUTXO to
downstream pallets, or a trait that codifies UTXOs or both. Perhaps the pallet would eventually be
an alternative to FRAME system, but that is not a goal. For now we will work within FRAME.
