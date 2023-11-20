# Mailer

SpiralOver mailer server

## Usage

Endpoint
> /api/v1/mail/send

Payload

```json
{
    "app": "spiralover",
    "mails": [
        {
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
            "cc": [],
            "bcc": [],
            "reply_to": [],
            "subject": "Welcome to Spiralover",
            "message": "Hello Ahmard,<br/>Welcome to SpiralOver, we are glad to have you here."
        }
    ]
}
```

### Spoofing Sender

Payload

```json
{
   "app": "spiralover",
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
         "cc": [],
         "bcc": [],
         "reply_to": [],
         "subject": "Welcome to Spiralover",
         "message": "Hello Ahmard,<br/>Welcome to SpiralOver, we are glad to have you here."
      }
   ]
}
```

### Encryption Options

- local (without authentication)
- basic
- startls | tls

Enjoy 😎
