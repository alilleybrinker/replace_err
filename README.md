# `replace_err`

Adds a method, `replace_err` to `Result` which throws away the current error and
replaces it with a new error type, equivalent to `result.map_err(|_| new_error_value)`.

## License

This code is dual-licensed MIT and Apache 2.0. Take your pick.

