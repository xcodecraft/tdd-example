#[macro_use] extern crate log;
use std::rc::Rc ;
extern crate pretty_env_logger;
fn main() {
    pretty_env_logger::init();
    info!("test-driver start!");
    println!("Hello, world!");
    serving();
}
#[derive(Clone)]
struct User
{}
impl User
{
    pub fn new() -> User
    {
        User{}
    }
    pub fn wait_answer(&self, question :&ExamQuest) ->Answer
    {
        Answer::new() 
    }
}

type UserRc = std::rc::Rc<User>;

struct Token
{
}
impl Token
{
    pub fn new() -> Token
    {
        Token{}
    }
    pub fn is_ok(&self) -> bool
    {
        true 
    }
}
struct Answer
{

}
impl Answer
{
    pub fn new() -> Answer
    {
        Answer{}
    }
}
struct ExamRoom
{
    users : Vec<UserRc>
}
impl ExamRoom
{
    pub fn new() -> ExamRoom
    {
        ExamRoom{ users: Vec::new() }
    }
    pub fn join( &mut self , user : UserRc ) -> Token
    {
       self.users.push(user)  ;
       Token::new()

    }
    pub fn wait_start(&self)
    {

    }
    pub fn wait_question(&self) -> ExamQuest
    {
        ExamQuest::new()

    }
    pub fn post_answer(&self, answer : &Answer)
    {

    }
    pub fn wait_judge(&self,user : &User) -> Token
    {
        Token::new()
    }
    pub fn is_open(&self) -> bool
    {
        true
    }
}

struct ExamQuest
{}
impl ExamQuest
{
    pub fn new() -> ExamQuest
    {
        ExamQuest{}
    }
}

struct AnswerSheet
{

}

struct JudgeService
{

}
fn serving()
{
    let mut muggle = UserRc::new(User::new());
    let mug_ref    = muggle.as_ref();
    let mut room   = ExamRoom::new();

    let mut token = room.join(muggle.clone());
    room.wait_start() ;
    while token.is_ok() && room.is_open()
    {
        let question = room.wait_question() ;
        let answer   = mug_ref.wait_answer(&question) ;
        room.post_answer(&answer) ;
        token = room.wait_judge(mug_ref);
        break;
    }

}
//#[cfg(test)]
mod tests
{
    use super::* ;
    #[test]
    fn useage()
    {
        serving();
    }
}
