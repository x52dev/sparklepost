use serde::Serialize;
use serde_json::to_value;
use serde_json::Value;
use std::convert::From;

/// Represents email message including some mata-data
/// ### Example
/// ```rust
///
///
/// use sparkpost::transmission::{Message, EmailAddress};
///
/// let mut email = Message::new(EmailAddress::with_name("marketing@example.sink.sparkpostmail.com", "Example Company"));
/// email.add_recipient("wilma@example.sink.sparkpostmail.com".into())
///        .campaign_id("postman_inline_both_example")
///        .subject("SparkPost inline template example")
///        .html("<html><body>Here is your inline html, {{first_name or 'you great person'}}!<br></body></html>")
///        .text("Here is your plain text, {{first_name or 'you great person'}}!");
/// ```
/// deserialize to json structure to be sent over http
/// ```json
/// {
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
///
#[derive(Debug, Serialize, Default)]
pub struct Message {
    pub options: Options,
    pub campaign_id: Option<String>,
    pub recipients: Vec<Recipient>,
    pub content: Content,
}

impl Message {
    /// create new message with sender emailAddress
    pub fn new(sender_address: EmailAddress) -> Self {
        let mut message = Message::default();
        message.content.from = sender_address;
        message
    }

    /// create new message with sending options
    pub fn with_options(sender_address: EmailAddress, options: Options) -> Self {
        let mut message = Message::default();
        message.options = options;
        message.content.from = sender_address;
        message
    }

    /// add an address to recipient list
    ///
    /// WARNING: it does not check for duplicates for now
    pub fn add_recipient(&mut self, address: EmailAddress) -> &mut Self {
        self.recipients.push(Recipient {
            address,
            substitution_data: None,
        });
        self
    }

    /// same as add_recipient but can contain substitution_data that can be serialized
    ///
    /// Usage:
    /// ```rust
    ///  // #[derive(Serialize)] struct Data{company: String}
    /// // email.add_recipient_with_substitution_data(
    /// //       "recipient@company.com".into(),
    /// //       Data{Company: "My Company".into()})
    /// ```
    pub fn add_recipient_with_substitution_data<T: Serialize>(
        &mut self,
        address: EmailAddress,
        substitution_data: T,
    ) -> &mut Self {
        let data = to_value(substitution_data);

        // TODO this feels totaly wrong here
        // need to figure out what to do
        let substitution_data = match data {
            Ok(value) => Some(value),
            _ => None,
        };
        self.recipients.push(Recipient {
            address,
            substitution_data,
        });
        self
    }

    pub fn subject(&mut self, subject: &str) -> &mut Self {
        self.content.subject = subject.to_owned();
        self
    }
    pub fn options(&mut self, options: Options) -> &mut Self {
        self.options = options;
        self
    }
    pub fn html(&mut self, html: &str) -> &mut Self {
        self.content.html = Some(html.to_owned());
        self
    }
    pub fn text(&mut self, text: &str) -> &mut Self {
        self.content.text = Some(text.to_owned());
        self
    }
    pub fn campaign_id(&mut self, campaign_id: &str) -> &mut Self {
        self.campaign_id = Some(campaign_id.to_owned());
        self
    }

    /// adds attachment to Message, multiple attachments allowed
    ///
    /// ``` rust
    /// use sparkpost::transmission::{Message, Attachment};
    ///
    /// let mut email = Message::new("marketing@example.sink.sparkpostmail.com".into());
    /// email.add_recipient("wilma@example.sink.sparkpostmail.com".into())
    ///     .add_attachment(Attachment {
    ///        file_type: "image/png".into(),
    ///        name: "AnImage.png".into(),
    ///        data: "iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAAAXNSR0IArs4c6QAAAAlwSFlzAAAWJQAAFiUBSVIk8AAAAXxJREFUOBFjvJVg84P5718WBjLAX2bmPyxMf/+xMDH8YyZDPwPDXwYGJkIaOXTNGdiUtHAqI2jA/18/GUQzGsg3gMfKg4FVQo6BiYcPqyF4XcChaczA4+DP8P//f4b/P3+SZgAzvxCDSGYjAyMjI8PvZw+AoYXdLuyiQLtE0uoZWAREwLb+fnKXQTipkngXcJu7MnACQx8G2FX1GHgs3bDGBlYX8HlFM/z9+JbhzewWhmf1CQyfti9j+PfzBwO/ZxTMTDiNmQKBfmZX1GB42V/K8P38YbDCX/dvMDAwMzPwuYbBNcIYmC4AhfjvXwx/376AqQHTf96+ZPj34xuKGIiDaQBQ8PPBTQwCoZkMjJzcYA3MgqIMAr7xDJ/3rAHzkQnGO7FWf5gZ/qLmBSZmBoHgNAZee1+Gf18/MzCyczJ83LyQ4fPetch6Gf4xMP3FbgBMGdAgJqAr/n37zABMTTBROA0ygAWUJUG5Civ4B8xwX78CpbD6FJiHmf4AAFicbTMTr5jAAAAAAElFTkSuQmCC".into(),
    ///    });
    /// ```
    pub fn add_attachment(&mut self, attachment: Attachment) -> &mut Self {
        self.content.attachments.push(attachment);
        self
    }
}

