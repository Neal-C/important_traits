// #![allow(dead_code)]

use std::sync::Arc;
#[cfg(feature="serde")]
use serde::{Deserialize, Serialize};


#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature="serde", derive(Deserialize, Serialize,))]
pub enum Role {
    Admin,
    Standard,
    #[default]
    Guest,
    Memer,
}

#[derive(Debug, Clone, Default, PartialEq)]
struct DB{}

//Send & Sync traits are auto-implemented as long as the full type is thread-safe
#[derive(Debug,Clone, Default, PartialEq)]
#[cfg_attr(feature="serde", derive(Deserialize, Serialize))]
pub struct User {
    id: u32,
    name: String,
    role: Role,
    #[cfg_attr(feature = "serde", serde(skip))]
    db: Arc<DB>
}

fn main() {
    let user = User {
        id: 123,
        name: String::from("Heeeh"),
        role: Role::Memer,
        db: Arc::new(DB{}),
    };

    let user2 = user.clone();

    let guest = User::default();

    let guest2 = User::default();

    assert_eq!(guest, guest2);

    println!("{:?}", user);
    println!("{:?}", guest);
}

fn is_normal<T: Sized + Send + Sync + Unpin>(){}

#[test]
fn normal_types(){
    is_normal::<User>();
}