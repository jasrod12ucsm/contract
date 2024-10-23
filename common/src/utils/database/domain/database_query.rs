use crate::utils::database::infrastructure::database_library::{FindQuery, UpdateQuery};

pub trait DatabaseQueryTrait {
    fn update() -> UpdateQuery;
    fn find()-> FindQuery;
}