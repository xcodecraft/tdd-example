#[macro_use] extern crate log;
use std::rc::Rc ;
extern crate pretty_env_logger;
fn main() {
    pretty_env_logger::init();
    info!("test-driver start!");
    println!("Hello, world!");
    let exam_svc = Box::new(ExamService::new()) ;
    serving(exam_svc);
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
    users : Vec<UserRc>,
    judge_svc : Box<JudgeService>,
}
impl ExamRoom
{
    pub fn new( judge_svc : Box<JudgeService>) -> ExamRoom
    {
        ExamRoom{ users: Vec::new(), judge_svc }
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

pub trait JudgeService
{

}
fn serving( judge_svc :Box<JudgeService>)
{
    let mut muggle = UserRc::new(User::new());
    let mug_ref    = muggle.as_ref();
    let mut room   = ExamRoom::new(judge_svc);

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

struct ExamService {}
impl ExamService{ 
    pub fn new() ->ExamService
    {
        ExamService{}
    }
}
impl JudgeService  for ExamService
{

} 

//#[cfg(test)]
mod tests
{
    use super::* ;
    struct JudgeStub { }
    impl JudgeStub
    {
        pub fn new() -> JudgeStub
        {
            JudgeStub{}
        }
    }
    impl JudgeService  for JudgeStub
    {
    } 
    #[test]
    fn useage()
    {
        //TODO: #1
        let judge_svc = Box::new(JudgeStub::new()) ;
        serving(judge_svc);
    }
}
