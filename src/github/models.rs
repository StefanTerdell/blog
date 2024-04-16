use crate::utils::user::User;
use leptos::*;

pub type UserResource = Resource<(), Result<Option<User>, ServerFnError>>;
