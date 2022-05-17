// Copyright Fauna, Inc.
// SPDX-License-Identifier: MIT-0

use fastly::Dictionary;
use fastly::http::{header, Method, StatusCode};
use fastly::{mime, Error, Request, Response};
use serde::{Deserialize, Serialize};

const BACKEND_NAME: &str = "fauna_gql";

#[fastly::main]
fn main(mut req: Request) -> Result<Response, Error> {

    match req.get_method() {
      // Allowed methods.
      &Method::GET | &Method::HEAD | &Method::POST | &Method::PUT | &Method::DELETE => (),

      // Deny anything else.
      _ => {
        return Ok(Response::from_status(StatusCode::METHOD_NOT_ALLOWED)
          .with_header(header::ALLOW, "GET, HEAD, POST, PUT, DELETE")
          .with_body_text_plain("This method is not allowed\n"))
      }
    };

    // Parse out the request
    let copy = req.clone_without_body();
    let path: Vec<&str> = copy.get_path().split("/").collect();
    let resource = path[1];
    let slug = if path.len() > 2 { path[2] } else { "" };
    let mut single_product_response = true;
    let mut response_code = StatusCode::OK;

    let gql = match (req.get_method(), resource, slug) {
      (&Method::GET, "", "") => {
        // Send a default synthetic response.
        return Ok(Response::from_status(StatusCode::OK)
          .with_content_type(mime::TEXT_HTML_UTF_8)
          .with_body(include_str!("welcome-to-compute@edge.html")))
      }

      // Create a product
      (&Method::POST, "product", "") => {
        response_code = StatusCode::CREATED;

        // Prepare the GraphQL query that includes a variable holding the request body
        let mut gql = r#"{
          "query": "mutation createProduct($product: ProductInput!) {
            product: createProduct(data: $product) {
              id: _id
              serialNumber
              title
              weightLbs
              quantity
            }
          }",
          "variables": {
            "product": $REQUEST_BODY
          }
        }"#.to_string();

        // Now populate the variable
        gql = gql.replace("$REQUEST_BODY", &req.into_body_str());
        gql
      }

      // get product by id
      (&Method::GET, "product", _) if slug != "" => {
        // The GraphQL query
        let mut gql = r#"{
          "query": "query FindProductById($id: ID!) {
            product: findProductByID(id: $id) {
              id: _id
              serialNumber
              title
              weightLbs
              quantity
            }
          }",
          "variables": {
            "id": "$PARAM_ID"
          } 
        }"#.to_string();

        // Populate the variable
        gql = gql.replace("$PARAM_ID", slug);
        gql
      }

      // Update a product
      (&Method::PUT, "product", _) if slug != "" => {
        // The GraphQL query
        let mut gql = r#"{
          "query": "mutation UpdateProductById(
            $id: ID!
            $product: ProductInput!
          ) {
            product: updateProduct(
              id: $id
              data: $product
            ) {
              id: _id
              serialNumber
              title
              weightLbs
              quantity            
            }
          }",
          "variables": {
            "id": "$PARAM_ID",
            "product": $REQUEST_BODY
          }        
        }"#.to_string();

        // Populate the variables
        gql = gql.replace("$PARAM_ID", slug);

        gql = gql.replace("$REQUEST_BODY", &req.into_body_str());
        gql
      }

      // Delete the product by id
      (&Method::DELETE, "product", _) if slug != "" => {
        response_code = StatusCode::NO_CONTENT;

        // The GraphQL query
        let mut gql = r#"{
          "query": "mutation DeleteProductById($id: ID!) {
            product: deleteProduct(id: $id) {
              id: _id
              serialNumber
              title
              weightLbs
              quantity            
            }
          }",
          "variables": {
            "id": "$PARAM_ID"
          }        
        }"#.to_string();

        // Populate the variable
        gql = gql.replace("$PARAM_ID", slug);
        gql
      }

      // get all products
      (&Method::GET, "product", "") => {
        single_product_response = false;

        r#"{
          "query": "query AllProducts {
            allProducts {
              data {
                id: _id
                serialNumber
                title
                weightLbs
                quantity
              }
            }
          }"
        }"#.to_string()
      }

      _ => {
        return Ok(Response::from_body("The page you requested could not be found")
          .with_status(StatusCode::NOT_FOUND))
      }
    };

    // Build the GraphQL request. Set the cache override to PASS
    let bereq = Request::post(get_fauna_url())
        .with_header("Authorization", format!("Bearer {}", get_api_key()))
        .with_header("Content-Type", "application/json")
        .with_body(gql)
        .with_pass(true);

    // Send the request to the backend
    let mut beresp = bereq.send(BACKEND_NAME)?;

    // Prepare the response
    let mut resp = Response::new()
        .with_status(response_code);

    if single_product_response {
      // Deserialize the response body into SingleProductResponse
      let api_response = beresp.take_body_json::<SingleProductResponse>()?;

      // Get the Product
      let product = api_response.data.product;

      match product {
        Some(_) => {
          resp.set_body_json(&product).unwrap();
        }
        None => {
          // Prepare the proper response if the query returned nothing
          resp.set_body_text_plain("Product Not Found");
          resp.set_status(StatusCode::NOT_FOUND);
        }
      };
    } else {
      // Get the response body and deserialize into an AllProductsResponse
      let api_response = beresp.take_body_json::<AllProductsResponse>()?;

      // Get the Products list from APIResponse
      let products = api_response.data.allProducts.data;

      resp.set_body_json(&products).unwrap();
    }

    Ok(resp)
}

// Struct representing Product
#[derive(Deserialize, Serialize)]
struct Product {
    id: String,
    serialNumber: String,
    title: String, 
    weightLbs: f64,
    quantity: i32
}

// Structs representing gql query response of a single product
#[derive(Deserialize)]
struct SingleProductResponse {
  data: DataProduct,
}
#[derive(Deserialize)]
struct DataProduct {
  product: Option<Product>,
}
// End: Structs representing gql query response of a single product

// Structs representing gql query response of a list of products
#[derive(Deserialize)]
struct AllProductsResponse {
    data: AllProducts,
}
#[derive(Deserialize)]
struct AllProducts {
    allProducts: AllProductsData,
}
#[derive(Deserialize, Serialize)]
struct AllProductsData {
  data: Vec<Product>
}
// End: Structs representing gql query response of a list of products

fn get_api_key() -> String {
  match Dictionary::open("fauna_env").get("key") {
      Some(key) => key,
      None => panic!("No Fauna API key!"),
  }
}

fn get_fauna_url() -> String {
  match Dictionary::open("fauna_env").get("url") {
      Some(url) => url,
      None => "https://graphql.fauna.com/graphql".to_string(),
  }
}