use chrono::prelude::*;
use serde::ser::Serialize;
use serde_json::{to_value, Value};

use super::models::*;

/// Represents email message including some mata-data
/// ### Example
/// ```rust
///
/// use sparkpost::transmission::{Message, EmailAddress};
///
/// let mut email = Message::new(EmailAddress::new("marketing@example.sink.sparkpostmail.com", "Example Company"));
/// email.add_recipient("wilma@example.sink.sparkpostmail.com")
///        .campaign_id("postman_inline_both_example")
///        .subject("SparkPost inline template example")
///        .html("<html><body>Here is your inline html, {{first_name or 'you great person'}}!<br></body></html>")
///        .text("Here is your plain text, {{first_name or 'you great person'}}!");
/// ```
/// deserialized json looks similar to this
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
    pub description: Option<String>,
    pub campaign_id: Option<String>,
    pub metadata: Option<Value>,
    pub substitution_data: Option<Value>,
    pub recipients: Recipients,
    pub(crate) content: Content,
}

impl Message {
    /// create new message with sender emailAddress
    pub fn new<T: Into<EmailAddress>>(sender_address: T) -> Self {
        let mut message = Message::default();
        message.content.from = sender_address.into();
        message
    }

    /// create new message with sending options
    pub fn with_options(
        sender_address: EmailAddress,
        options: Options,
    ) -> Self {
        let mut message = Message::default();
        message.options = options;
        message.content.from = sender_address;
        message
    }

    /// set an recipient list stored in the api
    /// replaces local recipients variable with a Sparkpost API list id
    ///
    /// see [Transport API ref](https://developers.sparkpost.com/api/transmissions/#header-stored-recipient-list)
    pub fn recipient_list(&mut self, list_name: &str) -> &mut Self {
        self.recipients = Recipients::ListName(list_name.into());
        self
    }

    /// add an address to recipient list
    ///
    /// Recipient is replaced if they have same email address
    pub fn add_recipient<T: Into<Recipient>>(
        &mut self,
        recipient: T,
    ) -> &mut Self {
        let recipient: Recipient = recipient.into();
        match self.recipients {
            Recipients::ListName(_) => {
                self.recipients = Recipients::LocalList(vec![recipient])
            }
            Recipients::LocalList(ref mut list) => {
                list.retain(|ref rec| {
                    rec.address.email.as_str()
                        != recipient.address.email.as_str()
                });
                list.push(recipient);
            }
        }

        self
    }

    /// set message subject
    pub fn subject<T: Into<String>>(&mut self, subject: T) -> &mut Self {
        self.content.subject = subject.into();
        self
    }
    /// set message options
    pub fn options<T: Into<Options>>(&mut self, options: T) -> &mut Self {
        self.options = options.into();
        self
    }
    /// set content html
    pub fn html<T: Into<String>>(&mut self, html: T) -> &mut Self {
        self.content.html = Some(html.into());
        self
    }
    /// set content text
    pub fn text<T: Into<String>>(&mut self, text: T) -> &mut Self {
        self.content.text = Some(text.into());
        self
    }
    /// set campaign id
    pub fn campaign_id<T: Into<String>>(
        &mut self,
        campaign_id: T,
    ) -> &mut Self {
        self.campaign_id = Some(campaign_id.into());
        self
    }
    /// set template id for content
    pub fn template_id<T: Into<String>>(
        &mut self,
        template_id: T,
    ) -> &mut Self {
        self.content.template_id = Some(template_id.into());
        self
    }

    /// set substitution_dat
    pub fn substitution_data<T: Serialize>(&mut self, data: T) -> &mut Self {
        self.substitution_data =
            Some(to_value(data).expect("Data cannot be searized"));
        self
    }

    /// set metadata
    pub fn metadata<T: Serialize>(&mut self, data: T) -> &mut Self {
        self.metadata = Some(to_value(data).expect("Data cannot be searized"));
        self
    }

    /// adds attachment to Message, multiple attachments allowed
    /// ``` rust
    /// use sparkpost::transmission::{Message, Attachment};
    ///
    /// let mut email = Message::new("marketing@example.sink.sparkpostmail.com");
    /// let attachment = Attachment::from_data(
    ///        "image.png",
    ///        "image/png", "iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAAAXNSR0IArs4c6QAAAAlwSFlzAAAWJQAAFiUBSVIk8AAAAXxJREFUOBFjvJVg84P5718WBjLAX2bmPyxMf/+xMDH8YyZDPwPDXwYGJkIaOXTNGdiUtHAqI2jA/18/GUQzGsg3gMfKg4FVQo6BiYcPqyF4XcChaczA4+DP8P//f4b/P3+SZgAzvxCDSGYjAyMjI8PvZw+AoYXdLuyiQLtE0uoZWAREwLb+fnKXQTipkngXcJu7MnACQx8G2FX1GHgs3bDGBlYX8HlFM/z9+JbhzewWhmf1CQyfti9j+PfzBwO/ZxTMTDiNmQKBfmZX1GB42V/K8P38YbDCX/dvMDAwMzPwuYbBNcIYmC4AhfjvXwx/376AqQHTf96+ZPj34xuKGIiDaQBQ8PPBTQwCoZkMjJzcYA3MgqIMAr7xDJ/3rAHzkQnGO7FWf5gZ/qLmBSZmBoHgNAZee1+Gf18/MzCyczJ83LyQ4fPetch6Gf4xMP3FbgBMGdAgJqAr/n37zABMTTBROA0ygAWUJUG5Civ4B8xwX78CpbD6FJiHmf4AAFicbTMTr5jAAAAAAElFTkSuQmCC");
    ///
    /// email.add_recipient("wilma@example.sink.sparkpostmail.com")
    ///     .add_attachment(attachment);
    /// ```
    pub fn add_attachment<T: Into<Attachment>>(
        &mut self,
        attachment: T,
    ) -> &mut Self {
        self.content.attachments.push(attachment.into());
        self
    }
}

