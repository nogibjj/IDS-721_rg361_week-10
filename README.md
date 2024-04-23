# Rust Serverless Transformer

## Overview

In this project we will deploy a simple rust fuction which uses a LLMs from huggingface to continue the text given to it. The function will be deployed as a lambda function on AWS.

## Prerequisites
Make sure you have the following installed:
- [Docker](https://docs.docker.com/get-docker/)
- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo Lambda](https://www.cargo-lambda.info/guide/installation.html)
- [AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-install.html)

## App Development

1. Create a new rust project using cargo lambda
```bash
cargo lambda new <project-name>
```

2. Use the desired model from huggingface, in this case, we used a [rustformers/pythia-ggml](https://huggingface.co/rustformers/pythia-ggml) model.
don't forget to add the correct path to the model in the file

3. Build the rust code as required, in this case we take in the query from the user and complete it using the model. In case there is no query, we use a default query to start the completion.

4. Test the code locally using the following command
```bash
cargo lambda watch
```

5. We can use services like postman to send and receive the requests from the lambda function.

6. Once the functionality is working as expected, make the production build

7. use Docker to dockerize the function along with the model file

## App deployment

1. Upload the docker image to AWS ECR
2. Go to AWS lambda and create a new function
3. select the container image as the source
4. provide the ECR image URI
5. Set the memory and timeout as required (suggested to set higher since the model takes time to load)
6. Create the function


## GitHub Actions
Github Actions is used to automatically perform functions like Linting, formatting etc. whenever there is an update in the repository.
