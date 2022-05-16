# Compute@Edge default starter kit for Rust

[![Deploy to Fastly](https://deploy.edgecompute.app/button)](https://deploy.edgecompute.app/deploy)

# Fauna Set
* Signup for a Free account and create an access key using [these steps](https://docs.fauna.com/fauna/current/learn/quick_start/client_quick_start).
* ⚠️ Copy the key to a location where you can retrieve it for the next step, below.

# Project Quick Start
* Rename `env.json.sample` to `env.json`
* Edit in the file:
  ```
  {
    "url": "https://graphql.fauna.com/graphql",
    "key": "<>"
  }
  ```
  > Populate `key` with the value obtained in the previous step
  > 
  > Populate `url` with one of the following depending on the 
  > database [Region Group](https://docs.fauna.com/fauna/current/learn/understanding/region_groups).
  > * `https://graphql.fauna.com/graphql`
  > * `https://graphql.us.fauna.com/graphql`
  > * `https://graphql.eu.fauna.com/graphql` 
 
* Run locally:
  ```
  fastly compute serve
  ```
* Deploy:
  ```
  fastly compute publish
  ```