/// Message options for a particular Message
/// ```rust
/// # extern crate chrono;
/// # extern crate sparkpost;
/// # fn main() {
/// use chrono::prelude::*;
/// use sparkpost::transmission::Options;
///
/// let options = Options {
///            open_tracking: false,
///            click_tracking: false,
///            transactional: false,
///            sandbox: false,
///            inline_css: false,
///            start_time: Some(Utc.ymd(2014, 7, 8).and_hms(9, 10, 11))
///        };
/// // or
/// let options2 = Options::default();
///
/// # }
///  ```
#[derive(Debug, Serialize, PartialEq, Default)]
pub struct Options {
    pub open_tracking: bool,
    pub click_tracking: bool,
    pub transactional: bool,
    pub sandbox: bool,
    pub inline_css: bool,
    pub start_time: Option<DateTime<Utc>>,
}

/// Attachment data
#[derive(Debug, Serialize, Default)]
pub struct Attachment {
    /// Name of the file
    /// i.e. 'file_name.png'
    name: String,

    /// File mime type
    /// i.e. 'image/png'
    #[serde(rename = "type")]
    file_type: String,

    /// base64 encoded data
    data: String,
}

impl<'a> From<&'a Attachment> for Attachment {
    fn from(attachment: &'a Attachment) -> Self {
        Attachment {
            name: attachment.name.to_owned(),
            file_type: attachment.file_type.to_owned(),
            data: attachment.data.to_owned(),
        }
    }
}

impl Attachment {
    pub fn from_data<T: Into<String>>(name: T, file_type: T, data: T) -> Self {
        Attachment {
            name: name.into(),
            file_type: file_type.into(),
            data: data.into(),
        }
    }
}

/// Email contents
#[derive(Debug, Serialize, Default)]
pub(crate) struct Content {
    pub from: EmailAddress,
    pub subject: String,
    pub tags: Option<Vec<String>>,
    pub text: Option<String>,
    pub html: Option<String>,
    pub template_id: Option<String>,
    pub attachments: Vec<Attachment>,
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::to_value;

    #[derive(Debug, Serialize)]
    struct Substitute {
        pub any_field: String,
    }

    #[test]
    fn create_message() {
        let mut email: Message =
            Message::new(EmailAddress::new("test@test.com", "name"));
        email.add_recipient("tech@hgill.io");
        email.recipient_list("my_list");

        let json_value: Value = to_value(&email).unwrap();

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
        // println!("{:#?}", json_value);
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
                start_time: Some(Utc.ymd(2014, 7, 8).and_hms(9, 10, 11)),
            },
        );
        let json_value = to_value(email).unwrap();
        // println!("{:?}", &json_value);
        assert_eq!(
            "test@test.com",
            json_value["content"]["from"]["email"].as_str().unwrap()
        );
        assert_eq!("test@test.com", json_value["content"]["from"]["email"]);
        assert!(json_value["options"]["sandbox"].as_bool().unwrap());
        assert!(json_value["options"]["click_tracking"].as_bool().unwrap());
        assert!(json_value["options"]["open_tracking"].as_bool().unwrap());
        assert!(json_value["options"]["transactional"].as_bool().unwrap());
        assert!(!json_value["options"]["inline_css"].as_bool().unwrap());
        assert_eq!("2014-07-08T09:10:11Z", json_value["options"]["start_time"]);
    }

    #[test]
    fn create_message_with_substitute_data() {
        let mut email: Message = Message::default();

        let data = Substitute {
            any_field: "any_value".into(),
        };

        email.add_recipient(Recipient {
            address: "name@domain.com".into(),
            substitution_data: Some(to_value(data).unwrap()),
        });
        let json_value = to_value(email).unwrap();
        // println!("{:#?}", &json_value);

        assert_eq!(
            json_value["recipients"][0]["address"]["email"],
            "name@domain.com"
        );
    }

    #[test]
    fn test_message_recipient_duplication() {
        let mut message = Message::default();
        let recipient: Recipient = "email@domain.com".into();
        let recipient1: Recipient = "email@domain.com".into();
        message.add_recipient(recipient);

        // println!("{:#?}", &message);
        match message.recipients {
            Recipients::LocalList(ref list) => {
                assert_eq!(list.get(0), Some(&recipient1));
            }
            _ => assert!(false),
        };

        message.add_recipient(Recipient {
            address: "email@domain.com".into(),
            substitution_data: Some(
                to_value(Substitute {
                    any_field: "any_value".into(),
                })
                .unwrap(),
            ),
        });

        match message.recipients {
            Recipients::LocalList(ref list) => {
                assert_eq!(list.len(), 1);
            }
            _ => assert!(false),
        };

        let json_value = to_value(&message).unwrap();
        // println!("{:#?}", &json_value);

        assert_eq!(
            json_value["recipients"][0]["substitution_data"]["any_field"],
            "any_value"
        );

        message.recipient_list("mylist");

        let json_value = to_value(&message).unwrap();
        // println!("{:#?}", &json_value);

        assert_eq!(json_value["recipients"]["list_id"], "mylist");
    }

    #[test]
    fn create_options() {
        let options = Options::default();
        assert_eq!(false, options.click_tracking);
        assert_eq!(false, options.open_tracking);
        assert_eq!(false, options.sandbox);
        assert_eq!(false, options.transactional);
    }
}
