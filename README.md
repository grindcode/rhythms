# rhythms

A rhythmic pattern generation library in Rust with `no_std` support.

[Documentation](https://docs.rs/rhythms/)

[Release notes](https://github.com/grindcode/rhythms/releases)

## Work In Progress

This project is under development and the current API is subjective to change. Please use at your own risk.

## Example

```
use rhythms::Pattern;

// Initialize the Pattern struct with a maximum of 64 steps
let pattern = Pattern::<64>::new(4, 2, 0);
assert_eq!([true, false, true, false], pattern.as_slice());

// or
let mut pattern = Pattern::with_length(4);
pattern.pulses(2);
pattern.rotate(-1);
assert_eq!([false, true, false, true], pattern.as_slice());
```

## License

This project is licensed under either of

- [Apache License, Version 2.0](https://github.com/grindcode/rhythms/blob/main/LICENSE-APACHE)
- [MIT license](https://github.com/grindcode/rhythms/blob/main/LICENSE-MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in rhythms by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.