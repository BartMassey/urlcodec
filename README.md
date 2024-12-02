# urlcodec: simple streaming URL-encoder/decoder
Bart Massey 2024

This Rust binary crate provides a simple "urlencode" encoder
and decoder. Both encoding and decoding can be streamed to
support very large URLs: this is not usually desirable as it
inhibits some convenient whitespace trimming.

The default choice of characters to encode is minimalistic
and somewhat arbitrary.

The code is not particularly tested.

# License

This work is licensed under the "MIT License". Please see the file
`LICENSE.txt` in this distribution for license terms.
