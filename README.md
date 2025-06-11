# Boolean Circuits for Yao's Millionaires Problem

This project implements a framework for defining and evaluating boolean circuits in Rust, serving as the foundational step toward solving Yao's Millionaires Problem using garbled circuits. In this initial phase, the focus is on building and evaluating plain boolean circuits capable of comparing two n-bit binary numbers to determine if one is greater than the other.

## Features
- Logic Gates: Implements And, Or, Equal, and Bigger gates for circuit construction.
- Circuit Structure: Defines Gate and Circuit structs to model boolean circuits.
- Evaluation: Provides a method to evaluate circuits given input bits.
- Comparison Circuit: Includes a function to construct a circuit for comparing two n-bit numbers.
- Utilities: Offers a helper function to convert scalars to bit vectors.
- Benchmarking: Measures performance of circuit construction and evaluation using the criterion crate.

### ~ work in progress ~