/// Message options for a particular Message
/// ```rust
/// use sparkpost::transmission::Options;
///
/// let options = Options {
///            open_tracking: false,
///            click_tracking: false,
///            transactional: false,
///            sandbox: false,
///            inline_css: false,
///        };
/// // or
/// let options2 = Options::default();
///
/// assert_eq!(options, options2);
/// ```
#[derive(Debug, Serialize, PartialEq, Default)]
pub struct Options {
    pub open_tracking: bool,
    pub click_tracking: bool,
    pub transactional: bool,
    pub sandbox: bool,
    pub inline_css: bool,
}

#[derive(Debug, Serialize, Default)]
pub struct Recipient {
    pub address: EmailAddress,

    /// holds option Json value
    pub substitution_data: Option<Value>,
}

/// Email address with name
///
/// ### Example
/// ```rust
/// use sparkpost::transmission::EmailAddress;
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

impl<'a> From<&'a str> for EmailAddress {
    fn from(email: &'a str) -> Self {
        EmailAddress {
            email: email.to_owned(),
            name: None,
        }
    }
}

impl From<String> for EmailAddress {
    fn from(email: String) -> Self {
        EmailAddress { email, name: None }
    }
}

/// Attachment data
#[derive(Debug, Serialize, Default)]
pub struct Attachment {
    /// Name of the file
    /// i.e. 'file_name.png'
    pub name: String,

    /// File mime type
    /// i.e. 'image/png'
    #[serde(rename = "type")]
    pub file_type: String,

    /// base64 encoded data
    pub data: String,
}

#[derive(Debug, Serialize, Default)]
pub struct Content {
    from: EmailAddress,
    subject: String,
    tags: Option<Vec<String>>,
    text: Option<String>,
    html: Option<String>,
    template_id: Option<String>,
    attachments: Vec<Attachment>,
}

#[test]
fn create_address() {
    let address: EmailAddress = "test@test.com".into();
    assert_eq!("test@test.com", address.email.as_str());
}

#[test]
fn create_message() {
    let mut email: Message = Message::new(EmailAddress::with_name("test@test.com", "name"));
    email.add_recipient("tech@hgill.io".into());

    let json_value: Value = to_value(email).unwrap();

    assert_eq!(
        "test@test.com",
        json_value["content"]["from"]["email"].as_str().unwrap()
    );
    assert_eq!(
        "name",
        json_value["content"]["from"]["name"].as_str().unwrap()
    );
    assert_eq!(
        "test@test.com",
        json_value["content"]["from"]["email"].as_str().unwrap()
    );
    assert!(!json_value["options"]["sandbox"].as_bool().unwrap());
    assert!(!json_value["options"]["click_tracking"].as_bool().unwrap());
    assert!(!json_value["options"]["open_tracking"].as_bool().unwrap());
    assert!(!json_value["options"]["transactional"].as_bool().unwrap());
}

#[test]
fn create_message_with_options() {
    let email: Message = Message::with_options(
        "test@test.com".into(),
        Options {
            open_tracking: true,
            click_tracking: true,
            transactional: true,
            sandbox: true,
            inline_css: false,
        },
    );
    let json_value = to_value(email).unwrap();

    assert_eq!(
        "test@test.com",
        json_value["content"]["from"]["email"].as_str().unwrap()
    );
    assert_eq!(
        "test@test.com",
        json_value["content"]["from"]["email"].as_str().unwrap()
    );
    assert!(json_value["options"]["sandbox"].as_bool().unwrap());
    assert!(json_value["options"]["click_tracking"].as_bool().unwrap());
    assert!(json_value["options"]["open_tracking"].as_bool().unwrap());
    assert!(json_value["options"]["transactional"].as_bool().unwrap());
    assert!(!json_value["options"]["inline_css"].as_bool().unwrap());
}

#[test]
fn create_message_with_substitute_data() {
    let mut email: Message = Message::default();

    #[derive(Debug, Serialize)]
    struct Substitute {
        pub any_field: String,
    }
    email.add_recipient_with_substitution_data(
        "name@domain.com".into(),
        Substitute {
            any_field: "any_value".into(),
        },
    );

    // let json_value = to_value(email).unwrap();

    // assert_eq!(
    //     "test@test.com",
    //     json_value["content"]["from"]["email"].as_str().unwrap()
    // );
    // assert_eq!(
    //     "test@test.com",
    //     json_value["content"]["from"]["email"].as_str().unwrap()
    // );
    // assert!(json_value["options"]["sandbox"].as_bool().unwrap());
    // assert!(json_value["options"]["click_tracking"].as_bool().unwrap());
    // assert!(json_value["options"]["open_tracking"].as_bool().unwrap());
    // assert!(json_value["options"]["transactional"].as_bool().unwrap());
    // assert!(!json_value["options"]["inline_css"].as_bool().unwrap());
}

#[test]
fn create_options() {
    let options = Options::default();
    assert_eq!(false, options.click_tracking);
    assert_eq!(false, options.open_tracking);
    assert_eq!(false, options.sandbox);
    assert_eq!(false, options.transactional);
}
