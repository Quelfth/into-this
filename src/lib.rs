pub trait IntoThis {
    fn into_this<T>(self) -> T where Self: Into<T> {
        self.into()
    }
    
    fn try_into_this<T>(self) -> Result<T, Self::Error> where Self: TryInto<T> {
        self.try_into()
    }
}

impl<T> IntoThis for T { }
