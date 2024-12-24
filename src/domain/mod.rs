pub mod transaction;

use crate::repository::Repository;

pub struct Domain {
    repository: Repository,
}

impl Domain {
    pub fn new(repository: Repository) -> Self {
        Self{repository}
    }
}
