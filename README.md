# Serca-core: The Open-Source Foundation of Serca

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Contributions Welcome](https://img.shields.io/badge/Contributions-Welcome-brightgreen.svg)](CONTRIBUTING.md)

Serca-core is the foundational, open-source component of Serca, an AI-powered media search engine. This repository houses the core logic and infrastructure that enable the indexing and searching of various media types.

**Important Note:** This repository contains the essential building blocks of Serca. Advanced features, proprietary algorithms, and sensitive configurations maybe omitted to a paid tier

## What is Serca-core?

Serca-core provides the fundamental capabilities for:

* **Media Ingestion:** Basic mechanisms for processing and indexing different types of media (e.g., text files, common image formats like JPEG and PNG).
* **Core Search Functionality:** Algorithms and data structures for efficient and relevant media retrieval based on keyword matching.
* **Basic Media Analysis:** Foundational analysis techniques to extract key information from media content.
* **API Foundation:** A core API that allows interaction with the search engine's basic functionalities.

Our goal with Serca-core is to build a robust and accessible open-source base for media search innovation, fostering community contributions and transparency in the fundamental aspects of the Serca engine.

## Key Features (of Serca-core)

* **Basic Keyword Search:** Basic keyword matching (todo)
* **Support for Common Media Types:** Indexing of common media file types (.jpg, .pdf, .mp3/4, .mov)

## Getting Started

Follow these steps to get Serca-core running in your local environment:

## Donations
We plan to host and run this as a service. That is not cheap at all between the web serivces and GPU hosting for training and inferencing the AI models. We are asking for donations no matter how big or small. We will be looking for sponcers and funding, but if your able please concider donating. - Kilroy
bc1qtk5ffv36hlguj9q2h6687reuaqmyqtfzv8utwv - BTC
0x4454540DFbBBE0BED02c1C736D9F269eE65A478e - ETH

**Donations Note**
I am in the process of creating a LLC to support this project. At that point I will switch all funding sources to there, for now it's a case of "Trust Me Bro". I will update this when that is correctly working.

### Matrix
Link to the Matrix room is here.
[Matrix](https://matrix.to/#/#serca_browser:matrix.org)

### Prerequisites

* Python 3.8+
* rust
* pip (Python package installer)

### Installation

1.  **Clone the Repository:**

    ```bash
    git clone [https://github.com/](https://github.com/)[Your GitHub Username]/Serca-core.git
    cd Serca-core
    ```

2.  **Set Up a Virtual Environment (Recommended):**

    ```bash
    python -m venv venv
    source venv/bin/activate  # On macOS/Linux
    venv\Scripts\activate  # On Windows
    ```

3.  **Install Dependencies:**

    ```bash
    pip install -r requirements.txt
    ```

    *(Make sure you have a `requirements.txt` file listing the necessary Python packages, including the Elasticsearch client.)*

4.  **Configuration:**

    * Copy the example configuration file:

        ```bash
        cp config.example.yaml config.yaml
        ```

    * Edit `config.yaml` to configure your Elasticsearch connection details, API settings, and other core parameters. Refer to the comments within the file for guidance.

5.  **Run Serca-core (Basic Example):**

    Assuming you have an Elasticsearch instance running on `localhost:9200` (or as configured in `config.yaml`), you can start a basic indexing and search example:

    ```bash
    python examples/basic_search.py
    ```

    *(This example script should demonstrate indexing a few sample files and performing a basic search. You'll need to create this `examples/basic_search.py` file.)*

## Contributing

We warmly welcome contributions to Serca-core! If you're interested in helping us build and improve the open-source foundation of Serca, please review our [CONTRIBUTING.md](CONTRIBUTING.md) file for guidelines on how to get started, submit bug reports, suggest features, and contribute code.

By contributing to Serca-core, you acknowledge that your contributions will be licensed under the MIT License.

## License

Serca-core is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
