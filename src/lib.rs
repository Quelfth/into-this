pub trait IntoThis {
    fn into_this<T>(self) -> T where Self: Into<T> {
        self.into()
    }
}

impl<T> IntoThis for T { }
