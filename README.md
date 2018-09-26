# sparkpost
## WIP initial release
version 0.0.1 only supports transmission api for sending emails.
as sparkpost has a huge api.

Please open an issue on [gitlab](https://gitlab.com/mygnu/spark_post/issues) if you need support or additional features

### Usage
```toml
[dependencies]
sparkpost = "0.0.3"

```


### Example
```rust
extern crate sparkpost;

use sparkpost::{Transmission, Message};

let tm = Transmission::new("api_key".to_string(), "https://api.eu.sparkpost.com/api/v1".to_string());
let mut email: Message = Message::new("sender@yourdomain.com", "Name");

email.add_recipient("name@domain.com", Some("Name"))
     .set_subject("My Awesome email 😎")
     .set_html("<h1>html body of the email</h1>")
     .set_text("text body of the email");

let result = tm.send(&email);

match result {
     Ok(res) => {
          match res.results {
              Some(result) => {
                 assert_eq!(1, result.total_accepted_recipients);
                 assert_eq!(0, result.total_rejected_recipients);
              }
              None => {
                 println!("api resonse: \n {:#?}", &res.errors.unwrap());
              }
          }
      }
      Err(error) => {
          println!("reqwest error \n {:#?}", error);
      }
}

```
### TODO
- [X] send email
- [ ] other api features

