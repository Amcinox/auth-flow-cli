
# Auth Flow CLI


Auth Flow CLI is a powerful command-line interface tool for user authentication with AWS Cognito. It provides a seamless way to manage multiple Cognito projects, handle user credentials, and perform authentication tasks efficiently.

## Features

- Multi-Project Support: Easily manage and switch between multiple AWS Cognito projects.
- Flexible Authentication: Choose between default accounts or manually enter credentials.
- Environment-Based Configuration: Configure projects using environment variables for enhanced security and flexibility.
- Cross-Platform Compatibility: Available for Linux, macOS, and Windows.

## Installation

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Steps
1. Clone the repository:
```bash 
git clone https://github.com/Amcinox/auth-flow-cli.git
cd auth-flow-cli 
```

2. Build the project:
```bash 
cargo build --release 
```


3. The binary will be available in target/release/auth-flow-cli

### Alternative: Install from GitHub Releases

You can download pre-built binaries for your platform from the GitHub Releases page.

 ## Usage

Run the CLI tool:

```bash
./auth-flow-cli 
```

Follow the interactive prompts to select your project and authenticate.

## Configuration

Auth Flow CLI uses environment variables for configuration. Create a .env file in the project root with the following structure:

```env
# Project 1 Configuration 
PROJECT_1_REGION=us-east-1
PROJECT_1_POOL_ID=your-cognito-pool-id-1
PROJECT_1_CLIENT_ID=your-cognito-client-id-1
PROJECT_1_DEFAULT_ACCOUNTS=[{"username":"user1", "password":"password1"}, {"username":"user2", "password":"password2"}]
# Project 2 Configuration
PROJECT_2_REGION=us-west-2
PROJECT_2_POOL_ID=your-cognito-pool-id-2
PROJECT_2_CLIENT_ID=your-cognito-client-id-2
# ... (Optional default accounts for project 2)
```


### Environment Variables Explained

- PROJECT_X_REGION: AWS region where your Cognito user pool is located.
- PROJECT_X_POOL_ID: ID of your Cognito user pool for project X.
- PROJECT_X_CLIENT_ID: Client ID of your Cognito app for project X.
- PROJECT_X_DEFAULT_ACCOUNTS (Optional): A JSON array containing default username and password objects for project X.

## Development

### Requirements

- Rust 1.51.0 or later
- Cargo (latest version)

### Building

```bash
cargo build 
```

### Running Tests
```bash
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (git checkout -b feature/AmazingFeature)
3. Commit your changes (git commit -m 'Add some AmazingFeature')
4. Push to the branch (git push origin feature/AmazingFeature)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Author

MOHAMED EL BSSIR

- Website: www.elbssir.com
- Email: contact@elbssir.com

## Acknowledgments

- Thanks to all contributors who have helped shape Auth Flow CLI.
- Special thanks to the Rust community for their excellent documentation and support.
