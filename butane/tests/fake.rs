#![allow(unused_imports)]

use butane::db::Connection;
use butane::{find, DataObject, ForeignKey};

use butane_test_helper::*;

mod common;
#[allow(dead_code)]
use common::blog::{Blog, Post, Tag};

#[cfg(feature = "fake")]
use fake::{Fake, Faker};

#[cfg(feature = "fake")]
fn fake_blog_post(conn: Connection) {
    let mut fake_blog: Blog = Faker.fake();
    let blog_name = fake_blog.name.clone();
    eprintln!("name: {blog_name}");
    fake_blog.save(&conn).unwrap();

    let mut post: Post = Faker.fake();
    // To have a fully functional object, the foreign keys
    // need to be populated manually.
    post.blog = ForeignKey::from(fake_blog);

    let mut tag_1: Tag = Faker.fake();
    tag_1.save(&conn).unwrap();
    let mut tag_2: Tag = Faker.fake();
    tag_2.save(&conn).unwrap();
    let mut tag_3: Tag = Faker.fake();
    tag_3.save(&conn).unwrap();

    post.tags.add(&tag_1).unwrap();
    post.tags.add(&tag_2).unwrap();
    post.tags.add(&tag_3).unwrap();
    post.save(&conn).unwrap();

    let post_from_db = find!(Post, id == { post.id }, &conn).unwrap();
    assert_eq!(post_from_db.title, post.title);
    assert_eq!(post_from_db.tags.load(&conn).unwrap().count(), 3);

    assert_eq!(post_from_db.blog.load(&conn).unwrap().name, blog_name);
}
#[cfg(feature = "fake")]
testall!(fake_blog_post);

#[cfg(feature = "fake")]
/// Fake ForeignKey values can be accessed, but will not be saved
/// resulting in inability to load them from the database.
fn fake_blog_post_without_blog(conn: Connection) {
    let mut post: Post = Faker.fake();

    // The ForeignKey value can be accessed
    assert!(post.blog.get().is_ok());
    let blog_name = post.blog.get().unwrap().name.clone();
    assert!(post.blog.load(&conn).is_ok());
    assert_eq!(post.blog.load(&conn).unwrap().name, blog_name);

    eprintln!("blog pk {}",post.blog.pk());
    assert_ne!(post.blog.pk(), 0);
    panic!();
    post.save(&conn).unwrap();

    let post_from_db = find!(Post, id == { post.id }, &conn).unwrap();
    assert_eq!(post_from_db.title, post.title);
    // assert_eq!(post_from_db.tags.load(&conn).unwrap().count(), 3);

    // Loading the fake value fails
    assert!(post_from_db.blog.load(&conn).is_err());
}
#[cfg(feature = "fake")]
testall!(fake_blog_post_without_blog);
