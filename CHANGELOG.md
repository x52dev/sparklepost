## Version 0.1.0
### Breaking changes
- Options now all defaults to false
- Transaction.send() method signature changed it now return 
a ```struct TransmissionResponse```   in the result
  ```rust
  send(&self, message: &Message) -> 
      Result<TransmissionResponse, ReqError> {
          ...
      }
  ```
- remove Message::json() method
- Message methods are now non consuming
    ##### Example
    ```rust
        let mut email = Message::new(EmailAddress::with_name("marketing@example.sink.sparkpostmail.com", "Example Company"));
        email.add_recipient("wilma@example.sink.sparkpostmail.com".into())
          .set_campaign_id("postman_inline_both_example")
          .set_subject("SparkPost inline template example")
          .set_html("<html><body>Here is your inline html, {{first_name or 'you great person'}}!<br></body></html>")
          .set_text("Here is your plain text, {{first_name or 'you great person'}}!");
    ```
## Version 0.0.3
- better documentation
- moved to rust stable

## Version 0.0.1 and 0.0.2
Initial release, experimental only works on nightly
