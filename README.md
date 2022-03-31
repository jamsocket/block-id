# `block_id`

`block_id` is a Rust library for generating opaque, unique, and short string values from (unsigned) integers.

## Introduction

Random-looking alphanumeric strings are often used in place of sequential numeric IDs for user-facing purposes. This has several advantages:

- String identifiers are usually visually distinct, even if they were generated adjacently in sequence.
- The higher information density of a larger alphabet allows for shorter codes.
- Sequential identifiers reveal unnecessary information about ordering and object creation rate that you may not want to reveal.

`block_id` is the successor to [`tiny_id`](https://github.com/paulgb/tiny_id), which allows the creation of tightly-packed alphanumeric strings. `tiny_id` turned out to be difficult to use in a distributed environment because its state needs to be synchronized across every node that needs to generate IDs. Rather than building distributed functionality into a short ID generator, `block_id` provides a way of turning a sequential ID generator into a string ID generator by creating a one-to-one mapping between integers and random-looking short strings. That way, anything system of generating sequential numeric IDs (for example, a database's sequence generator) can be turned into a system for generating random-looking string IDs.

```rust
use block_id::{Alphabet, BlockId};

fn main() {
    // The alphabet determines the set of valid characters in an ID.
    // For convenience, we include some common alphabets like `alphanumeric`. 
    let alphabet = Alphabet::alphanumeric();
    
    // The generator takes a u128 as a seed.
    let seed = 1234;

    // The minimum length of a generated code.
    let min_length = 4;

    // A small amount of pre-caching work happens when we create the BlockId instance,
    // so it's good to re-use it if possible.
    let generator = BlockId::new(alphabet, seed, min_length);
    
    // Now that we have a generator, we can turn numbers into short IDs.
    assert_eq!("hBiG", &generator.encode_string(0));

    assert_eq!("tSp9", &generator.encode_string(440));
    assert_eq!("6z6y", &generator.encode_string(441));
    assert_eq!("ft0M", &generator.encode_string(442));
    assert_eq!("qaHw", &generator.encode_string(443));

    // When we've exhausted all 4-digit codes, we simply move on to 5-digit codes.
    assert_eq!("8ldpN", &generator.encode_string(123456789));

    // ...and so on.
    assert_eq!("mI5hHw", &generator.encode_string(1234567890));

    // Codes are reversible, assuming we have the seed they were generated with.
    assert_eq!(1234567890, generator.decode_string("mI5hHw"));
}
```

## How it works

`block_id` applies a pipeline of reversible transformations on a data in order to turn it into a string.

- **Base conversion** turns the input integer into a base-N representation where N is the number of characters in the desired output alphabet.
- Rounds consisting of:
    - **Permutation** applies an N-to-N map to every digit of the base-N representation. The permutation is generated from the random seed passed in the `BlockId` constructor.
    - **Cascade** applies a left-to-right cumulative sum, modulo N, to the base-N representation.
    - **Rotate** takes the first digit of the base-N representation and moves it to the last digit, shifting all of the other digits left by one.
- **Alphabetization** translates every digit of the base-N representation to a “letter” in the alphabet provided at construction.

The number of rounds is the same as the number of digits in the base-N representation. This gives every digit a chance to influence every other digit.

## Security

`block_id` is designed to make it easy for a human to distinguish between two sequential codes, *not* to make it impossible for an adversary to reverse. It should not be considered cryptographically secure.
