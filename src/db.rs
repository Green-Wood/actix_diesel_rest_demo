use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use crate::model::{NewPost, NewUser, Post, User};
use crate::schema::{posts, users};
use diesel::r2d2::{ConnectionManager, Pool};

pub type DBPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_db_pool() -> DBPool {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("Fail to load env variable");
    let manager = ConnectionManager::new(db_url);

    Pool::builder().build(manager).expect("Fail to build pool")
}

pub fn db_all_users(conn: &PgConnection) -> Vec<User> {
    users::table.load(conn).expect("load all users failed")
}

pub fn db_insert_user(conn: &PgConnection, new_user: &NewUser) -> User {
    diesel::insert_into(users::table)
        .values(new_user)
        .get_result(conn)
        .expect("Fail to save user")
}

pub fn db_all_posts(conn: &PgConnection) -> Vec<Post> {
    posts::table.load(conn).expect("load all posts failed")
}

pub fn db_insert_post(conn: &PgConnection, new_post: &NewPost) -> Post {
    diesel::insert_into(posts::table)
        .values(new_post)
        .get_result(conn)
        .expect("Fail to save post")
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::result::Error;

    fn get_db_connection() -> PgConnection {
        dotenv().ok();

        let db_url = env::var("DATABASE_URL").expect("Fail to load env variable");
        PgConnection::establish(&db_url)
            .expect(&format!("Error connecting to {}", db_url))
    }

    #[test]
    fn test_user() {
        let conn = get_db_connection();
        conn.test_transaction::<_, Error, _>(|| {
            let user1 = NewUser {
                name: "green".to_string(),
                age: 10
            };
            let user2 = NewUser {
                name: "wood".to_string(),
                age: 12
            };

            db_insert_user(&conn, &user1);
            db_insert_user(&conn, &user2);

            // select all names from table ordered by user's age desc
            let all_names = users::table
                .select((users::name, users::age))
                .order(users::age.desc())
                .load::<(String, i32)>(&conn)?;
            assert_eq!(all_names, vec![("wood".to_string(), 12), ("green".to_string(), 10)]);

            Ok(())
        });

        // Even though we returned `Ok`, the transaction wasn't committed.
        let all_users = users::table.load::<User>(&conn).unwrap();
        assert!(all_users.is_empty());
    }

    #[test]
    fn test_post() {
        let conn = get_db_connection();
        conn.test_transaction::<_, Error, _>(|| {
            let user1 = NewUser {
                name: "green".to_string(),
                age: 10
            };
            let user2 = NewUser {
                name: "wood".to_string(),
                age: 12
            };
            let user1 = db_insert_user(&conn, &user1);
            let user2 = db_insert_user(&conn, &user2);

            let post1 = NewPost {
                title: "SARS".to_string(),
                body: "Today".to_string(),
                published: None,
                user_id: user1.id
            };
            let post2 = NewPost {
                title: "MERS".to_string(),
                body: "Yesterday".to_string(),
                published: Some(true),
                user_id: user1.id
            };
            let post3 = NewPost {
                title: "Health".to_string(),
                body: "Tomorrow".to_string(),
                published: None,
                user_id: user2.id
            };

            db_insert_post(&conn, &post1);
            db_insert_post(&conn, &post2);
            db_insert_post(&conn, &post3);

            // select all titles from table ordered by post's title
            let all_titles = posts::table
                .select(posts::title)
                .order(posts::title)
                .load::<String>(&conn)?;
            assert_eq!(all_titles, vec!["Health", "MERS", "SARS"]);

            // select post where user_id = user1's id
            let bodys_by_user_id = posts::table
                .select(posts::body)
                .filter(posts::user_id.eq(user1.id))
                .order(posts::title)
                .load::<String>(&conn)?;
            assert_eq!(bodys_by_user_id, vec!["Yesterday", "Today"]);

            // select all posts' published attribute to verify Option type's correctness
            let all_pub = posts::table
                .select(posts::published)
                .order(posts::title)
                .load::<bool>(&conn)?;
            assert_eq!(all_pub, vec![false, true, false]);

            Ok(())
        });
    }
}
