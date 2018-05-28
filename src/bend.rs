use model::* ;
pub struct BEndService {}
impl BEndService{ 
    pub fn new() ->BEndService
    {
        BEndService{}
    }
}
impl ExamService  for BEndService
{
    fn wait_judge(&self,_user: String) -> Token 
    {
        Token::new()
    }
    fn wait_question(&self) -> ExamQuest 
    {
        ExamQuest::stub() 
    }

} 

