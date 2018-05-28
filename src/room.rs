use model::* ;
use user::* ;

struct ExamRoom
{
    users : Vec<UserRc>,
    exam_svc : Box<ExamService>,
}
impl ExamRoom
{
    pub fn new( exam_svc : Box<ExamService>) -> ExamRoom
    {
        ExamRoom{ users: Vec::new(), exam_svc }
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
        self.exam_svc.wait_question() 

    }
    pub fn post_answer(&self, _answer : &Answer)
    {

    }
    pub fn wait_judge(&self,_user : &User) -> Token
    {
        self.exam_svc.wait_judge(String::from("stub user-id")) 
    }
    pub fn is_open(&self) -> bool
    {
        true
    }
}


pub fn serving( exam_svc :Box<ExamService>, muggle : UserRc)
{
    let mug_ref  = muggle.as_ref();
    let mut room = ExamRoom::new(exam_svc);

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
    struct ExamSvcStub { 
        judge_vec : RefCell<Vec<Token>> ,
        quest_vec : RefCell<Vec<ExamQuest>>
    }
    impl ExamSvcStub
    {
        pub fn new() -> ExamSvcStub
        {
            let judge_vec = RefCell::new(vec! [ 
                                         Token::new(),
                                         Token::stub(), 
                                         Token::stub(), 
                                         Token::stub(), 
                                         ]) ;

            let quest_vec = RefCell::new(vec! [ 
                                         ExamQuest::stub(),
                                         ExamQuest::stub(),
                                         ExamQuest::stub(),
                                         ExamQuest::stub(),
                                         ExamQuest::stub(),
                                         ExamQuest::stub(),
                                         ExamQuest::stub(),
                                         ExamQuest::stub(),
                                         ExamQuest::stub(),
                                         ]) ;
            ExamSvcStub{ judge_vec,quest_vec }
        }
    }
    impl ExamService  for ExamSvcStub
    {
        fn wait_judge(&self,_user: String) -> Token 
        {
            self.judge_vec.borrow_mut().pop().expect("empty stub data ")
        }
        fn wait_question(&self) -> ExamQuest 
        {
            self.quest_vec.borrow_mut().pop().expect("empty stub data ")
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
            info!("UI: question : {:?}" , question) ;
        }
        fn wait_answer(&self) -> Answer 
        {
            Answer::new(String::from("A")) 
        }

    }

    #[test]
    fn useage()
    {
        pretty_env_logger::init();
        debug!("begin...") ;
        let exam_svc = Box::new(ExamSvcStub::new()) ;
        let muggle   = UserRc::new(User::new(Box::new(StubUI::stub())));
        let mug_ref  = muggle.as_ref();
        let mut room = ExamRoom::new(exam_svc);

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
