FROM gcr.io/distroless/base-debian10
COPY draftcode/api-server /app/api-server
ENTRYPOINT ["/app/api-server"]
