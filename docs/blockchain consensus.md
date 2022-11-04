### Requirement
1. Termination: Eventually, every correct process decides some value.
1. Integrity: If all the correct processes proposed the same value ~v~, then any correct process must decide ~v~.
1. Agreement: Every correct process must agree on the same value.

### Aleph BFT
1. Erasure Coding:
    The goal of erasure coding is to split information i into n blocks and only require k blocks to restore i. An example could be if you want to save an important file distributed to 5 hard drives (n=5) and you want to be able to restore the file even if two of the hard drives are broken (k= 5–2 = 3). For example Reed-Solomon codes can achieve this.

1. Merkle Trees:
    Merkle trees are used to validate the integrity of a file in a faster way, it is especially handy if the data is chunked into blocks. Every block of the file is hashed and then these hashes are concatenated in pairs and then hashed again. This way a tree is built, which has a hash for its root. If we would receive the block L4, we would only need Hash 1–0 and Hash 0 to verify that block L4 is valid for the root hash.

1. Reliable Broadcast
    This algorithm ensures that all parties receive an input value v. The sender applies erasure coding to the input and receives N blocks. A merkle tree is computed over all blocks and the root hash is stored in h. Then the sender sends one block with the root hash and the corresponding tree branch to each party. Upon receiving this message, every party forwards its received block to all other parties with a message called ECHO. This ensures that the sending of the value doesn’t create a network bottleneck at the sender.
Whenever a block is received in an ECHO message, the block is validated against the merkle root. When enough (N-f) ECHO messages have been received, the blocks are interpolated from the received messages with erasure coding and the blocks are validated by calculating the merkle tree. Also if no READY message has been sent, a READY message is sent.

1. Threshold Cryptography
    Threshold Cryptography enables to distribute multiple secret keys among n parties and k of those parties need to collaborate to create a signature or to decrypt a ciphertext. For signatures each party can sign the message and with k signature shares a valid signature, that verifies under the public key, can be created. For decryption each party can produce a decryption share and with k decryption shares the original message, that was encrypted with the public key, can be obtained.

1. Cryptographic Common Coin
    For this algorithm threshold signatures are required. The Common Coin lets us create a random number, which is only revealed when at least f+1 parties have called the GetCoin method. All parties receive the same value. This is achieved by creating a collaborative signature on the coin sid, the random number is simply the signature itself, which can not be known before at least f+1 parties revealed their share and it can not be influenced by the adversary.

1. Binary Byzantine Agreement
    This algorithm allows the honest nodes to agree on the value of a single bit. The common coin is used for synchronization in the binary agreement. If the coin matches the majority vote, then this is the decided value. The majority can only be influenced by the adversary until the coin is revealed.

1. Common Subset Agreement
    The binary agreement and the reliable broadcast are used to construct the common subset agreement. In this algorithm every node inputs a value v to the agreement, which is delivered to all the other nodes with the reliable broadcast and the binary agreement is used to vote if the value of a node is included. The vote is yes (1) if the majority received the value through the broadcast and no (0) otherwise. The union of all values, for which the binary agreement was yes, is the outcome of this algorithm and every honest node knows the values that were agreed on. This algorithm is the centerpiece of the honey badger algorithm.
