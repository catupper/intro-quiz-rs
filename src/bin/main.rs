use device_query::Keycode;
use intro_quiz_rs::*;

use clap::{App, Arg};

fn main() {
    let matches = App::new("IntroQuiz!")
        .version("1.0")
        .author("Ken Ogura <nekarugo628@gmail.com>")
        .about("Intro Quiz with your own mp3 files!")
        .arg(
            Arg::with_name("list_dir")
                .short("l")
                .long("list")
                .value_name("LIST_DIR")
                .help("Set a QUIZ mp3 list directory path.\n Don't forget to add list.csv.")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("se_dir")
                .short("s")
                .long("se")
                .value_name("SE_DIR")
                .help("Set a SE mp3 list directory path.\n Have to include:\nporin, pexan, pinpon, buboo, bbbbb.")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let list_file_dir = matches.value_of("list_dir").expect("List Please");
    let se_file_dir = matches.value_of("se_dir").expect("SE Please");

    let device = rodio::default_output_device().unwrap();
    let mut intro_quiz = IntroQuizSet::from_path(list_file_dir, se_file_dir, &device);

    let mut event_listener = EventListener::default();
    loop {
        let events = event_listener.get_events();
        if intro_quiz.is_standby() && events.contains(&(Keycode::Q, KeyEvent::Press)) {
            intro_quiz.play(&device);
        }
        if intro_quiz.is_playing() && events.contains(&(Keycode::Space, KeyEvent::Press)) {
            intro_quiz.answer();
        }
        if intro_quiz.is_answering() && events.contains(&(Keycode::A, KeyEvent::Press)) {
            intro_quiz.ac();
        }
        if intro_quiz.is_answering() && events.contains(&(Keycode::W, KeyEvent::Press)) {
            intro_quiz.wa();
        }
        if intro_quiz.is_answering() && events.contains(&(Keycode::T, KeyEvent::Press)) {
            intro_quiz.tle();
        }
        if intro_quiz.is_standby() && events.contains(&(Keycode::N, KeyEvent::Press)) {
            intro_quiz.next();
            intro_quiz.print_problem();
        }
        if intro_quiz.is_standby() && events.contains(&(Keycode::P, KeyEvent::Press)) {
            intro_quiz.prev();
            intro_quiz.print_problem();
        }
        if events.contains(&(Keycode::Escape, KeyEvent::Press)) {
            break;
        }
    }
}
