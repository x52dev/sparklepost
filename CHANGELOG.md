# Changelog

## 0.5.5

- Forked from `sparkpost` crate.

## 0.5.1

### `Email::new()` now can take `String` and `&str`

## 0.5.0

### Breaking Changes

Ergonomic API, In most places you can now provide a `String` or `str`.

```rust
// email.add_recipient("wilma@example.sink.sparkpostmail.com".into())
// becomes
    email.add_recipient("wilma@example.sink.sparkpostmail.com");
    let body :String = String::form("...");
    email.html(&body) // gets copied internally
    email.html(body) // body moves ownership, no allocation
```

`Attachment` struct can only be created by `Attachment::from_data(...)`.
Attachment can be passed as `&Attachment` or by moving to the massage.

## 0.4.1

### No Breaking changes

## 0.4.0

### Full Example in example dir

### Breaking Changes

- added method to set campaign_id for message
- Transmission initialization now has both global and EU options. No need to provide Url
- Message contents are public only to crate
- Message options now take chrono Utf DateTime as start_time value
- Message can have both substitution data and metadata

## 0.3.0

### Breaking Changes

- message construction and emailAddress construction changed
- added support for using stored recipients list from the api
- Improved tests and documentation

## 0.2.1

- fix broken readme
- move example to docs

## 0.2.0

### Breaking Changes

- Message method names changed
- Message initial support for file attachments
- transmission released as module
- `TransmissionResponse` is now an `Enum`
- Expose more data types as `pub`

## 0.1.1

- fixed readme typo

## 0.1.0

### Breaking Changes

- Options now all defaults to false
- Transaction.send() method signature changed it now return
  a `struct TransmissionResponse` in the result
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

## 0.0.3

- better documentation
- moved to rust stable

## 0.0.1 and 0.0.2

Initial release, experimental only works on nightly
