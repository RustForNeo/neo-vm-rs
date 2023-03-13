# NEO Virtual Machine in Rust
The NEO Virtual Machine in Rust is an open-source project aimed at improving the performance and security of the NEO ecosystem by implementing the NEO Virtual Machine in Rust. By leveraging the strengths of the Rust programming language, we aim to create a more efficient and secure environment for NEO developers to build their applications.

## Motivation
The field of blockchain technology is constantly evolving, with new developments and advancements being made on a regular basis. One of the most significant areas of growth in recent years has been the adoption of Rust as a preferred programming language for blockchain development due to its performance, efficiency, and security features.

By implementing the NEO Virtual Machine in Rust, we can take advantage of Rust's memory safety and thread safety features, making it an ideal choice for developing applications in the blockchain space, where security is of the utmost importance.

Furthermore, by implementing NEO modules, such as the NEO-VM, in Rust, we can attract a new pool of developers who are experienced in the language and can contribute to the growth of the platform. This can also help NEO developers to implement layer two applications that can execute NEO smart contracts off-chain in Rust efficiently.

## Goals
The main goals of this project are to:

- Implement the NEO Virtual Machine in Rust
- Deliver a set of Rust crates that will provide various NEO-VM functions, cryptography operations, and exception handling
- Improve the performance and security of the NEO ecosystem
- Attract a new pool of developers to the NEO ecosystem

## Deliverables
The project will deliver a set of Rust crates that will implement various NEO-VM functions, cryptography operations, and exception handling. Specifically, the crates will include:

- Cryptography crate that performs bit operations and Murmur32. Test cases will be implemented to ensure correct functionality.
- Neo virtual machine Types crate that implements various types used in Neo-VM such as array, boolean, buffer, bytestring, etc. Test cases will be included to validate the implementation.
- Script-related functions crate that includes the OpCode and Script functions, which handle script building and format checking. Test cases will be provided for verification.
- Exception handling crate that handles exceptions that may occur during the execution of the VM. Test cases will be available for validation.
- The main body of the VM, including the stack, context, and engine, will be implemented as the final crate of this project.