// this is automatically linked because it is named lib.rs
// otherwise you'd need to link it the normal way

pub struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
