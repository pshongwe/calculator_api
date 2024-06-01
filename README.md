# Calculator API

This Calculator API is built using Rust with the Actix-web framework. It supports basic arithmetic operations: addition, subtraction, multiplication, and division.

## Prerequisites

To run this project, you will need Rust installed on your machine. You can install Rust from [rust-lang.org](https://www.rust-lang.org/tools/install).

## Installation

### Getting Started

1. **Clone the repository**:
   Clone the project to your local machine. Replace `pshongwe` with your actual GitHub username if you have forked or created a repository for this project.
   ```bash
   git clone https://github.com/pshongwe/calculator_api.git
   cd calculator_api
  
2. ## Build and run


```bash
cargo run
```

3. ### Usage

The API provides four endpoints for basic arithmetic operations: addition, subtraction, multiplication, and division. The operations are performed using query parameters `a` and `b`.

#### Addition

- **Endpoint**: `/add`
- **Method**: GET
- **Query Parameters**:
  - `a`: First operand (float)
  - `b`: Second operand (float)

Example:

```sh
curl "http://127.0.0.1:8080/add?a=10&b=5"
```

Result:

```sh
15
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Actix Web](https://actix.rs/): A powerful, pragmatic, and extremely fast web framework for Rust.
- [Serde](https://serde.rs/): A framework for serializing and deserializing Rust data structures efficiently.