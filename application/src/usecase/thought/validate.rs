use domain::thought::{Thought, Title, TitleConstraints};
use thiserror::Error;

pub type Request = Thought;
pub type Response = Result<(), ThoughtInvalidity>;

#[derive(Debug, Error)]
pub enum ThoughtInvalidity {
    #[error(transparent)]
    Title(#[from] TitleInvalidity),
}

#[derive(Debug, Error)]
pub enum TitleInvalidity {
    #[error("The title must have at least {min} but has {actual} chars")]
    MinLength { min: usize, actual: usize },
    #[error("The title must have at most {max} but has {actual} chars")]
    MaxLength { max: usize, actual: usize },
}

pub fn validate_thought(thought: &Request) -> Response {
    log::debug!("Validate thought {:?}", thought);
    validate_title(thought.title()).map_err(ThoughtInvalidity::Title)?;
    Ok(())
}

fn validate_title(title: &Title) -> Result<(), TitleInvalidity> {
    let actual = title.as_ref().len();
    let min = TitleConstraints::min_len();

    if actual < min {
        return Err(TitleInvalidity::MinLength { min, actual });
    }
    let max = TitleConstraints::max_len();
    if actual > max {
        return Err(TitleInvalidity::MaxLength { max, actual });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod the_title {
        use super::*;

        #[test]
        fn should_have_min_3_chars() {
            let title = Title::new("".to_string());
            let res = validate_title(&title);
            assert!(matches!(
                res.err().unwrap(),
                TitleInvalidity::MinLength { min: 3, actual: 0 }
            ));

            let title = Title::new(["a"; 3].join(""));
            assert!(validate_title(&title).is_ok());
        }

        #[test]
        fn should_have_max_80_chars() {
            let title = Title::new(["a"; 81].join(""));
            let res = validate_title(&title);
            assert!(matches!(
                res.err().unwrap(),
                TitleInvalidity::MaxLength {
                    max: 80,
                    actual: 81
                }
            ));

            let title = Title::new(["a"; 80].join(""));
            assert!(validate_title(&title).is_ok());
        }
    }
}
