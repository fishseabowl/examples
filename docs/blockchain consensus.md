### Requirement
1. Termination: Eventually, every correct process decides some value.
1. Integrity: If all the correct processes proposed the same value ~v~, then any correct process must decide ~v~.
1. Agreement: Every correct process must agree on the same value.

### Aleph BFT
1. Erasure Coding:
    The goal of erasure coding is to split information i into n blocks and only require k blocks to restore i. An example could be if you want to save an important file distributed to 5 hard drives (n=5) and you want to be able to restore the file even if two of the hard drives are broken (k= 5–2 = 3). For example Reed-Solomon codes can achieve this.

1. Merkle Trees:
    Merkle trees are used to validate the integrity of a file in a faster way, it is especially handy if the data is chunked into blocks. Every block of the file is hashed and then these hashes are concatenated in pairs and then hashed again. This way a tree is built, which has a hash for its root. If we would receive the block L4, we would only need Hash 1–0 and Hash 0 to verify that block L4 is valid for the root hash.
