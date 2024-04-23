use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};
use std::{convert::Infallible, io::Write, path::PathBuf};

// Model inference function
fn infer(prompt: String) -> Result<String, Box<dyn std::error::Error>> {
    let tokenizer_source = llm::TokenizerSource::Embedded;
    let model_architecture = llm::ModelArchitecture::GptNeoX;

    // Path to the model
    let model_path = PathBuf::from("src/pythia-1b-q4_0-ggjt.bin");

    let prompt = prompt.to_string();
    let model = llm::load_dynamic(
        Some(model_architecture),
        &model_path,
        tokenizer_source,
        Default::default(),
        llm::load_progress_callback_stdout,
    )?;

    let mut session = model.start_session(Default::default());
    let mut generated_tokens = String::new();

    let res = session.infer::<Infallible>(
        model.as_ref(),
        &mut rand::thread_rng(),
        &llm::InferenceRequest {
            prompt: (&prompt).into(),
            parameters: &llm::InferenceParameters::default(),
            play_back_previous_tokens: false,
            // define the maximum number of tokens to generate
            maximum_token_count: Some(50),
        },
        // OutputRequest
        &mut Default::default(),
        |r| match r {
            llm::InferenceResponse::PromptToken(t) | llm::InferenceResponse::InferredToken(t) => {
                print!("{t}");
                std::io::stdout().flush().unwrap();
                generated_tokens.push_str(&t);
                Ok(llm::InferenceFeedback::Continue)
            }
            _ => Ok(llm::InferenceFeedback::Continue),
        },
    );

    // Return statement
    match res {
        Ok(_) => Ok(generated_tokens),
        Err(err) => Err(Box::new(err)),
    }
}

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Get the query string parameter, if any, else use a default value
    let query = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("query"))
        .unwrap_or("My Favourite Food is");
    let message = match infer(query.to_string()) {
        Ok(inference_result) => {
            format!("{}", inference_result)
        }
        Err(err) => {
            format!("Error in inference: {:?}", err)
        }
    };
    // Print the generated response
    println!("Model generated response:");
    println!("{:?}", message);

    // Return the response
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
