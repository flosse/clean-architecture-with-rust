use cawr_domain::thought::Title;
use thiserror::Error;

#[derive(Debug)]
pub struct Request<'a> {
    pub title: &'a str,
}
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

pub fn validate_thought_properties(req: &Request) -> Response {
    log::debug!("Validate thought properties {:?}", req);
    validate_title(req.title).map_err(ThoughtInvalidity::Title)?;
    Ok(())
}

fn validate_title(title: &str) -> Result<(), TitleInvalidity> {
    let actual = title.len();
    let min = Title::min_len();

    if actual < min {
        return Err(TitleInvalidity::MinLength { min, actual });
    }
    let max = Title::max_len();
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
            let res = validate_title("");
            assert!(matches!(
                res.err().unwrap(),
                TitleInvalidity::MinLength { min: 3, actual: 0 }
            ));

            let title = ["a"; 3].join("");
            assert!(validate_title(&title).is_ok());
        }

        #[test]
        fn should_have_max_80_chars() {
            let title = ["a"; 81].join("");
            let res = validate_title(&title);
            assert!(matches!(
                res.err().unwrap(),
                TitleInvalidity::MaxLength {
                    max: 80,
                    actual: 81
                }
            ));

            let title = ["a"; 80].join("");
            assert!(validate_title(&title).is_ok());
        }
    }
}
