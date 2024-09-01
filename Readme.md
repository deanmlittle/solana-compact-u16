# CompactU16 Readme

## Overview

`CompactU16` is a compact, variable-length encoding for 16-bit unsigned integers (`u16`). It is designed to save space when serializing small integers by using fewer bytes for smaller values. The implementation is inspired by the variable-length quantity (VLQ) encoding used in formats like Protocol Buffers.

## Features

- **Compact Encoding**: Efficiently encodes `u16` integers using 1 to 3 bytes, depending on the size of the value.
- **Optional Borsh Support**: When the `use-borsh` feature is enabled, `CompactU16` implements the `BorshSerialize` and `BorshDeserialize` traits, allowing it to be used with the Borsh serialization format.
- **Error Handling**: Provides detailed error handling to catch cases where the encoded value exceeds the bounds of `u16`.

## Example Usage

### Deserialize `CompactU16` from Bytes

```rust
use compact_u16::CompactU16;

let mut input = &[0x80, 0x01][..];
let value = CompactU16::deserialize(&mut input).unwrap();
assert_eq!(value.0, 0x0080);
```

### Serialize `CompactU16` to Bytes

```rust
use compact_u16::CompactU16;
use borsh::BorshSerialize;

let value = CompactU16(0x3fff);
let mut output = Vec::new();
value.serialize(&mut output).unwrap();
assert_eq!(output, vec![0xff, 0x7f]);
```

## Details

### Encoding

- Values from `0x0000` to `0x007F` are encoded in a single byte.
- Values from `0x0080` to `0x3FFF` are encoded in two bytes.
- Values from `0x4000` to `0xFFFF` are encoded in three bytes.

### Borsh Integration

The `CompactU16` struct supports Borsh serialization and deserialization when the `use-borsh` feature is enabled. This allows easy integration with systems that use the Borsh format.

## Testing

Unit tests are included to verify the correctness of the encoding and decoding. These tests cover a range of values and ensure that the encoding and decoding process is accurate.

### Running Tests

```sh
cargo test
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.