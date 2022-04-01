# `block-id`

[![GitHub Repo stars](https://img.shields.io/github/stars/drifting-in-space/block-id?style=social)](https://github.com/drifting-in-space/block-id)
[![wokflow state](https://github.com/drifting-in-space/block-id/workflows/Rust/badge.svg)](https://github.com/drifting-in-space/block-id/actions/workflows/rust.yml)
[![crates.io](https://img.shields.io/crates/v/block-id.svg)](https://crates.io/crates/block-id)
[![docs.rs](https://img.shields.io/badge/docs-release-brightgreen)](https://docs.rs/block-id/)
[![dependency status](https://deps.rs/repo/github/drifting-in-space/block-id/status.svg)](https://deps.rs/repo/github/drifting-in-space/block-id)

`block-id` is a Rust library for generating opaque, unique, and short string values from (unsigned) integers.

**tl;dr:**

```rust
use block_id::{Alphabet, BlockId};

fn main() {
    // Random seed.
    let seed = 9876;
    
    // Code length.
    let length = 5;

    let generator = BlockId::new(Alphabet::alphanumeric(), seed, length);
    
    // Number to string.
    assert_eq!("wjweA", &generator.encode_string(0));
    assert_eq!("ZxJrE", &generator.encode_string(1));
    assert_eq!("3e0IT", &generator.encode_string(2));

    // String to number.
    assert_eq!(2, generator.decode_string("3e0IT"));
}
```

## Introduction

Random-looking alphanumeric strings are often used in place of sequential numeric IDs for user-facing purposes. This has several advantages:

- String identifiers are usually visually distinct, even if they were generated adjacently in sequence.
- The higher information density of a larger alphabet allows for shorter codes.
- Sequential identifiers reveal unnecessary information about ordering and object creation rate that you may not want to reveal.

`block-id` is the successor to [`tiny_id`](https://github.com/drifting-in-space/block-id), which allows the creation of tightly-packed alphanumeric strings. `tiny_id` turned out to be difficult to use in a distributed environment because its state needs to be synchronized across every node that needs to generate IDs. Rather than building distributed functionality into a short ID generator, `block-id` provides a way of turning a sequential ID generator into a string ID generator by creating a one-to-one mapping between integers and random-looking short strings. That way, any system of generating sequential numeric IDs (for example, a database's sequence generator) can be turned into a system for generating random-looking string IDs.

```rust
use block_id::{Alphabet, BlockId};

fn main() {
    // The alphabet determines the set of valid characters in an ID.
    // For convenience, we include some common alphabets like `alphanumeric`. 
    let alphabet = Alphabet::alphanumeric();
    
    // The generator takes a u128 as a seed.
    let seed = 1234;

    // The length of a generated code. This is really a _minimum_ length; larger numbers
    // will be converted to longer codes since that's the only way to avoid collisions.
    let length = 4;

    // A small amount of pre-caching work happens when we create the BlockId instance,
    // so it's good to re-use the same generator where possible.
    let generator = BlockId::new(alphabet, seed, length);
    
    // Now that we have a generator, we can turn numbers into short IDs.
    assert_eq!("In4R", &generator.encode_string(0));

    assert_eq!("4A7N", &generator.encode_string(440));
    assert_eq!("tSp9", &generator.encode_string(441));
    assert_eq!("6z6y", &generator.encode_string(442));
    assert_eq!("ft0M", &generator.encode_string(443));

    // When we've exhausted all 4-digit codes, we simply move on to 5-digit codes.
    assert_eq!("YeyKs", &generator.encode_string(123456789));

    // ...and so on.
    assert_eq!("pFbrRf", &generator.encode_string(1234567890));

    // Codes are reversible, assuming we have the seed they were generated with.
    assert_eq!(1234567890, generator.decode_string("pFbrRf"));
}
```

## How it works

`block-id` applies a pipeline of reversible transformations on a data in order to turn it into a string.

- **Base conversion** turns the input integer into a base-N representation where N is the number of characters in the desired output alphabet.
- Rounds consisting of:
    - **Permutation** applies an N-to-N map to every digit of the base-N representation. The permutation is generated from the random seed passed in the `BlockId` constructor.
    - **Cascade** applies a left-to-right cumulative sum, modulo N, to the base-N representation.
    - **Rotate** takes the first digit of the base-N representation and moves it to the last digit, shifting all of the other digits left by one.
- **Alphabetization** translates every digit of the base-N representation to a “letter” in the alphabet provided at construction.

The number of rounds is the same as the number of digits in the base-N representation. This gives every digit a chance to influence every other digit.

## Security

`block-id` is designed to make it easy for a human to distinguish between two sequential codes, *not* to make it impossible for an adversary to reverse. It should not be considered cryptographically secure.
