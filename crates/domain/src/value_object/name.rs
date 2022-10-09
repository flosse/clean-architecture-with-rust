use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct Name<T>(String, PhantomData<T>);

impl<T> Name<T> {
    pub const fn new(name: String) -> Self {
        Self(name, PhantomData)
    }
}

impl<T> AsRef<str> for Name<T> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<T> From<Name<T>> for String {
    fn from(from: Name<T>) -> Self {
        from.0
    }
}
