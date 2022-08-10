# pff
Examine your Internet connection quality in terminal

## Troubleshooting
Issue:
```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 1, kind: PermissionDenied, message: "Operation not permitted" }', src/main.rs:12:25
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Solution:
`sudo setcap cap_net_raw+ep ./target/debug/pff`

Copyright Jędrzej Paweł Maczan 2022