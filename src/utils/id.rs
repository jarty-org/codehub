use uuid::Uuid;

pub fn create_uid() -> String {
    let id = format!("{}", Uuid::new_v4());
    return id.replace("-", "");
}