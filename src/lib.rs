trait ReplaceErr {
    type Ok;

    fn replace_err<F>(self, err: F) -> Result<Self::Ok, F>;
}

impl<T, E> ReplaceErr for Result<T, E> {
    type Ok = T;

    fn replace_err<F>(self, err: F) -> Result<<Self as ReplaceErr>::Ok, F> {
        self.map_err(|_| err)
    }
}

#[cfg(test)]
mod tests {
    use super::ReplaceErr as _;

    #[test]
    fn it_works() {
        // Result<_, i32>
        let result = Err(1);

        // Result<(), &'static str>
        // have to specify the Ok variant here, usually it will be clear from context.
        let result: Result<(), _> = result.replace_err("hello");

        assert_eq!(result.unwrap_err(), "hello");
    }
}
