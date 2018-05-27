#[macro_use] extern crate log;
use std::rc::Rc ;
extern crate pretty_env_logger;
trait JudgeService
{
    fn wait_judge(&self,user: &User) -> Token ;
}

fn main() {
    pretty_env_logger::init();
    info!("test-driver start!");
    println!("Hello, world!");
    let exam_svc = Box::new(ExamService::new()) ;
    serving(exam_svc);
}
#[derive(Clone)]
pub struct User
{}
impl User
{
    pub fn new() -> User
    {
        User{}
    }
    fn wait_answer(&self, _question :&ExamQuest) ->Answer
    {
        Answer::new() 
    }
}

type UserRc = std::rc::Rc<User>;

#[derive(Debug,Clone)]
pub struct Token
{
    data : String
}
impl Token
{
    pub fn new() -> Token
    {
        Token{ data: String::from("")}
    }
    pub fn stub() -> Token
    {
        Token{ data: String::from("YES") }
    }
    pub fn is_ok(&self) -> bool
    {
        //TODO: stub impl
        self.data.len() > 0
    }
}
pub struct Answer
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
       Token::stub()

    }
    pub fn wait_start(&self)
    {

    }
    pub fn wait_question(&self) -> ExamQuest
    {
        ExamQuest::new()

    }
    pub fn post_answer(&self, _answer : &Answer)
    {

    }
    pub fn wait_judge(&self,user : &User) -> Token
    {
        self.judge_svc.wait_judge(user) 
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

#[allow(dead_code)]
struct AnswerSheet
{

}

fn serving( judge_svc :Box<JudgeService>)
{
    let muggle   = UserRc::new(User::new());
    let mug_ref  = muggle.as_ref();
    let mut room = ExamRoom::new(judge_svc);

    let mut token = room.join(muggle.clone());
    room.wait_start() ;
    while token.is_ok() && room.is_open()
    {
        let question = room.wait_question() ;
        let answer   = mug_ref.wait_answer(&question) ;
        room.post_answer(&answer) ;
        token = room.wait_judge(mug_ref);
        debug!("answer token {:?}", token);
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
    fn wait_judge(&self,_user: &User) -> Token 
    {
        Token::new()
    }

} 

#[cfg(test)]
mod tests
{
    use super::* ;
    use std::cell::RefCell ;
    struct JudgeStub { 
        judge_vec : RefCell<Vec<Token>>
    }
    impl JudgeStub
    {
        pub fn new() -> JudgeStub
        {
            let judge_vec = RefCell::new(vec! [ 
                                         Token::new(),
                                         Token::stub(), 
                                         Token::stub(), 
                                         Token::stub(), 
                                         ]) ;
            JudgeStub{ judge_vec }
        }
    }
    impl JudgeService  for JudgeStub
    {
        fn wait_judge(&self,_user: &User) -> Token 
        {
            self.judge_vec.borrow_mut().pop().expect("empty stub data ")
        }
    } 
    #[test]
    fn useage()
    {
        //TODO: #1
        pretty_env_logger::init();
        debug!("begin...") ;
        let judge_svc = Box::new(JudgeStub::new()) ;
        let muggle   = UserRc::new(User::new());
        let mug_ref  = muggle.as_ref();
        let mut room = ExamRoom::new(judge_svc);

        let mut token = room.join(muggle.clone());
        room.wait_start() ;
        let mut times = 0 ;
        while token.is_ok() && room.is_open()
        {
            let question = room.wait_question() ;
            let answer   = mug_ref.wait_answer(&question) ;
            room.post_answer(&answer) ;
            token = room.wait_judge(mug_ref);
            debug!("answer token {:?}", token);
            times += 1 ;
        }
        assert!(times > 1) ;

    }
}
