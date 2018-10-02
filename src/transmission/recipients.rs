use serde::ser::{SerializeSeq, SerializeStruct, Serializer};
use serde::Serialize;
use serde_json::Value;
use std::convert::From;

/// Email Recipient
#[derive(Debug, Serialize, Default, PartialEq)]
pub struct Recipient {
  pub address: EmailAddress,

  /// holds option Json value
  pub substitution_data: Option<Value>,
}

impl<'a> From<&'a str> for Recipient {
  fn from(email: &'a str) -> Self {
    Recipient {
      address: EmailAddress {
        email: email.to_owned(),
        name: None,
      },
      substitution_data: None,
    }
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

pub enum Recipients {
  LocalList(Vec<Recipient>),
  ListName(String),
}

impl Default for Recipients {
  fn default() -> Recipients {
    Recipients::LocalList(Vec::new())
  }
}

impl Serialize for Recipients {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match self {
      Recipients::LocalList(ref list) => {
        let mut seq = serializer.serialize_seq(Some(list.len()))?;
        for element in list {
          seq.serialize_element(element)?;
        }
        seq.end()
      }
      Recipients::ListName(ref list_name) => {
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
  // pub fn new(email: &str) -> Self {
  //   EmailAddress {
  //     email: email.to_owned(),
  //     name: None,
  //   }
  // }
  pub fn new(email: &str, name: &str) -> Self {
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
    Recipient {
      address: create_address(),
      substitution_data: Some(to_value(data).unwrap()),
    }
  }

  fn create_address() -> EmailAddress {
    EmailAddress {
      email: "test@test.com".into(),
      name: Some("Name".into()),
    }
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

    assert_eq!(recipient, create_recipient());
  }

  #[test]
  fn recipient_from_str() {
    let recipient: Recipient = "test@test.com".into();

    assert_eq!(recipient.address.email, create_recipient().address.email);
  }
}
