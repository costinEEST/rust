/**
 * Look up the standard library for the `Result` type and write a basic implementation of it.
 * */

enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Result<T, E> {
    // https://chat.openai.com/share/b0dc82d9-2b8d-47ed-aee7-6b10ad3b5e34
    pub fn is_ok(&self) -> bool {
        matches!(self, Self::Ok(_))
    }

    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }

    pub fn unwrap(self) -> T {
        match self {
            Self::Ok(val) => val,
            Self::Err(_) => panic!("called `Result::unwrap()` on an `Err` value"),
        }
    }

    pub fn unwrap_err(self) -> E {
        match self {
            Self::Ok(_) => panic!("called `Result::unwrap_err()` on an `Ok` value"),
            Self::Err(err) => err,
        }
    }

    pub fn map<U, F>(self, op: F) -> Result<U, E>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Self::Ok(val) => Result::Ok(op(val)),
            Self::Err(err) => Result::Err(err),
        }
    }
}
