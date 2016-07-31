extern crate diesel;
extern crate rustc_serialize;

use super::schema::posts;
use super::error::ServerError;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;

#[derive(Queryable, Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool
}

#[insertable_into(posts)]
struct NewPost {
    title: String,
    body: String
}

pub type PostInformation = (String, String);

impl ToJson for Post {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();

        d.insert("id".to_string(), self.id.to_json());
        d.insert("title".to_string(), self.title.to_json());
        d.insert("body".to_string(), self.body.to_json());
        d.insert("published".to_string(), self.published.to_json());

        Json::Object(d)
    }
}

impl Post {
    pub fn create(conn: &PgConnection, post: PostInformation) -> Result<Post, ServerError> {
        let (title, body) = post;

        let new_post = NewPost {
            title: title,
            body: body
        };

        new_post.save(conn)
    }

    pub fn get_all(conn: &PgConnection) -> Result<Vec<Post>, ServerError> {
        use schema::posts::dsl::*;

        let filter = published.eq(true);

        let results =
            match posts.filter(filter).load::<Post>(conn) {
                Ok(p) => Ok(p),
                Err(_) => Err(ServerError::UnableToLoadPosts)
            };

        results
    }

    pub fn publish(conn: &PgConnection, post_id: i32, publish: bool) -> Result<Post, ServerError> {
        use schema::posts::dsl::{posts, published};

        let pattern = posts.find(post_id);
        let publish = published.eq(publish);

        let post =
            match diesel::update(pattern).set(publish).get_result::<Post>(conn) {
                Ok(p) => Ok(p),
                Err(_) => Err(
                    ServerError::UnableToPublishPost(post_id)
                )
            };

        post
    }

    pub fn delete(conn: &PgConnection, post_id: i32) -> Result<usize, ServerError> {
        use schema::posts::dsl::*;

        let post = posts.find(post_id);

        let deleted_count =
            match diesel::delete(post).execute(conn) {
                Ok(count) => Ok(count),
                Err(_) => Err(ServerError::UnableToDeletePosts)
            };

        deleted_count
    }

    pub fn get(conn: &PgConnection, post_id: i32) -> Result<Post, ServerError> {
        use schema::posts::dsl::*;

        let filter = id.eq(post_id);

        let post =
            match posts.filter(filter).load::<Post>(conn) {
                Ok(mut p) => Ok(p.pop().unwrap()),
                Err(_) => Err(
                    ServerError::UnableToGetPost(post_id)
                )
            };

        post
    }
}

impl NewPost {
    fn save(&self, conn: &PgConnection) -> Result<Post, ServerError> {
        use schema::posts;

        let saved_post =
            match diesel::insert(self).into(posts::table).get_result(conn) {
                Ok(post) => Ok(post),
                Err(_) => Err(ServerError::UnableToSavePost)
            };

        saved_post
    }
}
