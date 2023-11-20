# Mailer Backend
Mailer Backend Application

Looking for the [frontend application](https://github.com/mailer/mailer-frontend)?

## Todo
- Clear up temp files after certain interval

## Payload Sample
```json
{
  "callback_on_success": false,
  "callback_on_failure": true,
  "reference": "{{$randomUUID}}",
  "endpoint": "https://mailer.spiralover.com/api/v1/mail/send",
  "callback": "https://hrms.spiralover.com/backend/callbacks/mailer",
  "impulse_name": "wallet.credit",
  "impulse_data": {
    "action": "credit",
    "amount": 1000,
    "narration": "Ahmard sent 1,000 Naira to your wallet"
  }
}
```
