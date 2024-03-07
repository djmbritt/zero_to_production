# zero_to_production

Repo to accompany the book Zero to Production.


## Deployment

Assuming your are using Digital Ocean as your deployment platform you can run the following commands to deploy this app:

```bash
# Make sure you are logged in
doctl auth init

# Create app with spec
doctl apps create --spec spec.yaml

# List your deployed apps
doctl app lsit

# Update your app
doctl apps update <DO_APP_ID> --spec=spec.yaml
```
