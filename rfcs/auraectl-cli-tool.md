# RFC: auraectl CLI utility

## Overview

`auraectl` is a CLI tool that can be used to administer and query all
aspects of `aurae`:

- PKI
- configuration
- scaffolding services and applications
- logs and metrics
- runtime status
- deployment

## Proposal

`auraectl` is built in Rust, using the `clap` and `config` crates 
to manage command line arguments and configuration files.

Output will support human-readable text and JSON when applicable, in order
to make it useful both for human interactive usage and for scriptability.

### PKI management

In a first step, it will support managing the PKI infrastructure:

- generating SSH keys
- generating the CA keys and certificates
- generating and signing server certificates and keys
- generating and signing client certificates and keys
- displaying and querying all the above

For the implementation, I want to use `rust-openssl` for the PKI management,
and ed25519_dalek for the SSH key generation.

## Open questions and future ideas

- How do we unify error handling and reporting?
This is especially useful when auraectl is used in scripts.

- What configuration files do we want to use and for what?

- How do we ensure proper versioning of the different resources (CLI tool, config files, generated files, aurae runtime, etc...) in the long term?
