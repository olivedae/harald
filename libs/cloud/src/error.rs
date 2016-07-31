use std::{fmt, error};

#[derive(Debug)]
pub enum ServerError {
    UnableToSavePost,
    UnspecifiedDatabaseUrl,
    UnableToConnectWithDatabase(String),
    UnableToLoadPosts,
    UnableToDeletePosts,
    UnableToPublishPost(i32),
    UnableToGetPost(i32)
}

#[allow(unused_variables)]
impl ServerError {
    fn desc(&self) -> &str {
        match *self {
            ServerError::UnableToSavePost => "Error saving post",
            ServerError::UnspecifiedDatabaseUrl => "Database URL is unspecified",
            ServerError::UnableToConnectWithDatabase(ref url) => "Error connecting to database",
            ServerError::UnableToLoadPosts => "Unable to load posts",
            ServerError::UnableToDeletePosts => "Unable to delete posts",
            ServerError::UnableToPublishPost(ref id) => "Unable to publish post",
            ServerError::UnableToGetPost(ref id) => "Unable to get post"
        }
    }
}

impl error::Error for ServerError {
    fn description(&self) -> &str {
        self.desc()
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.desc().fmt(formatter)
    }
}
