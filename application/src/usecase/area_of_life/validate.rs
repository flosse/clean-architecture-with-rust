use domain::area_of_life::{AreaOfLife, Name, NameConstraints};
use thiserror::Error;

pub type Request = AreaOfLife;
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

pub fn validate_area_of_life(area_of_life: &Request) -> Response {
    log::debug!("Validate area of life {:?}", area_of_life);
    validate_name(&area_of_life.name).map_err(AreaOfLifeInvalidity::Name)?;
    Ok(())
}

fn validate_name(name: &Name) -> Result<(), NameInvalidity> {
    let actual = name.as_ref().len();
    let min = NameConstraints::min_len();

    if actual < min {
        return Err(NameInvalidity::MinLength { min, actual });
    }
    let max = NameConstraints::max_len();
    if actual > max {
        return Err(NameInvalidity::MaxLength { max, actual });
    }
    Ok(())
}
