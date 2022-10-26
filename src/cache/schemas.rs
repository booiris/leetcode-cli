//! Leetcode data schemas
table! {
    problems(id) {
        category -> Text,
        fid -> Text,
        id -> Integer,
        level -> Integer,
        locked -> Bool,
        name -> Text,
        percent -> Float,
        slug -> Text,
        starred -> Bool,
        status -> Text,
        desc -> Text,
        updatetime -> Text,
    }
}


// Tags
// table! {
//     tags(tag) {
//         tag -> Text,
//         refs -> Text,
//     }
// }
