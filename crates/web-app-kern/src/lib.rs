use cawr_web_app_api as api;

mod usecase;

pub mod domain;

use self::domain::{AreaOfLife, AreaOfLifeId, Thought, ThoughtId};

// ------ ------
//    Message
// ------ ------

type Error = String;
type Result<T> = std::result::Result<T, Error>;

#[derive(derive_more::From, Debug)]
pub enum Msg<V> {
    View(V),
    #[from]
    Usecase(UsecaseResult),
}

#[derive(Debug)]
pub enum UsecaseResult {
    CreateThought(Result<ThoughtId>),
    UpdateThought(ThoughtId, Result<()>),
    CreateAreaOfLife(Result<AreaOfLifeId>),
    UpdateAreaOfLife(AreaOfLifeId, Result<()>),
    FetchAllThoughts(Result<Vec<Thought>>),
    FetchAllAreasOfLife(Result<Vec<AreaOfLife>>),
    FindThought(Result<Thought>),
    DeleteThought(Result<ThoughtId>),
    DeleteAreaOfLife(Result<AreaOfLifeId>),
}

// -- Map usecases to messages -- //

pub async fn create_thought(title: String, area_of_life: Option<AreaOfLifeId>) -> UsecaseResult {
    let areas_of_life = area_of_life.map(|id| vec![id]).unwrap_or_default();
    let res = usecase::thought::create(title, areas_of_life).await;
    UsecaseResult::CreateThought(res)
}

pub async fn update_thought(thought: Thought) -> UsecaseResult {
    let id = thought.id;
    let res = usecase::thought::update(thought).await;
    UsecaseResult::UpdateThought(id, res)
}

pub async fn fetch_all_thoughts() -> UsecaseResult {
    let res = usecase::thought::fetch_all().await;
    UsecaseResult::FetchAllThoughts(res)
}

pub async fn find_thought_by_id(id: domain::ThoughtId) -> UsecaseResult {
    let res = usecase::thought::find_by_id(&id).await;
    UsecaseResult::FindThought(res)
}

pub async fn delete_thought(id: domain::ThoughtId) -> UsecaseResult {
    let res = usecase::thought::delete(&id).await;
    UsecaseResult::DeleteThought(res.map(|()| id))
}

pub async fn create_area_of_life(name: String) -> UsecaseResult {
    let res = usecase::area_of_life::create(name).await;
    UsecaseResult::CreateAreaOfLife(res)
}

pub async fn update_area_of_life(aol: AreaOfLife) -> UsecaseResult {
    let id = aol.id;
    let res = usecase::area_of_life::update(aol).await;
    UsecaseResult::UpdateAreaOfLife(id, res)
}

pub async fn fetch_all_areas_of_life() -> UsecaseResult {
    let res = usecase::area_of_life::fetch_all().await;
    UsecaseResult::FetchAllAreasOfLife(res)
}

pub async fn delete_area_of_life(id: domain::AreaOfLifeId) -> UsecaseResult {
    let res = usecase::area_of_life::delete(&id).await;
    UsecaseResult::DeleteAreaOfLife(res.map(|()| id))
}
