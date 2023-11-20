# Mailer Backend
Mailer Backend Application

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
        "email": "noreply@spiralover.com"
      },
      "receiver": [
        {
          "name": "Ahmard",
          "email": "super.admin@spiralover.com"
        },
        {
          "name": "Ahmad Mustapha",
          "email": "ahmard.mu@gmail.com"
        }
      ],
      "cc": [
        {
          "name": "Ahmad Mustapha",
          "email": "ahmard.mu@gmail.com"
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
