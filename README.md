# gmailer-rust
A rust implementation to send mails using the Gmail SMTP

## Requirements

- `env.sh` where you define variables: APP_USER and APP_PASS (SMTP credentials)
- `mail.tmpl` with the template of the mailbody and a {CODE} placeholder
- `mail_code.csv` with the following structure (leave header row):
```
email_address,code
example@example.com,ABCDEF
```

## Usage

1. `source ./env.sh` to get the APP_USER and APP_PASS environment variables active.
1. setup `mail.tmpl` and `mail_code.csv`
1. Run the program `cargo run`