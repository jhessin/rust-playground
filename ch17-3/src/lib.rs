use blog::*;

#[allow(dead_code)]
#[allow(unused_variables)]
fn setup() -> DraftPost {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    post
}

#[test]
fn draft_can_be_submitted_for_review() {
    let post = setup();
    post.request_review();
}

#[test]
fn review_post_can_be_approved() {
    let post = setup().request_review();
    post.approve();
}

#[test]
fn approved_post_has_matching_content() {
    let post = setup();
    let post = post.request_review();
    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
