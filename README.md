# Yao's Garbled Circuits
The goal of this project is to implement a secure two-party protocol for Yao’s Millionaires Problem using garbled circuits. Yao’s Millionaires Problem is a classical example in the field of Secure Multi-Party Computation (MPC). Two parties, commonly referred to as Alice and Bob, each possess a secret input representing their respective wealth. Their objective is to determine who is richer — that is, to compute whether A > B — without revealing any additional information about their actual values. Formally, the parties hold private inputs A, B ∈ N, and wish to securely evaluate whether A > B.

To solve this problem, we will employ Yao’s garbled circuits protocol, one of the foundational tools in MPC. In this protocol, one party (called the garbler ) represents the desired function as a boolean circuit, replaces all wire values with encrypted keys, and sends this encrypted (or garbled) circuit to the other party (the evaluator ). The evaluator learns its own input keys using oblivious transfer, and receives the garbler’s input keys directly. It then evaluates the garbled circuit without learning anything beyond the final result.

## Todo
- [x] Gate evaluation logic
- [x] Method to evaluate the entire circuit
- [x] Construct a circuit that compares two n-bit numbers
- [x] Helper function to convert a scalar to a bit vector
- [x] Benchmark
- [x] Implement garbling
- [x] Implement OT Protocol
- [ ] Implement a web server and client
