//! `error_hook` is a library for automation to insert action at error conversion.
//!
//! # Example
//! ```
//! use error_hook_attr::hook;
//!
//! #[hook(e => println!("{e}"))]
//! fn test(a: i32, b: i32) -> error_hook::Result<i32> {
//!     a.checked_mul(b).ok_or(anyhow::anyhow!("overflow"))
//! }
//!
//! fn main() -> anyhow::Result<()> {
//!     let _ = test(888888888, 888888888);
//!     //      ^^^^ this prints 'overflow'
//!
//!     Ok(())
//! }
//! ```

#[cfg(feature = "attribute")]
pub use error_hook_attr::hook;
use thiserror::Error;

#[derive(Error, Debug)]
#[error(transparent)]
pub enum Error {
    Boxed(Box<dyn std::error::Error + Send + Sync>),
    Anyhow(#[from] anyhow::Error),
}

#[doc(hidden("or you will be fired"))]
pub struct SecretStructDoNotUseOrYouWillBeFired<E, F>(E, F);

/// Internal use only
impl<F> From<SecretStructDoNotUseOrYouWillBeFired<Box<dyn std::error::Error + Send + Sync>, F>>
    for Error
where
    F: FnOnce(&Box<dyn std::error::Error + Send + Sync>),
{
    fn from(
        source: SecretStructDoNotUseOrYouWillBeFired<Box<dyn std::error::Error + Send + Sync>, F>,
    ) -> Self {
        let SecretStructDoNotUseOrYouWillBeFired(src, hook) = source;
        hook(&src);
        Self::Boxed(src)
    }
}

/// Internal use only
impl<F> From<SecretStructDoNotUseOrYouWillBeFired<anyhow::Error, F>> for Error
where
    F: FnOnce(&anyhow::Error),
{
    fn from(source: SecretStructDoNotUseOrYouWillBeFired<anyhow::Error, F>) -> Self {
        let SecretStructDoNotUseOrYouWillBeFired(src, hook) = source;
        hook(&src);
        Self::Anyhow(src)
    }
}

/// Type alias for hook.
///
/// The return value of a function that attaches a hook attribute should be of this type (required).
pub type Result<T> = std::result::Result<T, Error>;

/// for meta programming
pub trait SameAs<T> {}

/// Literally
impl<T> SameAs<T> for T {}

#[doc(hidden("or you will be fired"))]
pub trait SecretTraitDoNotUseOrYouWillBeFired<T, E> {
    fn into_ghost<F>(
        self,
        hook: F,
    ) -> std::result::Result<T, SecretStructDoNotUseOrYouWillBeFired<E, F>>
    where
        Self: SameAs<std::result::Result<T, E>>,
        F: FnOnce(&E);
}
///　Trait to convert Err type of Result to dynamic trait object.
///
/// Since hook only supports `Box<dyn std::error::Error>` and `anyhow::Error`,
/// please use this trace to convert it to a dynamic trace object if necessary.
pub trait IntoBoxed<T> {
    ///　Converts Err type of Result to dynamic trait object.
    fn into_boxed(self) -> std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
}

///　Trait to convert Err type of Result to dynamic trait object.
///
/// Since hook only supports `Box<dyn std::error::Error>` and `anyhow::Error`,
/// please use this trace to convert it to a dynamic trace object if necessary.
impl<T, E: std::error::Error + Send + Sync + 'static> IntoBoxed<T> for std::result::Result<T, E> {
    ///　Converts Err type of Result to dynamic trait object.
    fn into_boxed(self) -> std::result::Result<T, Box<dyn std::error::Error + Send + Sync>> {
        self.map_err(Into::into)
    }
}

#[doc(hidden("or you will be fired"))]
impl<T> SecretTraitDoNotUseOrYouWillBeFired<T, Box<dyn std::error::Error + Send + Sync>>
    for std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>
{
    #[doc(hidden("or you will be fired"))]
    fn into_ghost<F>(
        self,
        hook: F,
    ) -> std::result::Result<
        T,
        SecretStructDoNotUseOrYouWillBeFired<Box<dyn std::error::Error + Send + Sync>, F>,
    >
    where
        Self: SameAs<std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>>,
        F: FnOnce(&Box<dyn std::error::Error + Send + Sync>),
    {
        self.map_err(|e| SecretStructDoNotUseOrYouWillBeFired(e, hook))
    }
}

#[doc(hidden("or you will be fired"))]
impl<T> SecretTraitDoNotUseOrYouWillBeFired<T, anyhow::Error> for anyhow::Result<T> {
    #[doc(hidden)]
    fn into_ghost<F>(
        self,
        hook: F,
    ) -> std::result::Result<T, SecretStructDoNotUseOrYouWillBeFired<anyhow::Error, F>>
    where
        Self: SameAs<std::result::Result<T, anyhow::Error>>,
        F: FnOnce(&anyhow::Error),
    {
        self.map_err(|e| SecretStructDoNotUseOrYouWillBeFired(e, hook))
    }
}

#[cfg(test)]
#[cfg(feature = "attribute")]
mod tests {
    use crate as error_hook;
    use error_hook::hook;
    use std::io::ErrorKind;

    #[test]
    fn anyhow_works() {
        #[hook(_ => println!("error"))]
        fn func(res: anyhow::Result<()>) -> crate::Result<()> {
            res
        }

        assert!(func(Ok(())).is_ok());
        assert!(func(Err(anyhow::anyhow!("error"))).is_err());
    }

    #[test]
    fn boxed_works() {
        #[hook(_ => println!("error"))]
        fn func(res: Result<(), Box<dyn std::error::Error + Send + Sync>>) -> crate::Result<()> {
            res
        }

        assert!(func(Ok(())).is_ok());
        assert!(func(Err(Box::new(std::io::Error::new(
            ErrorKind::NotFound,
            "ho, no"
        ))))
        .is_err());
    }
}
