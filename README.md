# Mailer Server
Mail Routing Server

Looking for the [frontend application](https://github.com/mailer/mailer-frontend)?

## Todo
- Clear up temp files after certain interval

## Payload Sample
```json
{
  "mails": [
    {
      "from": {
        "name": "SpiralOver",
        "email": "noreply@example.com"
      },
      "receiver": [
        {
          "name": "Super Admin",
          "email": "super.admin@example.com"
        },
        {
          "name": "Ahmad Mustapha",
          "email": "ahmad.mustapha@example.com"
        }
      ],
      "cc": [
        {
          "name": "Jane Doe",
          "email": "jane.doe@example.com"
        }
      ],
      "bcc": [],
      "reply_to": [],
      "subject": "Welcome to Spiralover",
      "message": "Hello Ahmard,<br/>Welcome to SpiralOver, we are glad to have you here."
    }
  ]
}
```
