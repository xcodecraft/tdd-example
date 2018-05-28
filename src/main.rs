#[macro_use] extern crate log;
extern crate pretty_env_logger;

mod model ;
mod room  ;
mod user ;
mod bend ;
mod ui ; 

use user::* ;
fn main() {
    pretty_env_logger::init();
    info!("tdd-example start!");
    let exam_svc = Box::new(bend::BEndService::new()) ;
    let muggle   = UserRc::new(User::new(Box::new(ui::PhoneUI::stub())));
    room::serving(exam_svc,muggle);
}








