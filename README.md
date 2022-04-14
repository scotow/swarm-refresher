## swarm-refresher

### Usage

```sh
swarm-refresher COMPOSE_CONFIGS_DIRECTORY PROJECT_NAME...
swarm-refresher ~/docker-projects dropit rusdirect 
````

### Using [webhook](https://github.com/adnanh/webhook)...

... to refresh your stacks after a GitHub Actions Docker build.  

```json
[
  {
    "id": "refresh-stack",
    "execute-command": "/usr/bin/swarm-refresher",
    "pass-arguments-to-command": [
      {
        "source": "string",
        "name": "/home/me/docker-projects"
      },
      {
        "source": "payload",
        "name": "package.name"
      }
    ],
    "trigger-rule": {
      "and": [
        {
          "match": {
            "type": "payload-hmac-sha256",
            "secret": "GITHUB_WEBHOOK_SECRET",
            "parameter": {
              "source": "header",
              "name": "X-Hub-Signature-256"
            }
          }
        },
        {
          "match": {
            "type": "value",
            "value": "published",
            "parameter": {
              "source": "payload",
              "name": "action"
            }
          }
        }
      ]
    }
  }
]
```