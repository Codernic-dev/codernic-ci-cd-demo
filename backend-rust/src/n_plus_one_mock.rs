// Mock file to demonstrate an N+1 Query problem for Galileus CI to catch.

struct User {
    id: i32,
    name: String,
}

struct Post {
    id: i32,
    user_id: i32,
    title: String,
}

/// Fetches all users and then fetches their posts one by one.
/// This creates a massive N+1 query problem that static linters miss
/// but Galileus CI will immediately flag as a performance disaster.
pub fn get_users_with_posts_bad() {
    // Query 1: Get all users
    let users = db_query_users("SELECT * FROM users");

    // N Queries: Get posts for EACH user in a loop
    for user in users {
        // ERROR: Calling a DB query inside a loop!
        let posts = db_query_posts(&format!("SELECT * FROM posts WHERE user_id = {}", user.id));
        
        println!("User {} has {} posts", user.name, posts.len());
    }
}

// Dummy functions to make the file compile in a real project
fn db_query_users(_query: &str) -> Vec<User> { vec![User { id: 1, name: "Alice".to_string() }] }
fn db_query_posts(_query: &str) -> Vec<Post> { vec![] }
