use serde_json::{to_value, Value};


/// Represents email message including some mata-data
/// use MessageBuilder to build the email
#[derive(Debug, Serialize, Default)]
pub struct Message {
    options: Options,
    recipients: Vec<Recipient>,
    content: Content,
}

impl Message {
    pub fn json(&self) -> Value {
        to_value(self).unwrap()
    }
}

#[derive(Debug, Serialize)]
pub struct Options {
    pub open_tracking: bool,
    pub click_tracking: bool,
    pub transactional: bool,
    pub sandbox: bool,
    pub inline_css: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            open_tracking: false,
            click_tracking: false,
            transactional: false,
            sandbox: true,
            inline_css: false,
        }
    }
}


#[derive(Debug, Serialize, Default)]
struct Recipient {
    address: EmailAddress,
}

#[derive(Debug, Serialize, Default)]
struct EmailAddress {
    email: String,
    name: Option<String>,
}

#[derive(Debug, Serialize, Default)]
struct Content {
    from: EmailAddress,
    subject: String,
    tags: Option<Vec<String>>,
    text: Option<String>,
    html: Option<String>,
    template_id: Option<String>,
}

/// MessageBuilder for convenience
#[derive(Debug, Default)]
pub struct MessageBuilder {
    message: Message,
}

impl MessageBuilder {
    pub fn new(sender_email: &str, sender_name: &str) -> MessageBuilder {
        let mut message = Message::default();
        message.content.from = EmailAddress {
            name: Some(sender_name.to_owned()),
            email: sender_email.to_owned(),
        };
        MessageBuilder { message }
    }
    /// Adds one recipient at a time, can be called multiple times
    pub fn add_recipient(mut self, email: &str, name: Option<&str>) -> MessageBuilder {
        let name = match name {
            Some(n) => Some(n.to_string()),
            None => None
        };
        self.message.recipients.push(Recipient {
            address: EmailAddress {
                email: email.to_owned(),
                name,
            },
        });
        self
    }
    pub fn set_subject(mut self, subject: &str) -> MessageBuilder {
        self.message.content.subject = subject.to_owned();
        self
    }
    pub fn set_options(mut self, options: Options) -> MessageBuilder {
        self.message.options = options;
        self
    }
    pub fn set_html(mut self, html: &str) -> MessageBuilder {
        self.message.content.html = Some(html.to_owned());
        self
    }
    pub fn set_text(mut self, text: &str) -> MessageBuilder {
        self.message.content.text = Some(text.to_owned());
        self
    }
    pub fn finish(self) -> Message {
        self.message
    }
}
