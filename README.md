# Web API Routes Documentation

This repository contains Rust code that defines a web API using the Rocket framework. The API provides various routes for account and user management, interacting with a database to perform different operations. Below are the details of the available routes and their functionalities.

## Table of Contents

- [Getting Started](#getting-started)
- [Routes](#routes)
  - [Account Routes](#account-routes)
  - [User Routes](#user-routes)
- [Contributing](#contributing)
- [License](#license)

## Getting Started

To run this web API on your local machine, follow these steps:

1. Clone this repository.
2. Install Rust and Cargo if you haven't already.
3. Navigate to the project directory.
4. Run the following command to start the web server:

   ```shell
   cargo run


1. The API will be accessible at http://localhost:8000.



### Account Routes

#### POST /

Create an account.

**Request:**

```rust
POST /
{
  // Account data
}




Response:

Success: Account creation status.
Error: Custom error message.
POST /deposit
Initialize a deposit for an account.

Request:

rust
Copy code
POST /deposit
{
  // Deposit account data
}
Response:

Success: Deposit initialization status.
Error: Custom error message.
GET /deposit
Get deposit information for an account.

Response:

Success: Deposit information.
Error: Custom error message.
POST /withdraw
Initialize a withdrawal for an account.

Request:

rust
Copy code
POST /withdraw
{
  // Withdrawal account data
}
Response:

Success: Withdrawal initialization status.
Error: Custom error message.
POST /transfer_funds
Transfer funds between accounts.

Request:

rust
Copy code
POST /transfer_funds
{
  // Transfer data
}
Response:

Success: Transfer status.
Error: Custom error message.
GET /dashboard
Get dashboard information for an account.

Response:

Success: Dashboard information.
Error: Custom error message.
GET /transactions
Get transactions based on query parameters.

Request:

rust
Copy code
GET /transactions?currency=&transaction_id=&account_id=&limit=&page=
Response:

Success: Transactions information.
Error: Custom error message.
POST /callback/<provider>
Handle payment webhooks.

Request:

rust
Copy code
POST /callback/<provider>
{
  // Payment event data
}
Response:

Success: Empty response.
Error: Custom error message.
User Routes
POST /sign_up
Sign up a new user.

Request:

rust
Copy code
POST /sign_up
{
  // User sign-up data
}
Response:

Success: User sign-up status.
Error: Custom error message.
POST /sign_in
Sign in a user.

Request:

rust
Copy code
POST /sign_in
{
  // User login data
}
Response:

Success: User login status.
Error: Custom error message.
GET /me
Get the profile of the authenticated user.

Response:

Success: User profile information.
Error: Custom error message.
Contributing
Contributions are welcome! If you find any issues or want to add new features, feel free to open a pull request.

License
This project is licensed under the MIT License.