1. add_transparency_log method: request to add transparency log transactions to blockchain
    1. The transparency module calls the *add_transparency_log* method in the blockchain module
    2. The blockchain module receives this call, builds a block and communicates with other authorized nodes
        1. If a consensus is reached, commit a new block
        2. If no consensus can be reached, the relevant information will be discarded and no new blocks will be committed
    - Related Data Structure
    ```rust
    pub struct Transaction {
      type_id: TransactionType::AddTransparencyLog,
      submitter: Address,
      timestamp: u64,
      payload: Vec<u8>,
      nonce: u128, // Adds a salt to harden
      hash: HashDigest,
      signature: TransactionSignature,
    }
    
    pub struct Block {
      pub header: Header,
      pub transactions: Vec<Transaction>,
      signature: BlockSignature,
    }
    ``` 
    3. If a new block is committed, the blockchain module broadcasts the event *add_block* 
      
2. add_authorized_node method: request to add authorized_node transactions to blockchain(including successful and unsuccessful)
    1. Pyrsia Cli Command
      ```
      pyrsia add_authorized_node -PeerID -PubKey
      ```
    2. The transparency/authorized module receives the cli command add_authorized_node
    3. The transparency/authorized module verifies the new authorized node information and adds the Signature with its own Privkey to create the payload, and calls the *add_authorized_node* method in the blockchain module
    4. The blockchain module receives the call, builds the block and communicates with other authorized nodes
        1. If a consensus is reached, and most of the authorized nodes agree to add this authorized node, commit a new block that successfully added this authorized node
        2. If a consensus is reached, and most of the authorized nodes refuse to add the authorized node, commit a new block that unsuccessfully added this authorized node
        3. If no consensus can be reached, the relevant information will be discarded and no new blocks will be committed.
        - Related Data Structure
        ```rust
        pub struct Transaction {
          type_id: TransactionType::AddAuthorizedNode,
          submitter: Address,
          timestamp: u64,
          payload: Vec<u8>,
          nonce: u128, // Adds a salt to harden
          hash: HashDigest,
          signature: TransactionSignature,
        }
        ```
    5. If a new block is committed, the blockchain module broadcasts the event *add_block*

3. query_block method: request a specific block and a range of blocks from the local node
   1. The transparency module calls the *query_block* method in the blockchain module with a block number or a range of block numbers
   2. The blockchain module returns the relevant block/blocks.

4. get_authorized_node_list method: request the current list of authorized nodes ***perhaps this method should preferably be part of the transparency module/authorized module***
    1. The transparency module calls the *get_authorized_node_list* method(the list of authorized nodes may be maintained by the transparency module/authorized module)
    2. Return the current list of authorized nodes

5. add_block event: broadcast a new block to be committed(Push)
    1. The authorized node broadcast *add_block* event with a new block
    2. When a node receives an *add_block* event, it will call all registered listener methods to update relevant data (transpancy log database and authorized node list)

6. request_block event: request a specific block and a range of blocks from other nodes(Pull)
    1. Node sends *request_block* event to nearby nodes
    2. Other nodes respond to relevant blocks

7. add_block_listener: registered callback functions


