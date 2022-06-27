1. add_transparency_log method
    1. The transparency module calls the *add_transparency_log* method in the blockchain module
    2. The blockchain module receives this call, builds a block and communicates with other authorized nodes to reach a consensus and commits the block
    3. The blockchain module broadcasts the event *add_block* 
      
2. add_authorized_node method
    1. Pyrsia Cli Command
      ````
      pyrsia add_authorized_node -PeerID -PubKey
      ````
    2. The authorized module receives the cli command add_authorized_node
    3. The authorized module verifies the new authorized node information and adds the Signature with its own Privkey to create the payload, and calls the *add_authorized_node* method in the blockchain module
    4. The blockchain module receives the call, builds the block and communicates with other authorized nodes, reaches a consensus and commits the block
        1. consensus failed....
    5. Blockchain module broadcast event *add_block*

3. query_block method

4. get_authorized_node_list method

5. add_block event

6. Add_block_listener


