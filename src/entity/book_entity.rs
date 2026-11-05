use std::fmt::Display;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct BookEntity {
    id: u64,
    name: String,
    author: String,
    translator: String,
    publisher: String,
    context: String,
}

impl BookEntity {
    #[must_use]
    pub fn new(
        id: u64,
        name: impl Into<String>,
        author: impl Into<String>,
        translator: impl Into<String>,
        publisher: impl Into<String>,
        context: impl Into<String>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            author: author.into(),
            translator: translator.into(),
            publisher: publisher.into(),
            context: context.into(),
        }
    }

    pub const fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    pub fn set_author(&mut self, author: impl Into<String>) {
        self.author = author.into();
    }

    pub fn set_translator(&mut self, translator: impl Into<String>) {
        self.translator = translator.into();
    }

    pub fn set_publisher(&mut self, publisher: impl Into<String>) {
        self.publisher = publisher.into();
    }

    pub fn set_context(&mut self, context: impl Into<String>) {
        self.context = context.into();
    }

    #[must_use]
    pub const fn id(&self) -> u64 {
        self.id
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn author(&self) -> &str {
        &self.author
    }

    #[must_use]
    pub fn translator(&self) -> &str {
        &self.translator
    }

    #[must_use]
    pub fn publisher(&self) -> &str {
        &self.publisher
    }

    #[must_use]
    pub fn context(&self) -> &str {
        &self.context
    }
}

impl Display for BookEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[id:{}, name:{}, author:{}, translator:{}, publisher:{}, context:{}]",
            self.id(),
            self.name(),
            self.author(),
            self.translator(),
            self.publisher(),
            self.context()
        )
    }
}
