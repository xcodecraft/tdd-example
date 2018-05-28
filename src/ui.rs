use model::* ;
#[derive(Clone)]
pub struct PhoneUI 
{}

impl PhoneUI 
{
    pub fn stub() ->PhoneUI
    {
        PhoneUI{}
    }
}

impl ExamUi for PhoneUI
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
