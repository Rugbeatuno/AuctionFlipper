use lazy_static::lazy_static;
use std::time::Duration;

extern crate surf;

lazy_static! {
    pub static ref HTTP_CLIENT1: surf::Client = surf::Config::new()
    .set_timeout(Some(Duration::from_secs(15)))
    .try_into()
    .unwrap();
    
    
}