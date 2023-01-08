use std::{
    fmt,
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
    str::FromStr,
};

use inquire::{
    validator::{CustomTypeValidator, ErrorMessage, Validation},
    CustomUserError,
};

macro_rules! impl_path {
    ($PathIdent:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Default)]
        pub struct $PathIdent(PathBuf);

        impl fmt::Display for $PathIdent {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0.display())
            }
        }

        impl FromStr for $PathIdent {
            type Err = anyhow::Error;

            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Self(PathBuf::from_str(
                    s.trim_matches(|ch| ch == '"' || ch == '\'').trim(),
                )?))
            }
        }

        impl Deref for $PathIdent {
            type Target = PathBuf;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl AsRef<Path> for $PathIdent {
            #[inline]
            fn as_ref(&self) -> &Path {
                self.0.as_path()
            }
        }

        impl DerefMut for $PathIdent {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

impl_path!(InputPath);
impl_path!(OutputPath);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PathValidator;

impl CustomTypeValidator<InputPath> for PathValidator {
    #[inline]
    fn validate(&self, input: &InputPath) -> Result<Validation, CustomUserError> {
        if input.0.is_file() {
            return Ok(Validation::Valid);
        }

        Ok(Validation::Invalid(ErrorMessage::Custom(
            "Provided path is not a file".to_owned(),
        )))
    }
}

impl CustomTypeValidator<OutputPath> for PathValidator {
    #[inline]
    fn validate(&self, input: &OutputPath) -> Result<Validation, CustomUserError> {
        if !input.0.exists() || input.0.is_file() {
            return Ok(Validation::Valid);
        }

        Ok(Validation::Invalid(ErrorMessage::Custom(
            "Provided path exists and is not a file".to_owned(),
        )))
    }
}
