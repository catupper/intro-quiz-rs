use glob::glob;
use id3::Tag;
use rodio::{Device, Sink};
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Clone)]
pub struct IntroQuiz {
    pub title: String,
    pub artist: String,
    pub file_path: String,
}

impl std::fmt::Display for IntroQuiz {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Title: {}\nArtist {}\n", self.title, self.artist)
    }
}

#[derive(Debug, Clone)]
pub enum IntroQuizState {
    Standby,
    Playing,
    Answering,
}

pub struct IntroQuizSet {
    quiz_list: Vec<IntroQuiz>,
    se_path: String,
    index: usize,
    state: IntroQuizState,
    music_sink: Sink,
    se_sink: Sink,
}

impl IntroQuizSet {
    pub fn new(quiz_list: Vec<IntroQuiz>, se_path: &str, device: &Device) -> Self {
        Self {
            quiz_list,
            se_path: se_path.to_string(),
            index: 0,
            state: IntroQuizState::Standby,
            music_sink: rodio::Sink::new(device),
            se_sink: rodio::Sink::new(device),
        }
    }

    pub fn from_path(list_path: &str, se_path: &str, device: &Device) -> Self {
        Self::new(get_quiz(list_path), se_path, device)
    }

    pub fn reset(&mut self) {
        self.index = 0;
        self.state = IntroQuizState::Standby;
    }

    pub fn play(&mut self, device: &Device) {
        porin(&self.se_sink, &self.se_path);
        self.music_sink = rodio::Sink::new(device);
        self.music_sink.play();
        let file = File::open(&self.quiz_list[self.index].file_path).unwrap();
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
        self.music_sink.append(source);
        self.state = IntroQuizState::Playing;
    }

    pub fn answer(&mut self) {
        self.music_sink.stop();
        pexan(&self.se_sink, &self.se_path);
        self.state = IntroQuizState::Answering;
    }

    fn clear(&mut self) {
        self.state = IntroQuizState::Standby;
    }

    pub fn ac(&mut self) {
        pinpong(&self.se_sink, &self.se_path);
        self.clear()
    }
    pub fn wa(&mut self) {
        bubboo(&self.se_sink, &self.se_path);
        self.clear()
    }
    pub fn tle(&mut self) {
        self.music_sink.stop();
        bbbbb(&self.se_sink, &self.se_path);
        self.clear()
    }

    pub fn next(&mut self) {
        self.state = IntroQuizState::Standby;
        self.index += 1;
        self.index %= self.quiz_list.len();
    }

    pub fn prev(&mut self) {
        self.state = IntroQuizState::Standby;
        if self.index == 0 {
            self.index = self.quiz_list.len() - 1;
        } else {
            self.index -= 1;
        }
    }

    pub fn is_standby(&self) -> bool {
        match self.state {
            IntroQuizState::Standby => true,
            _ => false,
        }
    }

    pub fn is_playing(&self) -> bool {
        match self.state {
            IntroQuizState::Playing => true,
            _ => false,
        }
    }

    pub fn is_answering(&self) -> bool {
        match self.state {
            IntroQuizState::Answering => true,
            _ => false,
        }
    }

    pub fn print_problem(&self) {
        println!("{}", self.quiz_list[self.index]);
    }
}

pub fn get_quiz(dir_path: &str) -> Vec<IntroQuiz> {
    let mut res = vec![];
    for music_file in glob(&format!("{}/*.mp3", dir_path)).expect("Failed to read glob pattern") {
        if let Ok(file_path) = music_file {
            let tag = Tag::read_from_path(file_path.clone()).unwrap();
            res.push(IntroQuiz {
                title: tag.title().unwrap().to_string(),
                artist: tag.artist().unwrap().to_string(),
                file_path: file_path.to_str().unwrap().to_string(),
            })
        }
    }
    res
}

pub fn porin(sink: &Sink, se_path: &str) {
    let file = File::open(&format!("{}/porin_sound.mp3", se_path)).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    sink.append(source);
    sink.sleep_until_end();
}

pub fn pexan(sink: &Sink, se_path: &str) {
    let file = File::open(&format!("{}/pexan_sound.mp3", se_path)).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    sink.append(source);
}

pub fn pinpong(sink: &Sink, se_path: &str) {
    let file = File::open(&format!("{}/pinpong_sound.mp3", se_path)).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    sink.append(source);
}

pub fn bubboo(sink: &Sink, se_path: &str) {
    let file = File::open(&format!("{}/bubboo_sound.mp3", se_path)).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    sink.append(source);
    sink.sleep_until_end();
}

pub fn bbbbb(sink: &Sink, se_path: &str) {
    let file = File::open(&format!("{}/bbbbb_sound.mp3", se_path)).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    sink.append(source);
}
