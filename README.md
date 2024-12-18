# BEP.AI Core

BEP.AI Core is an open-source framework for building customizable AI-powered agents. It allows developers to easily integrate multiple large language models (LLMs) and create agents with personalized behavior and capabilities. Whether you're building a simple chatbot or a complex AI assistant, BEP.AI Core offers the flexibility to suit a wide range of use cases.

## Features

- **Integration with Multiple LLMs**: Currently supports OpenAI's GPT models, with plans for future integrations (PaLM, Claude, etc.).
- **Customizable Agent Behavior**: Fine-tune agents' personalities, knowledge base, and responses.
- **Open Source**: Community-driven, with active contributions from developers around the world.
- **Easy Setup**: Designed for ease of installation and integration with existing systems.
- **Extensible**: Modular architecture that allows for easy addition of new features and LLMs.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Examples](#examples)
- [Contributing](#contributing)
- [License](#license)
- [FAQ](#faq)

## Installation

To install BEP.AI Core, you need to have [Rust](https://www.rust-lang.org/) installed on your machine.

### Prerequisites

- Rust (version 1.60.0 or later)
- A code editor (such as Visual Studio Code or IntelliJ)
- (Optional) Docker for containerized deployment

### Steps to Install

1. **Install Rust**:  
   Follow the [Rust installation guide](https://www.rust-lang.org/learn/get-started) to install `rustc` and `cargo`.

2. **Clone the Repository**:  
   Clone this repository to your local machine:
   ```bash
   git clone https://github.com/code/bepai-core.git
   cd bepai-core
   ```

3. **Build the Project**:  
   Build the project using Cargo:
   ```bash
   cargo build
   ```

4. **Run Tests**:  
   Ensure everything is working by running the tests:
   ```bash
   cargo test
   ```

Now, you're ready to start customizing and using BEP.AI Core!

## Usage

### Creating an Agent

1. **Create a New Agent**:  
   Create an agent using the `agent.rs` file. The agent is built with a client (e.g., OpenAI), a preamble, and customizable parameters.

   Example:
   ```rust
   use bepai_core::agent::Agent;
   use bepai_core::providers::openai::Client;

   let openai_client = Client::new("your-openai-api-key");
   let agent = Agent::new(openai_client, "Hello, I am your assistant.", 0.7);
   ```

2. **Interact with the Agent**:  
   Use the `prompt` method to interact with the agent and get responses.
   ```rust
   let response = agent.prompt("What is the capital of France?").await;
   println!("{}", response);
   ```

### Configuration Options

- **Preamble**: A string that defines the initial instructions or personality for the agent.
- **Temperature**: A floating-point value between 0 and 1 that controls the creativity and randomness of the responses.

### Example Use Cases

- **Basic Agent**: See the `examples/basic_agent.rs` for a simple example of how to set up and interact with an agent.
- **Advanced Agent**: See `examples/advanced_agent.rs` for more complex scenarios with multiple integrations and behaviors.

## Contributing

We welcome contributions to improve BEP.AI Core! To contribute:

1. Fork the repository.
2. Clone your fork to your local machine:
   ```bash
   git clone https://github.com/yourusername/bepai-core.git
   ```
3. Create a new branch for your changes:
   ```bash
   git checkout -b feature-branch
   ```
4. Make your changes and commit them:
   ```bash
   git add .
   git commit -m "Description of your changes"
   ```
5. Push your changes to your fork:
   ```bash
   git push origin feature-branch
   ```
6. Open a pull request from your fork to the `main` branch of the original repository.

Please ensure your changes follow the project's coding style, and add tests where necessary.

For more details, see the [CONTRIBUTING.md](CONTRIBUTING.md).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## FAQ

### What is BEP.AI Core?

BEP.AI Core is a framework for building AI-powered agents using multiple LLMs. It allows for extensive customization of agents' behavior and responses.

### How do I integrate a new LLM?

To integrate a new LLM, create a new provider in the `providers/` folder. You'll need to implement the communication logic for interacting with the model API.

### Can I use BEP.AI Core for commercial purposes?

Yes, BEP.AI Core is open-source and licensed under the MIT License, which allows for commercial use. Please check the [LICENSE](LICENSE) file for more details.

### How do I report a bug?

If you encounter a bug, please use the [Bug Report Template](.github/ISSUE_TEMPLATE/bug_report.md) to report the issue. Provide as much detail as possible to help us diagnose the problem.

---


