use users::{get_user_by_uid, get_current_uid};

fn main() {
    let current_uid = get_current_uid();
    let current_user = get_user_by_uid(current_uid).unwrap();

    for group in current_user.groups().expect("Error looking for groups") {
        println!("User is a member of {:?}", group.name());
    }
}
