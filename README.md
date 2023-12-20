# Mailer Server
Mail Routing Server

Looking for the [frontend application](https://github.com/mailer/mailer-frontend)?
## Setup

Execute below command to install DB tables & seed default data
```shell
docker exec -i mailer-user-service sh app-setup.sh
```

Execute below command to refresh DB, re-install tables & seed data
```shell
docker exec -i mailer-user-service sh app-refresh-setup.sh
```

## Examples
- [Docker-Compose Example](/examples/basic)

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

## Todo
- Clear up temp files after certain interval
