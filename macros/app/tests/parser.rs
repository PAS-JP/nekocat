#![cfg(feature = "parser")]
use neko_cat_macros::Parser;
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Parser, Archive, Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[rkyv(compare(PartialEq), derive(Debug))]
struct User {
    name: String,
    friend: Friend,
    age: u8,
    empty: String,
    bool: bool,
}

#[derive(Parser, Archive, Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct Friend {
    name: String,
}

#[test]
fn parser_hash_map() {
    let user = User {
        age: 18,
        friend: Friend {
            name: "John".to_string(),
        },
        name: "Abby".to_string(),
        empty: "".to_string(),
        bool: false,
    };

    let hash = user.clone().to_hash_map();

    let name = hash.get(&UserParserKey::UserName).unwrap();
    let age = hash.get(&UserParserKey::UserAge).unwrap();
    let empty = hash.get(&UserParserKey::UserEmpty).unwrap();
    let friend = hash.get(&UserParserKey::UserFriend).unwrap();

    assert_eq!(name, &UserParserValue::UserName(String::from("Abby")));
    assert_eq!(age, &UserParserValue::UserAge(18));
    assert_eq!(empty, &UserParserValue::UserEmpty("".to_string()));
    assert_eq!(
        friend,
        &UserParserValue::UserFriend(Friend {
            name: "John".to_string()
        })
    );

    let parsed = User::from_hash_map(hash).expect("failed to parse from hash map");

    assert_eq!(parsed.age, 18);
    assert_eq!(parsed.name, "Abby");
    assert_eq!(parsed.empty, "");
    assert_eq!(
        parsed.friend,
        Friend {
            name: "John".to_string()
        }
    );

    let hash = user.to_clean_hash_map();

    let name = hash.get(&UserParserKey::UserName).unwrap();
    let age = hash.get(&UserParserKey::UserAge).unwrap();
    let friend = hash.get(&UserParserKey::UserFriend).unwrap();

    assert!(!hash.contains_key(&UserParserKey::UserEmpty));
    assert!(!hash.contains_key(&UserParserKey::UserBool));
    assert_eq!(name, &UserParserValue::UserName(String::from("Abby")));
    assert_eq!(age, &UserParserValue::UserAge(18));
    assert_eq!(
        friend,
        &UserParserValue::UserFriend(Friend {
            name: "John".to_string()
        })
    );
}

#[test]
fn parser_hash_set() {
    let user = User {
        age: 18,
        friend: Friend {
            name: "John".to_string(),
        },
        name: "Abby".to_string(),
        empty: "".to_string(),
        bool: false,
    };

    let user = user.to_hash_set();

    let name = user
        .get(&UserParserValue::UserName(String::from("Abby")))
        .unwrap();
    let age = user.get(&UserParserValue::UserAge(18)).unwrap();
    let friend = user
        .get(&UserParserValue::UserFriend(Friend {
            name: "John".to_string(),
        }))
        .unwrap();

    assert_eq!(name, &UserParserValue::UserName(String::from("Abby")));
    assert_eq!(age, &UserParserValue::UserAge(18));
    assert_eq!(
        friend,
        &UserParserValue::UserFriend(Friend {
            name: "John".to_string()
        })
    );
}

#[test]
fn parser_to_bytes() {
    let user = User {
        age: 18,
        friend: Friend {
            name: "John".to_string(),
        },
        name: "Abby".to_string(),
        empty: "".to_string(),
        bool: false,
    };

    let bytes = user.clone().to_bytes().unwrap();
    let decoded = User::from_bytes(bytes).unwrap();

    assert_eq!(user, decoded)
}
