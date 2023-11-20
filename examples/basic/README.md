# Mailer Server 
Basic Example - this example is tested with [MailDev](https://github.com/maildev/maildev) 
and can also be configured to work with any SMTP server

## Setup

Execute below command to install DB tables & seed default data
```shell
docker exec -i mailer-user-service sh app-setup.sh
```

Execute below command to refresh DB, re-install tables & seed data
```shell
docker exec -i mailer-user-service sh app-refresh-setup.sh
```
