use serde::{
    ser::{SerializeSeq, SerializeStruct, Serializer},
    Serialize,
};
use serde_json::{to_value, Value};
use std::convert::From;

/// Email Recipient
/// Example
/// ```rust
/// extern crate sparkpost;
///
/// use sparkpost::transmission::Recipient;
///
/// let recipient = Recipient::from("test@test.com");
///  ```
#[derive(Debug, Serialize, Default, PartialEq)]
pub struct Recipient {
    pub(crate) address: EmailAddress,
    pub(crate) substitution_data: Option<Value>,
}

impl Recipient {
    /// create recipient with substitute data for any type that implements Serialize from serde
    pub fn with_substitution<T: Serialize>(
        address: EmailAddress,
        data: T,
    ) -> Self {
        Recipient {
            address,
            substitution_data: Some(
                to_value(data).expect("unable to serialize data"),
            ),
        }
    }
}

impl<'a> From<&'a str> for Recipient {
    fn from(email: &'a str) -> Self {
        email.to_owned().into()
    }
}

impl From<String> for Recipient {
    fn from(email: String) -> Self {
        Recipient {
            address: EmailAddress { email, name: None },
            substitution_data: None,
        }
    }
}

impl From<EmailAddress> for Recipient {
    fn from(address: EmailAddress) -> Self {
        Recipient {
            address,
            substitution_data: None,
        }
    }
}

#[derive(Debug)]
pub enum RecipientSet {
    LocalList(Vec<Recipient>),
    ListName(String),
}

impl Default for RecipientSet {
    fn default() -> RecipientSet {
        RecipientSet::LocalList(Vec::new())
    }
}

impl Serialize for RecipientSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            RecipientSet::LocalList(ref list) => {
                let mut seq = serializer.serialize_seq(Some(list.len()))?;
                for element in list {
                    seq.serialize_element(element)?;
                }
                seq.end()
            }
            RecipientSet::ListName(ref list_name) => {
                let mut s = serializer.serialize_struct("listname", 1)?;
                s.serialize_field("list_id", list_name)?;
                s.end()
            }
        }
    }
}

/// Email address with optional name
///
/// ### Example
/// ```rust
/// use sparkpost::transmission::EmailAddress;
///
/// let address = EmailAddress::from("test@test.com");
///
/// // create address with name
/// let address = EmailAddress::new("test@test.com", "Name");
///
///```
#[derive(Debug, Serialize, Default, PartialEq)]
pub struct EmailAddress {
    pub(crate) email: String,
    pub(crate) name: Option<String>,
}

impl EmailAddress {
    /// create new email address with email and name
    pub fn new<E: Into<String>, N: Into<String>>(email: E, name: N) -> Self {
        EmailAddress {
            email: email.into(),
            name: Some(name.into()),
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

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::to_value;

    #[derive(Debug, Serialize)]
    struct Data {
        name: String,
    }

    fn create_recipient() -> Recipient {
        let data = Data {
            name: "Name".to_owned(),
        };
        Recipient::with_substitution(create_address(), data)
    }

    fn create_address() -> EmailAddress {
        EmailAddress {
            email: "test@test.com".into(),
            name: Some("Name".into()),
        }
    }

    #[test]
    fn address_with_new() {
        let address = EmailAddress::new("test@test.com", "Name".to_string());
        assert_eq!(address, create_address());

        let address = EmailAddress::new("test@test.com".to_string(), "Name");
        assert_eq!(address, create_address());

        let address = EmailAddress::new("test@test.com", "Name");
        assert_eq!(address, create_address());
    }
    #[test]
    fn address() {
        let address = EmailAddress::from("test@test.com".to_string());

        assert_eq!("test@test.com", address.email.as_str());

        let address = EmailAddress {
            email: "test@test.com".into(),
            name: Some("Name".into()),
        };

        assert_eq!(address, create_address())
    }

    #[test]
    fn address_from_str() {
        let address1: EmailAddress = "test@test.com".into();
        let address2: EmailAddress = create_address();
        assert_eq!(address1.email, address2.email);
    }

    #[test]
    fn address_from_string() {
        let address1: EmailAddress = String::from("test@test.com").into();
        let address2: EmailAddress = create_address();
        assert_eq!(address1.email, address2.email);
    }

    #[test]
    fn address_with_name() {
        let address = EmailAddress::new("test@test.com", "Name");
        assert_eq!(address, create_address());
    }

    #[test]
    fn recipient() {
        let data = Data {
            name: "Name".to_owned(),
        };

        let recipient = Recipient {
            address: create_address(),
            substitution_data: Some(to_value(data).unwrap()),
        };
        let string_value = "{\"address\":{\"email\":\"test@test.com\",\"name\":\"Name\"},\"substitution_data\":{\"name\":\"Name\"}}".to_owned();
        assert_eq!(string_value, to_value(&recipient).unwrap().to_string());
        assert_eq!(recipient, create_recipient());
    }

    #[test]
    fn recipient_from_str() {
        let recipient: Recipient = "test@test.com".into();

        assert_eq!(recipient.address.email, create_recipient().address.email);
    }
}
