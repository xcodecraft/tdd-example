use model::* ;
use user::User ;
pub struct ExamService {}
impl ExamService{ 
    pub fn new() ->ExamService
    {
        ExamService{}
    }
}
impl JudgeService  for ExamService
{
    fn wait_judge(&self,_user: String) -> Token 
    {
        Token::new()
    }

} 

