## Problems
The partitioning aspect of sharding, however, raises a significant potential problem: without downloading and validating the entire history of a particular shard the participant cannot necessarily be certain that the state with which they interact is the result of some valid sequence of blocks and that such sequence of blocks is indeed the canonical chain in the shard. A problem that doesn’t exist in a non-sharded blockchain.

## Solutions
There are two major directions of properly solving data validity: fishermen and cryptographic proofs of computation.

### Fishermen
The idea behind the first approach is the following: whenever a block header is communicated between chains for any purpose (such as cross-linking to the beacon chain, or a cross-shard transaction), there’s a period of time during which any honest validator can provide a proof that the block is invalid. There are various constructions that enable very succinct proofs that the blocks are invalid, so the communication overhead for the receiving nodes is way smaller than that of receiving a full block.

### Succinct Non-interactive Arguments of Knowledge (ZKP)
