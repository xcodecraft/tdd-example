
pub trait JudgeService
{
    fn wait_judge(&self,user: String) -> Token ;
}

pub trait ExamUi
{
    fn show_question(&self, question : &ExamQuest) ;
    fn wait_answer(&self) -> Answer ;
}
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

#[derive(Debug,Clone)]
pub struct ExamQuest
{}
impl ExamQuest
{
    pub fn new() -> ExamQuest
    {
        ExamQuest{}
    }
}

#[derive(Debug,Clone)]
pub struct Answer
{
    chose : String,

}
impl Answer
{
    pub fn new(chose : String ) -> Answer
    {
        Answer{chose}
    }
    pub fn stub()-> Answer
    {
        Answer{chose : String::from("A")}
    }
}

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct AnswerSheet
{
    answers : Vec<(String,String)>

}
impl AnswerSheet
{
    pub fn new() -> AnswerSheet
    {

        let answers = Vec::new();
        AnswerSheet{ answers}

    }
    pub fn record(&mut self ,id : String, chose : String)
    {
        self.answers.push( (id,chose) ) ;
        
    }
}
