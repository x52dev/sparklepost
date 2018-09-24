# spark_post
## WIP initial release
version 0.0.1 only supports transmission api for sending emails.
as sparkpost has a huge api, please open an issue on gitlab if you need support.

## Example
```rust
extern crate spark_post;

use spark_post::{Transmission, Message, MessageBuilder};

let tm = Transmission::new("sparkpost_api_key");
let email: Message = MessageBuilder::new("sender@yourdomain.com", "Name")
         .add_recipient("name@domain.com", Some("Name"))
         .set_subject("My Awesome email 😎")
         .set_html("<h1>html body of the email</h1>")
         .set_text("text body of the email")
         .finish();
let result = tm.send(&email);

match result {
    OK(response)=>{
        println!("{:#?}", response);
    },
    Err(e) => println("{:#?}", e);
}

```
