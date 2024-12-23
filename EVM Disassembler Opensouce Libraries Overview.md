# Disassembly Tooling Evaluation

## Introduction

In order to pick one of these tools, I decided to compare them in the following dimensions:
- Extensibility — how is the tool capable of being extended?
- Interoperability — how is the tool capable of being integrated into a given pipeline?
- Performance — how well does the tool perform?
- Feature Richness — is it considered out of the box, and how well is it suited with useful features?

## evm-dis

### Overview:
evm-dis is a tool designed for disassembling Ethereum Virtual Machine (EVM) bytecode. Its primary function is to convert raw bytecode into a human-readable format, facilitating analysis of smart contracts.

### Extensibility:
evm-dis offers basic disassembly capabilities but lacks modularity for extending its functionality beyond its core purpose. Adding new features or integrating advanced analyses would require significant modifications to its codebase.

### Interoperability (Scripting Support):
The tool provides command-line outputs suitable for scripting. However, it doesn’t offer native bindings for popular scripting languages, which might limit seamless integration into larger automated workflows.

### Performance:
evm-dis performs efficiently for straightforward disassembly tasks. However, without advanced optimization features, its performance may not scale well for complex analyses or large datasets.

### Feature Richness:
While effective at disassembling bytecode, evm-dis lacks advanced features such as control flow analysis, decompilation, or symbolic execution, which are crucial for in-depth smart contract analysis.

## Heimdall-rs

### Overview:
Heimdall-rs is an advanced EVM smart contract toolkit specializing in bytecode analysis and extracting information from unverified contracts. Written in Rust, it is designed to be fast, modular, and more accurate than other existing tools.

### Extensibility:
Heimdall-rs is built with a modular architecture, allowing developers to extend its capabilities by adding new modules or integrating it with other tools. This design facilitates customization and enhancement to meet specific analysis requirements.

### Interoperability (Scripting Support):
The toolkit offers a command-line interface and can be integrated into larger workflows. While it doesn’t provide direct bindings for scripting languages, its modular design and command-line utilities enable effective scripting and automation.

### Performance:
Written in Rust, Heimdall-rs benefits from the language’s performance optimizations, offering efficient analysis even for complex smart contracts. Its design ensures scalability and responsiveness during intensive tasks.

### Feature Richness:
Heimdall-rs provides a comprehensive suite of features, including:

- EVM Bytecode Disassembly
- EVM Smart-Contract Control Flow Graph Generation
- EVM Smart-Contract Decompilation
- Smart-Contract Storage Dumping
- Raw Transaction Calldata Decoding
- Raw Transaction Trace Decoding

## R2/rizin (additional option)

### Overview:
_r2_[^1]/_rizin_[^2] is a disassembler and debugger framework that can be used for analyzing EVM bytecode after contribution made by me in 2023[^3]. It has a long history of success and production use, with a large and active community.

### Extensibility:
r2/rizin has a modular architecture, allowing developers to extend its capabilities by adding new modules or integrating it with other tools. It provides bridges to be scripted to more than eight popular languages, including Python, Java, and Go.

### Interoperability (Scripting Support):
The tool offers a command-line interface and can be integrated into larger workflows. Its scripting capabilities enable effective automation, and its cross-platform GUI interface built on QT provides a user-friendly experience[^4] [^5].

### Performance:
r2/rizin is designed to be fast and efficient, with support for a wide range of binary architectures, including EVM. Its performance optimization features ensure scalability and responsiveness during intensive tasks.

### Feature Richness:
r2/rizin provides a wide range of features, including disassembly, debugging, and analysis capabilities. Its feature-richness, combined with its extensibility and interoperability, make it a powerful tool for analyzing EVM bytecode.

## Alternative Solutions

There are a lot of open-source disassembling and low-level debugging tools within the Ethereum developers' tools ecosystem.

Some of them good and up-to-date: _etk_[^6], the others are incomplete and outdated: _evmdis_[^7], _ethersplay 0.1.0_[^8], _pyevmasm 0.2.3_[^9], _ethereum-dasm 0.1.10_[^10], _py-evm 0.6.1-alpha.2_[^11].

Closed source tools do stuff too, and some are even as feature-rich as r2 [^12] [^13], but both are not open-sourced nor free.

## Conclusion

If to compare just the two options provided in the task definition I definitely would pick **Heimdall-rs** as more mature and comprehensive solution. But if to take a broader picture there are few more options worth to consider exists including such bulletproof ones as **r2**, **Binary Ninja** and **IDA Pro**.

[^1]: [r2](https://github.com/radareorg/radare2)
[^2]: [rizin](https://github.com/rizinorg/rizin)
[^3]: [PR with EVM support](https://github.com/radareorg/radare2/pull/21306)
[^4]: [iaito](https://github.com/radareorg/iaito)
[^5]: [cutter](https://github.com/rizinorg/cutter)
[^6]: [etk 0.2.1](https://github.com/quilt/etk/tree/master/etk-dasm)
[^7]: [evmdis](https://github.com/Arachnid/evmdis)
[^8]: [ethersplay 0.1.0](https://github.com/crytic/ethersplay)
[^9]: [pyevmasm 0.2.3](https://github.com/crytic/pyevmasm)
[^10]: [ethereum-dasm 0.1.10](https://github.com/tintinweb/ethereum-dasm)
[^11]: [py-evm 0.6.1-alpha.2](https://github.com/ethereum/py-evm)
[^12]: [Binary Ninja](https://binary.ninja)
[^13]: [IDA Pro](https://hex-rays.com/IDA-pro/)
