mod common;

use common::*;
use reqwest::{Client, StatusCode};

use notegraf::NoteLocator;
use serde_json::{json, Value};

#[tokio::test]
async fn new_note() {
    let app = spawn_app().await;
    let client = Client::new();

    let response = client
        // Use the returned application address
        .post(&format!("{}/api/v1/note", &app.address))
        .json(&json!({
            "title": "My title",
            "note_inner": "# Hey Markdown Note\n## H2",
            "metadata_tags": "",
            "metadata_custom_metadata": "null"
        }))
        .send()
        .await
        .expect("Failed to execute request.")
        .json::<Value>()
        .await
        .expect("Failed to parse response");
    println!("{}", response);
    assert!(response.is_object());
    let loc = response.as_object().unwrap();
    assert!(loc.contains_key("Specific"));
}

async fn create_note_helper(
    client: &Client,
    address: &str,
    title: &str,
    note_inner: &str,
) -> NoteLocator {
    client
        .post(&format!("{}/api/v1/note", address))
        .json(&json!({
            "title": title.to_owned(),
            "note_inner": note_inner.to_owned(),
            "metadata_tags": "",
            "metadata_custom_metadata": "null"
        }))
        .send()
        .await
        .expect("Failed to execute request.")
        .json()
        .await
        .expect("Failed to parse response")
}

async fn update_note_helper(
    client: &Client,
    address: &str,
    loc: &NoteLocator,
    title: &str,
    note_inner: &str,
) -> NoteLocator {
    client
        .post(&format!(
            "{}/api/v1/note/{}/revision",
            address,
            loc.get_id()
        ))
        .json(&json!({
            "title": title.to_owned(),
            "note_inner": note_inner.to_owned(),
            "metadata_tags": "",
            "metadata_custom_metadata": "null"
        }))
        .send()
        .await
        .expect("Failed to execute request.")
        .json()
        .await
        .expect("Failed to parse response")
}

#[tokio::test]
async fn note_retrive() {
    let app = spawn_app().await;
    let client = Client::new();

    let loc1 = create_note_helper(&client, &app.address, "title", "## body text").await;
    let response = client
        .get(&format!(
            "{}/api/v1/note/{}",
            &app.address,
            loc1.get_id().as_ref()
        ))
        .send()
        .await
        .expect("Failed to execute request.")
        .json::<Value>()
        .await
        .expect("Failed to parse response");

    assert!(response.is_object());
    assert_eq!(response["id"], loc1.get_id().as_ref());
    assert_eq!(response["revision"], loc1.get_revision().unwrap().as_ref());
    assert_eq!(response["next"], Value::Null);
    assert_eq!(response["title"], "title");
    assert_eq!(response["note_inner"], "## body text");
}

#[tokio::test]
async fn note_delete() {
    let app = spawn_app().await;
    let client = Client::new();

    let loc1 = create_note_helper(&client, &app.address, "title", "## body text").await;
    client
        .get(&format!(
            "{}/api/v1/note/{}",
            &app.address,
            loc1.get_id().as_ref()
        ))
        .send()
        .await
        .expect("Failed to execute request.")
        .json::<Value>()
        .await
        .expect("Failed to parse response");
    client
        .delete(&format!(
            "{}/api/v1/note/{}",
            &app.address,
            loc1.get_id().as_ref()
        ))
        .send()
        .await
        .expect("Failed to execute request.");
    let response = client
        .get(&format!(
            "{}/api/v1/note/{}",
            &app.address,
            loc1.get_id().as_ref()
        ))
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn note_update() {
    let app = spawn_app().await;
    let client = Client::new();

    let loc1 = create_note_helper(&client, &app.address, "title", "## body text").await;
    let loc2 = update_note_helper(&client, &app.address, &loc1, "New title", "New body text").await;

    let response = client
        .get(&format!(
            "{}/api/v1/note/{}",
            &app.address,
            loc1.get_id().as_ref()
        ))
        .send()
        .await
        .expect("Failed to execute request.")
        .json::<Value>()
        .await
        .expect("Failed to parse response");
    assert_eq!(response["id"], loc1.get_id().as_ref());
    assert_eq!(response["revision"], loc2.get_revision().unwrap().as_ref());
    assert_eq!(response["next"], Value::Null);
    assert_eq!(response["title"], "New title");
    assert_eq!(response["note_inner"], "New body text");
}
