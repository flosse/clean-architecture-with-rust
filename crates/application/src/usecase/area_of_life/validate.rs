use cawr_domain::area_of_life::Name;
use thiserror::Error;

#[derive(Debug)]
pub struct Request<'a> {
    pub name: &'a str,
}
pub type Response = Result<(), AreaOfLifeInvalidity>;

#[derive(Debug, Error)]
pub enum AreaOfLifeInvalidity {
    #[error(transparent)]
    Name(#[from] NameInvalidity),
}

#[derive(Debug, Error)]
pub enum NameInvalidity {
    #[error("The name must have at least {min} but has {actual} chars")]
    MinLength { min: usize, actual: usize },
    #[error("The name must have at most {max} but has {actual} chars")]
    MaxLength { max: usize, actual: usize },
}

pub fn validate_area_of_life_properties(req: &Request) -> Response {
    log::debug!("Validate area of life properties {:?}", req);
    validate_name(req.name).map_err(AreaOfLifeInvalidity::Name)?;
    Ok(())
}

const fn validate_name(name: &str) -> Result<(), NameInvalidity> {
    let actual = name.len();
    let min = Name::min_len();

    if actual < min {
        return Err(NameInvalidity::MinLength { min, actual });
    }
    let max = Name::max_len();
    if actual > max {
        return Err(NameInvalidity::MaxLength { max, actual });
    }
    Ok(())
}
