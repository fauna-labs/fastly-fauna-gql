# About
*Generated from [Compute@Edge default starter kit for Rust](https://github.com/fastly/compute-starter-kit-rust-default)*

[![Deploy to Fastly](https://deploy.edgecompute.app/button)](https://deploy.edgecompute.app/deploy)

# Fauna Setup
* Signup for a Free account and create an access key using [these steps](https://docs.fauna.com/fauna/current/learn/quick_start/client_quick_start).
* ⚠️ Copy the key to a location where you can retrieve it for the next step, below.

# Local Quick Start
* Rename `env.json.sample` to `env.json`
* Edit in the file:
  ```
  {
    "url": "https://graphql.fauna.com/graphql",
    "key": "<secret>"
  }
  ```
  > Populate `key` with the value obtained in the previous step
  > 
  > Populate `url` with one of the following depending on the 
  > database [Region Group](https://docs.fauna.com/fauna/current/learn/understanding/region_groups):
  >  | Region Group | Value to enter |
  >  | ------------ | -------------- |
  >  | Classic      | https://graphql.fauna.com/graphql |
  >  | US           | https://graphql.us.fauna.com/graphql |
  >  | EU           | https://graphql.eufauna.com/graphql |

* Run locally:
  ```
  fastly compute serve
  ```

# Deploy
* Run `fastly compute publish` to create a new service:
  * When prompted for __backend__, enter one of the following depending on your Region Group
    | Region Group | Value to enter |
    | ------------ | -------------- |
    | Classic      | graphql.fauna.com |
    | US           | graphql.us.fauna.com |
    | EU           | graphql.eufauna.com |
  * When prompted for __Backend port number__ enter __443__
  * When prompted for __Backend name__ enter __fauna_gql__
  ```shell
  $ fastly compute publish                                  
  ✓ Initializing...
  ✓ Verifying package manifest...
  ✓ Verifying local rust toolchain...
  ✓ Building package using rust toolchain...
  ✓ Creating package archive...

  SUCCESS: Built package 'fastly-fauna-gql' (pkg/fastly-fauna-gql.tar.gz)


  There is no Fastly service associated with this package. To connect to an existing service
  add the Service ID to the fastly.toml file, otherwise follow the prompts to create a
  service now.

  Press ^C at any time to quit.

  Create new service: [y/N] y

  ✓ Initializing...
  ✓ Creating service...

  Domain: [properly-sweeping-mackerel.edgecompute.app] 

  Backend (hostname or IP address, or leave blank to stop adding backends): graphql.us.fauna.com
  Backend port number: [80] 443
  Backend name: [backend_1] fauna_gql

  Backend (hostname or IP address, or leave blank to stop adding backends): 

  ✓ Creating domain 'properly-sweeping-mackerel.edgecompute.app'...
  ✓ Creating backend 'fauna_gql' (host: graphql.us.fauna.com, port: 443)...
  ✓ Uploading package...
  ✓ Activating version...

  Manage this service at:
          https://manage.fastly.com/configure/services/3dkNIZu4EYHUmZy0gM89uA

  View this service at:
          https://properly-sweeping-mackerel.edgecompute.app


  SUCCESS: Deployed package (service 3dkNIZu4EYHUmZy0gM89uA, version 1)
  ```
* Add dictionary (note this creates a Draft version 2):
  ```shell
  $ fastly dictionary create --version=latest --name=fauna_env --autoclone                        

  SUCCESS: Created dictionary fauna_env (service 3dkNIZu4EYHUmZy0gM89uA version 2)
  ```
* Get dictionary id:
  ```shell
  $ fastly dictionary list --version=latest
  Service ID: 3dkNIZu4EYHUmZy0gM89uA
  Version: 2
  ID: 1ggKpT5AGcs55K3sX1126C
  Name: fauna_env
  Write Only: false
  Created (UTC): 2022-05-17 17:27
  Last edited (UTC): 2022-05-17 17:27  
  ```
* Create dictionary entry for `url` as per the `env.json` file (Note: use the `dictionary-id` from previous step)
  ```shell
  $ fastly dictionary-item create --dictionary-id=1ggKpT5AGcs55K3sX1126C --key=url --value=https://graphql.us.fauna.com/graphql

  SUCCESS: Created dictionary item key (service 3dkNIZu4EYHUmZy0gM89uA, dictionary 1ggKpT5AGcs55K3sX1126C)
  ```
* Create dictionary entry for `key` as per the `env.json` file
  ```shell
  $ fastly dictionary-item create --dictionary-id=1ggKpT5AGcs55K3sX1126C --key=key --value=some-$3cret-123

  SUCCESS: Created dictionary item key (service 3dkNIZu4EYHUmZy0gM89uA, dictionary 1ggKpT5AGcs55K3sX1126C)
  ```
* Activate the service
  ```shell
  $ fastly compute publish
  ✓ Initializing...
  ✓ Verifying package manifest...
  ✓ Verifying local rust toolchain...
  ✓ Building package using rust toolchain...
  ✓ Creating package archive...

  SUCCESS: Built package 'fastly-fauna-gql' (pkg/fastly-fauna-gql.tar.gz)


  ✓ Uploading package...
  ✓ Activating version...

  Manage this service at:
          https://manage.fastly.com/configure/services/3dkNIZu4EYHUmZy0gM89uA

  View this service at:
          https://properly-sweeping-mackerel.edgecompute.app


  SUCCESS: Deployed package (service 3dkNIZu4EYHUmZy0gM89uA, version 2)  
  ```