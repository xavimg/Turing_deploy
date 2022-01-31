use std::{collections::{HashSet}, hash::Hash};
use crate::{PlayerTokenLoged};

pub struct DatabaseCache<T: Hash + Eq, const MAX: usize>(HashSet<T>);

pub async fn check_logged_user (token: PlayerTokenLoged) {

}