#![cfg(feature = "search")]
use neko_cat_macros::Search;

const EPS: f32 = 1e-6;

#[derive(Search)]
struct User {
    name: String,
    nick: &'static str,
}

#[test]
fn exact_match_leads_to_one() {
    let u = User {
        name: "alice".to_string(),
        nick: "alice",
    };

    let lev = u.levenshtein_name("alice");
    assert!(
        (lev - 1.0).abs() < EPS,
        "lev exact should be 1.0, got {}",
        lev
    );

    let cos = u.search_cosine_name("alice", 2);
    assert!(
        (cos - 1.0).abs() < EPS,
        "cosine exact should be 1.0, got {}",
        cos
    );

    let jac = u.jaccard_name("alice", 2);
    assert!(
        (jac - 1.0).abs() < EPS,
        "jaccard exact should be 1.0, got {}",
        jac
    );

    let j = u.jaro_name("alice", 4);
    assert!((j - 1.0).abs() < EPS, "jaro exact should be 1.0, got {}", j);
}

#[test]
fn totally_different_strings_are_near_zero() {
    let u = User {
        name: "aaaaaaaa".to_string(),
        nick: "bbbbbbbb",
    };

    let lev = u.levenshtein_name("zzzzzzzz");
    assert!(
        lev <= 0.1,
        "expected low levenshtein similarity, got {}",
        lev
    );

    let cos = u.search_cosine_name("zzzzzzzz", 2);
    assert!(cos <= 0.1, "expected low cosine similarity, got {}", cos);

    let jac = u.jaccard_name("zzzzzzzz", 2);
    assert!(jac <= 0.1, "expected low jaccard similarity, got {}", jac);

    let j = u.jaro_name("zzzzzzzz", 3);
    assert!(j <= 0.2, "expected low jaro similarity, got {}", j);
}

#[test]
fn similar_but_not_equal_strings_produce_intermediate_scores() {
    let u = User {
        name: "jonathan".to_string(),
        nick: "jon",
    };

    let lev = u.levenshtein_name("jon");
    assert!(
        (lev > 0.2) && (lev < 0.9),
        "expected intermediate levenshtein, got {}",
        lev
    );
    let cos = u.search_cosine_name("jon", 2);
    assert!(
        cos > 0.3,
        "expected cosine>0.3 for related strings, got {}",
        cos
    );
    let jac = u.jaccard_name("jon", 2);
    assert!(jac > 0.0 && jac < 1.0, "expected 0<jaccard<1, got {}", jac);
    let j = u.jaro_name("jon", 3);
    assert!(j > 0.6, "expected jaro>0.6 for shared prefix, got {}", j);
}

#[test]
fn methods_work_on_both_string_and_str_fields() {
    let u = User {
        name: "maria".to_string(),
        nick: "maria",
    };

    let cos_name = u.search_cosine_name("maria", 2);
    let cos_nick = u.search_cosine_nick("maria", 2);

    assert!((cos_name - 1.0).abs() < EPS, "cosine on name exact failed");
    assert!((cos_nick - 1.0).abs() < EPS, "cosine on nick exact failed");

    let lev_nick = u.levenshtein_nick("maria");
    assert!(
        (lev_nick - 1.0).abs() < EPS,
        "levenshtein on nick exact failed"
    );
}
