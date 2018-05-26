#[macro_use] extern crate log;
extern crate pretty_env_logger;
fn main() {
    pretty_env_logger::init();
    info!("test-driver start!");
    println!("Hello, world!");
}


struct ExamRoom
{}

struct ExamQuest
{}

struct AnswerSheet
{

}

struct User
{}

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

        //room.count_donw(10);
        room.wait_start() ;
        while token.is_ok() && room.is_open()
        {
            let question = room.wait_question() ;
            let answer   = one.wait_answer(question) ;
            room.post_answer(answer) ;
            token = room.wait_judge(one);
        }
//        room.stop();

    }
}
