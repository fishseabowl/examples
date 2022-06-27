1. add_transparency_log method
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
      
2. add_authorized_node method
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

3. query_block method

4. get_authorized_node_list method

5. add_block event

6. Add_block_listener


