use {
    super::id::Id,
    serenity::{builder::CreateMessage, model::channel::Message, utils::Color},
    std::{ffi::OsStr, path::PathBuf},
};

const IMAGE_FORMATS: [&str; 3] = ["jpg", "jpeg", "png"];
const COLOR: Color = Color::from_rgb(0, 0, 0);

pub trait Image {
    fn path(&self) -> PathBuf;
    fn id(&self) -> Id;
}

pub trait ImageMut {
    fn add_message(&mut self, msg: Message);
    fn remove_message(&mut self) -> Option<Message>;
}

#[derive(Clone, Debug)]
pub struct RawImage {
    messages: Vec<Message>,
    file_name: String,
    path: PathBuf,
    title: String,
    id: Id,
}

impl RawImage {
    pub fn new(path: PathBuf, id: Id, title: String) -> Option<Self> {
        Some(Self {
            messages: Vec::new(),
            file_name: path.file_name()?.to_str()?.to_string(),
            path,
            title,
            id,
        })
    }

    pub fn is_image<T: AsRef<OsStr>>(s: T) -> Option<bool> {
        Some(IMAGE_FORMATS.contains(&s.as_ref().to_str()?.to_lowercase().as_str()))
    }

    pub fn as_message<'a, 'b>(&'b self, m: &'a mut CreateMessage<'b>) -> &'a mut CreateMessage<'b> {
        m.embed(|e| {
            e.title(self.title.clone());
            e.attachment(self.file_name.clone());
            e.color(COLOR);
            e.footer(|f| f.text(self.id.clone()))
        });
        m.add_file(&self.path)
    }
}

impl Image for RawImage {
    fn path(&self) -> PathBuf {
        self.path.clone()
    }

    fn id(&self) -> Id {
        self.id.clone()
    }
}

impl ImageMut for RawImage {
    fn add_message(&mut self, msg: Message) {
        self.messages.push(msg)
    }

    fn remove_message(&mut self) -> Option<Message> {
        let n = self.messages.len();
        if n == 0 {
            None
        } else {
            Some(self.messages.remove(n - 1))
        }
    }
}
