use serde_json::{to_value, Value};
use std::convert::From;


/// Represents email message including some mata-data
/// use MessageBuilder to build the email
///
#[derive(Debug, Serialize, Default)]
pub struct Message {
    pub options: Options,
    campaign_id: Option<String>,
    recipients: Vec<Recipient>,
    content: Content,
}

impl Message {
    pub fn new(sender_email_address: EmailAddress) -> Message {
        let mut message = Message::default();
        message.content.from = sender_email_address;
        message
    }

    pub fn with_options(sender_email_address: EmailAddress, options: Options) -> Message {
        let mut message = Message::default();
        message.options = options;
        message.content.from = sender_email_address;
        message
    }
    /// Adds one recipient at a time, can be called multiple times
    pub fn add_recipient(mut self, address: EmailAddress) -> Message {
        self.recipients.push(Recipient {
            address,
        });
        self
    }
    pub fn set_subject(mut self, subject: &str) -> Message {
        self.content.subject = subject.to_owned();
        self
    }
    pub fn set_options(mut self, options: Options) -> Message {
        self.options = options;
        self
    }
    pub fn set_html(mut self, html: &str) -> Message {
        self.content.html = Some(html.to_owned());
        self
    }
    pub fn set_text(mut self, text: &str) -> Message {
        self.content.text = Some(text.to_owned());
        self
    }
    /// returns a json structure to be sent over http
    /// ```json
    ///{
    ///  "campaign_id": "postman_inline_both_example",
    ///  "recipients": [
    ///    {
    ///      "address": {"email": "wilma@example.sink.sparkpostmail.com", "name": "Name"}
    ///    }
    ///  ],
    ///  "content": {
    ///    "from": {
    ///      "email": "marketing@example.sink.sparkpostmail.com",
    ///      "name": "Example Company"
    ///    },
    ///
    ///    "subject": "SparkPost inline template example",
    ///    "html": "<html><body>Here is your inline html, {{first_name or 'you great person'}}!<br></body></html>",
    ///    "text": "Here is your plain text, {{first_name or 'you great person'}}!"
    ///  }
    ///}
    /// ```
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
pub struct Recipient {
    address: EmailAddress,
}


/// Email address with name
///
/// ### Example
/// ```rust
/// use spark_post::EmailAddress;
///
/// let expected = EmailAddress::new("test@test.com");
/// let address: EmailAddress = "test@test.com".into();
///
/// assert_eq!(expected, address);
///
/// // create address with name
/// let address = EmailAddress::with_name("test@test.com", "Joe Blow");
///```
#[derive(Debug, Serialize, Default, PartialEq)]
pub struct EmailAddress {
    pub(crate) email: String,
    pub(crate) name: Option<String>,
}

impl EmailAddress {
    pub fn new(email: &str) -> Self {
        EmailAddress {
            email: email.to_owned(),
            name: None,
        }
    }
    pub fn with_name(email: &str, name: &str) -> Self {
        EmailAddress {
            email: email.to_owned(),
            name: Some(name.to_owned()),
        }
    }
}

impl From<&str> for EmailAddress {
    fn from(email: &str) -> Self {
        EmailAddress {
            email: email.to_owned(),
            name: None,
        }
    }
}

impl From<String> for EmailAddress {
    fn from(email: String) -> Self {
        EmailAddress {
            email,
            name: None,
        }
    }
}


#[derive(Debug, Serialize, Default)]
pub struct Content {
    from: EmailAddress,
    subject: String,
    tags: Option<Vec<String>>,
    text: Option<String>,
    html: Option<String>,
    template_id: Option<String>,
}

///// MessageBuilder for convenience
//#[derive(Debug, Default)]
//pub struct MessageBuilder {
//    message: Message,
//}
//
//impl MessageBuilder {
//    pub fn new(sender_email_address: EmailAddress) -> MessageBuilder {
//        let mut message = Message::default();
//        message.content.from = sender_email_address;
//        MessageBuilder { message }
//    }
//
//    pub fn with_options(sender_email_address: EmailAddress, options: Options) -> MessageBuilder {
//        let mut message = Message::default();
//        message.options = options;
//        message.content.from = sender_email_address;
//        MessageBuilder { message }
//    }
//    /// Adds one recipient at a time, can be called multiple times
//    pub fn add_recipient(mut self, address: EmailAddress) -> MessageBuilder {
//        self.message.recipients.push(Recipient {
//            address,
//        });
//        self
//    }
//    pub fn set_subject(mut self, subject: &str) -> MessageBuilder {
//        self.message.content.subject = subject.to_owned();
//        self
//    }
//    pub fn set_options(mut self, options: Options) -> MessageBuilder {
//        self.message.options = options;
//        self
//    }
//    pub fn set_html(mut self, html: &str) -> MessageBuilder {
//        self.message.content.html = Some(html.to_owned());
//        self
//    }
//    pub fn set_text(mut self, text: &str) -> MessageBuilder {
//        self.message.content.text = Some(text.to_owned());
//        self
//    }
//    pub fn finish(self) -> Message {
//        self.message
//    }
//}


#[test]
fn create_address() {
    let address: EmailAddress = "test@test.com".into();
    assert_eq!("test@test.com", address.email.as_str());
}

#[test]
fn create_message() {
    let email: Value = Message::new("test@test.com".into())
        .add_recipient("tech@hgill.io".into())
        .json();
//    println!("{:#?}", email.to_string());
    assert_eq!("test@test.com", email["content"]["from"]["email"].as_str().unwrap());
//    assert_eq!("name", email["content"]["from"]["name"].as_str().unwrap());
    assert_eq!("test@test.com", email["content"]["from"]["email"].as_str().unwrap());
    assert!(email["options"]["sandbox"].as_bool().unwrap());
    assert!(!email["options"]["click_tracking"].as_bool().unwrap());
    assert!(!email["options"]["open_tracking"].as_bool().unwrap());
    assert!(!email["options"]["transactional"].as_bool().unwrap());
}

#[test]
fn create_message_with_options() {
    let email: Value = Message::with_options(
        EmailAddress::with_name("test@test.com", "name"),
        Options {
            open_tracking: true,
            click_tracking: true,
            transactional: true,
            sandbox: true,
            inline_css: false,
        },
    )
        .json();

    assert_eq!("test@test.com", email["content"]["from"]["email"].as_str().unwrap());
    assert_eq!("name", email["content"]["from"]["name"].as_str().unwrap());
    assert_eq!("test@test.com", email["content"]["from"]["email"].as_str().unwrap());
    assert!(email["options"]["sandbox"].as_bool().unwrap());
    assert!(email["options"]["click_tracking"].as_bool().unwrap());
    assert!(email["options"]["open_tracking"].as_bool().unwrap());
    assert!(email["options"]["transactional"].as_bool().unwrap());
    assert!(!email["options"]["inline_css"].as_bool().unwrap());
}

#[test]
fn create_options() {
    let options = Options::default();
    assert_eq!(false, options.click_tracking);
    assert_eq!(false, options.open_tracking);
    assert_eq!(true, options.sandbox);
    assert_eq!(false, options.transactional);
//    assert!(!options.click_tracking);
}

