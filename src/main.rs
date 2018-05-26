#[macro_use] extern crate log;
extern crate pretty_env_logger;
fn main() {
    pretty_env_logger::init();
    info!("test-driver start!");
    println!("Hello, world!");
    let room = ExamRoom::new();
}

struct User
{}

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
    pub fn is_ok() -> bool
    {
        true 
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
       self.users.insert(user)  ;
       Token::new()

    }
    pub fn wait_start(&self)
    {

    }
    pub fn wait_question()
    {

    }
    pub fn post_answer(&self)
    {

    }
    pub fn wait_judge(&self) -> Token
    {
        Token::new()
    }
}

struct ExamQuest
{}

struct AnswerSheet
{

}

struct JudgeService
{

}
//#[cfg(test)]
mod tests
{
    use super::* ;
    #[test]
    fn useage()
    {
        let one  = User::new();
        let room = ExamRoom::new();

        let token = room.join(one);
        room.wait_start() ;
        while token.is_ok() && room.is_open()
        {
            let question = room.wait_question() ;
            let answer   = one.wait_answer(question) ;
            room.post_answer(answer) ;
            token = room.wait_judge(one);
        }

    }
}
