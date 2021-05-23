use entity::item::{Item, Title};
use thiserror::Error;

const MAX_TITLE_LEN: usize = 80;
const MIN_TITLE_LEN: usize = 3;

#[derive(Debug, Error)]
pub enum ItemInvalidity {
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

pub fn validate_item(item: &Item) -> Result<(), ItemInvalidity> {
    validate_title(&item.title).map_err(ItemInvalidity::Title)?;
    Ok(())
}

fn validate_title(title: &Title) -> Result<(), TitleInvalidity> {
    let actual = title.0.len();
    if actual < MIN_TITLE_LEN {
        return Err(TitleInvalidity::MinLength {
            min: MIN_TITLE_LEN,
            actual,
        });
    }
    if actual > MAX_TITLE_LEN {
        return Err(TitleInvalidity::MaxLength {
            max: MAX_TITLE_LEN,
            actual,
        });
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
            let title = Title("".to_string());
            let res = validate_title(&title);
            assert!(matches!(
                res.err().unwrap(),
                TitleInvalidity::MinLength { min: 3, actual: 0 }
            ));

            let title = Title(["a"; 3].join(""));
            assert!(validate_title(&title).is_ok());
        }

        #[test]
        fn should_have_max_80_chars() {
            let title = Title(["a"; 81].join(""));
            let res = validate_title(&title);
            assert!(matches!(
                res.err().unwrap(),
                TitleInvalidity::MaxLength {
                    max: 80,
                    actual: 81
                }
            ));

            let title = Title(["a"; 80].join(""));
            assert!(validate_title(&title).is_ok());
        }
    }
}
