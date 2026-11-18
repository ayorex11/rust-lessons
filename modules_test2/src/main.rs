mod media{
    pub mod audio{
        pub struct Track{ 
            pub title: String
        }
        pub fn describe(t: &Track) -> String{
            format!("audio: {}", t.title)
        }
    }
    pub mod video{
        pub struct Track{
            pub title: String, 
            pub seconds: u32 
        }
        pub fn describe(t: &Track) -> String{
            format!("video: {} ({}s)", t.title, t.seconds)
        }
    }
}

pub mod library{
        pub use crate::media::audio::Track;
}
use media::audio::Track;
use media::video::Track as VideoTrack;
use media::audio;
use media::video;

fn main() {
    let audio_track = Track{
        title: String::from("Intro")
    };
    let audio_description = audio::describe(&audio_track);
    let video_track = VideoTrack{
        title: String::from("Demo"),
        seconds: 90
    };
    let video_description = video::describe(&video_track);
    let bonus = library::Track{
        title: String::from("Bonus")
    };
    let bonus_description = audio::describe(&bonus);
    println!("{audio_description}");
    println!("{video_description}");
    println!("{bonus_description}");

}
