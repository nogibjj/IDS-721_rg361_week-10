# Rust Serverless Transformer
[![CI/CD](https://github.com/nogibjj/IDS-721_rg361_week-10/actions/workflows/cicd.yml/badge.svg)](https://github.com/nogibjj/IDS-721_rg361_week-10/actions/workflows/cicd.yml)

## Overview

In this project we will deploy a simple rust fuction which uses an LLM from huggingface to continue the text given to it. The function will be deployed as a lambda function on AWS.

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
![local test](/resources/local_start.png)

5. We can use services like postman to send and receive the requests from the lambda function.  

6. Once the functionality is working as expected, make the production build


## App deployment

### Docker Image Creation
1. Do docker init in the project directory
2. modify the Dockerfile as required
3. Build the docker image  
![docker build](/resources/docker_build.png)

4. verify that the image is created either using the docker images command or viewing in the docker desktop
![docker image](/resources/docker_image.png)

### AWS ECR

We will use AWS ECR service to store our docker image so that we can use it to build the lambda function.
1. Create a new **private** repository in ECR
2. once the repository is created, click on the "view push commands" button to get the commands to push the image to the repository
3. Run the commands in the terminal to push the image to the repository
![push image](/resources/docker_push.png)
4. Verify that the image is pushed to the repository by checking the repository in the AWS console
![ECR image](/resources/ecr.png)
5. copy the URI of the image to use it in the lambda function

### AWS Lambda Function

1. Go to AWS lambda and create a new function
2. select the container image as the source
3. paste the URI of the image we copied earlier in the repository URI
4. select the other settings as required
![lambda settings](/resources/lambda_setup.png)
5. Create the function
6. Once the function is created, goto configurations and enable function URL
7. The lambda function is now ready to be used
![lambda function](/resources/lambda_func.png)


## Sample Output

With no query, uses the default starter:  
![no query](/resources/local_auto.png)

With a query:
![with query](/resources/local_custom.png)

## GitHub Actions
Github Actions is used to automatically perform functions like Linting, formatting etc. whenever there is an update in the repository.
![CI/CD](/resources/cicd.png)