use model::* ;
use std::rc::Rc ;
#[derive(Clone)]
pub struct User
{
    sheet : AnswerSheet,
    exam_ui : Rc<Box<ExamUi>> ,

}
pub type UserRc = Rc<User>;
impl User
{
    pub fn new(ui : Box<ExamUi>) -> User
    {
        User{sheet : AnswerSheet::new(), exam_ui: Rc::new(ui) }
    }
    pub fn wait_answer(&self, question :&ExamQuest) ->Answer
    {
        let ui_ref  = self.exam_ui.as_ref();
        ui_ref.show_question(question) ;
        Answer::stub() 
    }
    pub fn answer_sheet(&self) -> AnswerSheet
    {
        AnswerSheet::new()

    }
}
