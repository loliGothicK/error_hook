#[cfg(feature = "attribute")]
use error_hook_attr::hook;

use thiserror::Error;

#[derive(Error, Debug)]
#[error(transparent)]
pub enum Error {
    Boxed(Box<dyn std::error::Error>),
    Anyhow(#[from] anyhow::Error),
}

struct Ghost<E, F>(E, F);

impl<F> From<Ghost<Box<dyn std::error::Error>, F>> for Error
where
    F: FnOnce(&Box<dyn std::error::Error>),
{
    fn from(source: Ghost<Box<dyn std::error::Error>, F>) -> Self {
        let Ghost(src, hook) = source;
        hook(&src);
        Self::Boxed(src)
    }
}

impl<F> From<Ghost<anyhow::Error, F>> for Error
where
    F: FnOnce(&anyhow::Error),
{
    fn from(source: Ghost<anyhow::Error, F>) -> Self {
        let Ghost(src, hook) = source;
        hook(&src);
        Self::Anyhow(src)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

trait SameAs<T> {}
impl<T> SameAs<T> for T {}

trait ResultExt<T, E> {
    fn into_ghost<F>(self, hook: F) -> std::result::Result<T, Ghost<E, F>>
    where
        Self: SameAs<std::result::Result<T, E>>,
        F: FnOnce(&E);
}

trait IntoBoxed<T> {
    fn into_boxed(self) -> std::result::Result<T, Box<dyn std::error::Error>>;
}

impl<T, E: std::error::Error + 'static> IntoBoxed<T> for std::result::Result<T, E> {
    fn into_boxed(self) -> std::result::Result<T, Box<dyn std::error::Error>> {
        self.map_err(Into::into)
    }
}

impl<T> ResultExt<T, Box<dyn std::error::Error>>
    for std::result::Result<T, Box<dyn std::error::Error>>
{
    fn into_ghost<F>(self, hook: F) -> std::result::Result<T, Ghost<Box<dyn std::error::Error>, F>>
    where
        Self: SameAs<std::result::Result<T, Box<dyn std::error::Error>>>,
        F: FnOnce(&Box<dyn std::error::Error>),
    {
        self.map_err(|e| Ghost(e, hook))
    }
}

impl<T> ResultExt<T, anyhow::Error> for anyhow::Result<T> {
    fn into_ghost<F>(self, hook: F) -> std::result::Result<T, Ghost<anyhow::Error, F>>
    where
        Self: SameAs<std::result::Result<T, anyhow::Error>>,
        F: FnOnce(&anyhow::Error),
    {
        self.map_err(|e| Ghost(e, hook))
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
        fn func(res: Result<(), Box<dyn std::error::Error>>) -> crate::Result<()> {
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
