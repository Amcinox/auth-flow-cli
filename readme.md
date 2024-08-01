# cognito-cli-auth 

A command-line interface for user authentication with AWS Cognito.

**Features:**

* Supports multiple Cognito projects
* Allows selection or manual entry of credentials
* Handles default accounts defined in environment variables

**Installation:**

```bash
# Install dependencies (assuming Rust is already installed)
cargo install --path .
```

**Usage:**

```bash
cognito-cli-auth
```

**Environment Variables in `.env` (Example):**

```
# Project 1 Configuration
PROJECT_1_REGION=us-east-1
PROJECT_1_POOL_ID=your-cognito-pool-id-1
PROJECT_1_CLIENT_ID=your-cognito-client-id-1
PROJECT_1_DEFAULT_ACCOUNTS=[{"username":"user1", "password":"password1"}, {"username":"user2", "password":"password2"}]

# Project 2 Configuration (Optional)
PROJECT_2_REGION=us-west-2
PROJECT_2_POOL_ID=your-cognito-pool-id-2
PROJECT_2_CLIENT_ID=your-cognito-client-id-2
# ... (Optional default accounts for project 2)
```

**Explanation of Environment Variables:**

* `PROJECT_X_REGION`: AWS region where your Cognito user pool is located.
* `PROJECT_X_POOL_ID`: ID of your Cognito user pool for project X.
* `PROJECT_X_CLIENT_ID`: Client ID of your Cognito app for project X.
* `PROJECT_X_DEFAULT_ACCOUNTS` (Optional): A JSON array containing default username and password objects for project X.

**Contributing:**

Feel free to contribute to this project!

**License:**

[Specify license, e.g., MIT]
