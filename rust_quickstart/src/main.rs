use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use futures::TryStreamExt;
use mongodb::{bson::doc, bson::oid::ObjectId, Client, Collection};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    name: Option<String>,
    email: Option<String>,
    password: Option<String>,
    role: Option<String>,
    number: Option<i64>,
}

/// Main function to run the server and define the routes
#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let uri = "mongodb+srv://thumarkeval12:eavj9VqvUdQ6JBJa@rustdata.yjvdx.mongodb.net/?retryWrites=true&w=majority&appName=RustData";
    let client = Client::with_uri_str(uri).await?;

    let app = Router::new()
        .route("/", get(root)) // Root endpoint
        .route("/get-user/{id}", {
            let client = client.clone();
            get(move |id| get_user(id, client.clone()))
        })
        .route("/get-all-users", {
            let client = client.clone();
            get(move || get_all_users(client.clone()))
        })
        .route("/add-user", {
            let client = client.clone();
            post(move |body| add_user(body, client))
        })
        .route("/update-user/{id}",{
            let client = client.clone();
            post(move |id, body| update_user(id, body, client))
        })
        .route("/delete-user/{id}", {
            let client = client.clone();
            get(move |id| delete_user(id, client))
        }); // Update user endpoint

    // Run the server
    // Define the address to serve
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on {}", listener.local_addr().unwrap());

    // Start the server
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

/// Root endpoint to return a simple message
async fn root() -> &'static str {
    "Hello, World!"
}

/// Function to add a user and return the result as an HTTP response
async fn add_user(
    Json(user): Json<User>, // Extract user data from the request body
    client: Client,
) -> impl IntoResponse {
    let my_coll: Collection<User> = client.database("sample_restaurants").collection("users");

    match my_coll.insert_one(user).await {
        Ok(res) => {
            // Return inserted ID as a success response
            let response = doc! {
                "inserted_id": res.inserted_id,
                "message": "User inserted successfully"
            };
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(err) => {
            // Handle errors gracefully
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(doc! { "error": format!("Failed to insert user: {}", err) }),
            )
                .into_response()
        }
    }
}

/// Function to get user from the database
async fn get_user(
    Path(id): Path<String>, // Extract `id` from the URL path
    client: Client,
) -> impl IntoResponse {
    let my_coll: Collection<User> = client.database("sample_restaurants").collection("users");

    match ObjectId::parse_str(&id) {
        Ok(object_id) => match my_coll.find_one(doc! { "_id": object_id }).await {
            Ok(Some(document)) => Json(document).into_response(),
            Ok(None) => (StatusCode::NOT_FOUND, "User not found").into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(doc! { "error": format!("Failed to fetch user: {}", err) }),
            )
                .into_response(),
        },
        Err(_) => (
            StatusCode::BAD_REQUEST,
            Json(doc! { "error": "Invalid ID format" }),
        )
            .into_response(),
    }
}

/// Function to get all users from the database
async fn get_all_users(client: Client) -> impl IntoResponse {
    let my_coll: Collection<User> = client.database("sample_restaurants").collection("users");

    match my_coll.find(doc! {}).await {
        Ok(cursor) => {
            let users: Vec<User> = cursor.try_collect().await.unwrap_or_default();
            Json(users).into_response()
        }
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(doc! { "error": format!("Failed to fetch users: {}", err) }),
        )
            .into_response(),
    }
}

async fn update_user(
    Path(id): Path<String>,        // Extract `id` from the URL path
    Json(update_data): Json<User>, // Extract partial update data
    client: Client,                // MongoDB client
) -> impl IntoResponse {
    let collection: Collection<User> = client.database("sample_restaurants").collection("users");

    // Parse the `id` into an `ObjectId`
    match ObjectId::parse_str(&id) {
        Ok(object_id) => {
            // Create a filter to locate the document to update
            let filter = doc! { "_id": object_id };

            // Dynamically build the `$set` update document
            let mut update_doc = doc! {};
            if let Some(name) = update_data.name {
                update_doc.insert("name", name);
            }
            if let Some(email) = update_data.email {
                update_doc.insert("email", email);
            }
            if let Some(password) = update_data.password {
                update_doc.insert("password", password);
            }
            if let Some(role) = update_data.role {
                update_doc.insert("role", role);
            }
            if let Some(number) = update_data.number {
                update_doc.insert("number", number);
            }

            // Ensure there is something to update
            if update_doc.is_empty() {
                return (StatusCode::BAD_REQUEST, "No fields provided for update").into_response();
            }

            let update = doc! { "$set": update_doc };

            // Perform the update
            match collection.update_one(filter, update).await {
                Ok(update_result) => {
                    if update_result.matched_count == 0 {
                        (StatusCode::NOT_FOUND, "User not found").into_response()
                    } else {
                        (StatusCode::OK, "User updated successfully").into_response()
                    }
                }
                Err(err) => {
                    eprintln!("Database error: {}", err);
                    (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update user").into_response()
                }
            }
        }
        Err(_) => (StatusCode::BAD_REQUEST, "Invalid user ID").into_response(),
    }
}

async fn delete_user(
    Path(id): Path<String>, // Extract `id` from the URL path
    client: Client,
) -> impl IntoResponse {
    let my_coll: Collection<User> = client.database("sample_restaurants").collection("users");

    match ObjectId::parse_str(&id) {
        Ok(object_id) => match my_coll.delete_one(doc! { "_id": object_id }).await {
            Ok(delete_result) => {
                if delete_result.deleted_count > 0 {
                    (StatusCode::OK, "User deleted successfully").into_response()
                } else {
                    (StatusCode::NOT_FOUND, "User not found").into_response()
                }
            }
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(doc! { "error": format!("Failed to delete user: {}", err) }),
            )
                .into_response(),
        },
        Err(_) => (StatusCode::BAD_REQUEST, "Invalid ID format").into_response(),
    }
}