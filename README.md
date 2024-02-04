# rust-baseconv

Simple Base Conversion Utility in Rust

This demonstrates:

- conversion from numbers to byte arrays
- basic argument processing
- string formatting

## Example usage

- double input: `basec 5.432`
- float input: `basec 5.432f`
- integer input: `basec 100`
- hex input:
  - big-endian: `basec 0x12345`
  - big-endian (alt1): `basec 12345h`
  - big-endian (alt2): `basec ">0x12345"`
  - big-endian (alt3): `basec ">12345h"`
  - little-endian: `basec "<0x12345"`
  - little-endian (alt): `basec "<12345h"`
