use model::* ;
use user::* ;

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
    pub fn wait_judge(&self,_user : &User) -> Token
    {
        self.judge_svc.wait_judge(String::from("stub user-id")) 
    }
    pub fn is_open(&self) -> bool
    {
        true
    }
}


pub fn serving( judge_svc :Box<JudgeService>, muggle : UserRc)
{
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
#[cfg(test)]
mod tests
{
    use super::* ;
    use pretty_env_logger ;
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
        fn wait_judge(&self,_user: String) -> Token 
        {
            self.judge_vec.borrow_mut().pop().expect("empty stub data ")
        }
    } 

    #[derive(Clone)]
    struct StubUI 
    {}

    impl StubUI 
    {
        pub fn stub() ->StubUI
        {
            StubUI{}
        }
    }

    impl ExamUi for StubUI
    {
        fn show_question(&self, question : &ExamQuest) 
        {
            debug!("UI: question : {:?}" , question) ;
        }
        fn wait_answer(&self) -> Answer 
        {
            Answer::new(String::from("A")) 
        }

    }

    #[test]
    fn useage()
    {
        //TODO: #1
        pretty_env_logger::init();
        debug!("begin...") ;
        let judge_svc = Box::new(JudgeStub::new()) ;
        let muggle   = UserRc::new(User::new(Box::new(StubUI::stub())));
        let mug_ref  = muggle.as_ref();
        let mut room = ExamRoom::new(judge_svc);

        let mut token = room.join(muggle.clone());
        room.wait_start() ;
        let mut times = 0 ;
        while token.is_ok() && room.is_open()
        {
            let question = room.wait_question() ;
            debug!("question: {:?}", question);
            let answer   = mug_ref.wait_answer(&question) ;
            debug!("answer: {:?}", answer);
            room.post_answer(&answer) ;
            token = room.wait_judge(mug_ref);
            debug!("answer token {:?}", token);
            times += 1 ;
        }
        assert!(times > 1) ;
        let sheet = mug_ref.answer_sheet() ;
        debug!("answer sheet: {:?} ",sheet) ;

    }
}
