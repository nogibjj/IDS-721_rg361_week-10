FROM ghcr.io/cargo-lambda/cargo-lambda:latest as builder

WORKDIR /usr/app

COPY . .

RUN cargo clean && cargo lambda build --release --arm64

FROM public.ecr.aws/lambda/provided:al2-arm64

WORKDIR /rust_llm

COPY --from=builder /usr/app/target/ ./
COPY --from=builder /usr/app/src/pythia-410m-q4_0-ggjt.bin ./

ENTRYPOINT ["/lambda-entrypoint.sh"]
