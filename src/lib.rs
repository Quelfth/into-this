//! Ever been in a situation where you needed `x.into()` but rust couldn't infer the type?
//! Maybe you tried `x.into::<T>()` but it didn't work because the generic is in the wrong place?
//! You considered `Into::into::<T>(x)`, but it was really ugly?
//! 
//! With this crate, you can import the [`IntoThis`] trait, which is implemented for all types,
//! and then use `x.into_this::<T>()`.  So much cleaner!
//! 
//! Also works with try_into: `x.try_into_this::<T>()`


pub trait IntoThis {
    fn into_this<T>(self) -> T where Self: Into<T> {
        self.into()
    }
    
    fn try_into_this<T>(self) -> Result<T, Self::Error> where Self: TryInto<T> {
        self.try_into()
    }
}

impl<T> IntoThis for T { }
