use crate::user::User;
use std::ops::Deref;

pub trait Repository: Send + Sync + 'static {
    fn get_user(&self, user_id: &uuid::Uuid) -> Result<User, String>;
}

pub struct RepositoryInjector(Box<dyn Repository>);

impl RepositoryInjector {
    pub fn new(repo: impl Repository) -> Self {
        Self(Box::new(repo))
    }
}

impl Deref for RepositoryInjector {
    type Target = dyn Repository;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

pub struct MemoryRepository {
    users: Vec<User>,
}

impl Default for MemoryRepository {
    fn default() -> Self {
        Self {
            users: vec![User::new("Roman".to_string(), (1988, 9, 8))],
        }
    }
}

impl Repository for MemoryRepository {
    fn get_user(&self, user_id: &uuid::Uuid) -> Result<User, String> {
        self.users
            .iter()
            .find(|u| &u.id == user_id)
            // .map(|u| u.clone())
            .cloned()
            .ok_or_else(|| "Invalid UUID".to_string())
    }
}
