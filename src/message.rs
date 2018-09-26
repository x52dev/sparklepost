use std::convert::From;


/// Represents email message including some mata-data
/// ### Example
/// ```rust
///
/// 
/// use sparkpost::{Message, EmailAddress};
/// 
/// let mut email = Message::new(EmailAddress::with_name("marketing@example.sink.sparkpostmail.com", "Example Company"));
/// email.add_recipient("wilma@example.sink.sparkpostmail.com".into())
///        .set_campaign_id("postman_inline_both_example")
///        .set_subject("SparkPost inline template example")
///        .set_html("<html><body>Here is your inline html, {{first_name or 'you great person'}}!<br></body></html>")
///        .set_text("Here is your plain text, {{first_name or 'you great person'}}!");
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
    /// this method can be called multiple times
    /// WARNING: it does not check for duplicates for now
    pub fn add_recipient(&mut self, address: EmailAddress) -> &mut Self {
        self.recipients.push(Recipient {
            address,
        });
        self
    }

    pub fn set_subject(&mut self, subject: &str) -> &mut Self {
        self.content.subject = subject.to_owned();
        self
    }
    pub fn set_options(&mut self, options: Options) -> &mut Self {
        self.options = options;
        self
    }
    pub fn set_html(&mut self, html: &str) -> &mut Self {
        self.content.html = Some(html.to_owned());
        self
    }
    pub fn set_text(&mut self, text: &str) -> &mut Self {
        self.content.text = Some(text.to_owned());
        self
    }
    pub fn set_campaign_id(&mut self, campaign_id: &str) -> &mut Self {
        self.campaign_id = Some(campaign_id.to_owned());
        self
    }
}

/// Message options for a particular Message
/// ```rust
/// use sparkpost::Options;
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
    address: EmailAddress,
}


/// Email address with name
///
/// ### Example
/// ```rust
/// use sparkpost::EmailAddress;
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

#[cfg(test)]
use serde_json::{to_value, Value};

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

    assert_eq!("test@test.com", json_value["content"]["from"]["email"].as_str().unwrap());
    assert_eq!("name", json_value["content"]["from"]["name"].as_str().unwrap());
    assert_eq!("test@test.com", json_value["content"]["from"]["email"].as_str().unwrap());
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

    assert_eq!("test@test.com", json_value["content"]["from"]["email"].as_str().unwrap());
    assert_eq!("test@test.com", json_value["content"]["from"]["email"].as_str().unwrap());
    assert!(json_value["options"]["sandbox"].as_bool().unwrap());
    assert!(json_value["options"]["click_tracking"].as_bool().unwrap());
    assert!(json_value["options"]["open_tracking"].as_bool().unwrap());
    assert!(json_value["options"]["transactional"].as_bool().unwrap());
    assert!(!json_value["options"]["inline_css"].as_bool().unwrap());
}

#[test]
fn create_options() {
    let options = Options::default();
    assert_eq!(false, options.click_tracking);
    assert_eq!(false, options.open_tracking);
    assert_eq!(false, options.sandbox);
    assert_eq!(false, options.transactional);
}

