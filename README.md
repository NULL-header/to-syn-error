# to-syn-error
The util for convertion to error of syn.

# What this crate should be used

this is for developping the macro of Rust, largely. Add, this is useful on conversion between the error that developers define and the error of syn crate.

# How to use

Before developping with this crate, Two crates must be installed the project with `Cargo.toml`.

```toml

[dependencies]
# --- some crates ---
to-syn-error="1.0"
# the following is important!
syn="1.0"
proc_macro2="1.0"

```

Then, as first on a writing a code, declare an enum for error handling with thiserror. Then, add the trait of this crate into the derive macros.

```rust

use thiserror::Error;
use to_syn_error::ToSynError;

#[derive(Error,Debug,ToSynError)]
enum ParseError{
    #[error("it is occurred that is wrong.")]
    Something
}

```

As the end, the error can trans as the error of syn.

```
let input: syn::parse::ParseStream;
let span = input.span();
let error_syn = ParseError::Something.to_syn_error(span);
```

# License

MIT
