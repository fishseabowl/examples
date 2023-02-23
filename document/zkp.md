https://medium.com/degate/the-simple-introduction-to-zero-knowledge-proofs-6d8639dd6253
### Zero-knowledge proof and zkSNARK

A Zero-knowledge proof is a proof to others that there is a high probability that someone does know or own something without revealing that something that someone knows or possesses. zkSNARK is the most widely used type of zero-knowledge proof in the blockchain, and its full name is ‘zero-knowledge Succinct Non-Interactive Arguments of Knowledge’.

In this section, we will borrow an example to introduce these two concepts in a simple way.

Alice, Bob, and Charlie loved to solve Sudoku problems. Sudoku is a game in which players need to reason out the numbers of all remaining spaces based on the known numbers on a 9×9 board and satisfy that the numbers in each row, column, and block (3x3) contain 1 through 9 and are not repeated.

One day, Bob, who was working on an especially difficult Sudoku that Alice designed complained to Alice that it must be an unsolvable question. Alice devised an ingenious ‘zero-knowledge proof’ method to prove that she really knew the solution without telling Bob the actual solution.

### The Proof

Alice retrieved 81 blank index cards and wrote a single digit (from 1 to 9) on each card. Then she placed the card number representing the puzzle face up and the card number representing the answer to the puzzle face down, in a 9-by-9 matrix.

### The Random Challenge

Next, how can Bob confirm that this is the correct solution? Alice let Bob randomly chose one of the rows, columns, or blocks for verification. If he chose a row, put the 81 cards into 9 opaque bags in 9 rows, shake them well and ensure the index cards inside were mixed well.

### The Verification

Bob opened 9 opaque bags in turn. If each opaque bag contains 9 cards with numbers 1–9, the verification is passed, otherwise the verification fails.

### Succinct

Although there are three situations in the Sudoku that need to be verified (row, column, block), one of them is required to be verified each time. This effectively reduces the verification workload and the actual proof provided to the verifier is a much smaller proof than the original proposition. This is succinctness.

### Repeated

After repeating this random verification step, we assume that Alice is lucky, every time she can guess in advance which verification method Bob will choose, and use this to simulate a solution. The probability of her passing the verification once is 1/3, the probability of passing 2 verifications is 1/9, and the probability of passing 10 verifications is only 1/59049. After tirelessly performing 20 verifications, Bob reluctantly admitted that Alice knew the solution to the answer, because the probability of Alice passing the verification by luck is only 1 in 3.5 billion (This is why the zero-knowledge proof is a proof that holds in probability).

### The Simulation

Then, Charlie also complained to Alice about the unsolvable problem. Alice and Bob repeated the proof just now, but they didn’t expect Charlie’s approval. Charlie proposed a loophole in this proof. If Bob and Alice are in the same group, each time Bob will tell Alice in advance the verification method he wanted to choose, then Alice can easily simulate a proof without a solution to pass these tests.

### Non-Interactive Proofs

It is impossible for everyone who holds this kind of suspicion to repeat the random verification, and the three friends designed a magical machine. Alice only needs to submit the card once, and the machine can be set up according to the initial settings. The verification sequence is automatically repeated for these cards and the verification changed from interactive to non-interactive. We should note that it does not mean that the process of random experiments is not repeated in non-interactive proofs. It’s just that the random point is not given by the verifier, but by a trusted third party in the initialization phase. In this way, the prover can give the proof directly, and the verifier only needs to verify the proof. There is no longer any need for interaction between the verifier and the prover.

### Trusted Setup Ceremony
The most interesting and important part is the initial setup of the verification sequence. Before the machine is started, there will be a row of setting knobs through which the verification method for each round can be selected. When setting these knobs, everyone enters the room where the machine is placed in turn, selects a knob and sets it, and then uses an iron box to completely weld the knob so that other people cannot see or change the selection of the knob. In order to make the initial setting as credible as possible, the friends invited the town mayor, the primary school principal and the police chief, the three most respected elders in the town, to participate in the setting ceremony. Everyone believed that they would never participate in fraud. Therefore, They call it the ‘Trusted Initial Setup Ceremony’.



