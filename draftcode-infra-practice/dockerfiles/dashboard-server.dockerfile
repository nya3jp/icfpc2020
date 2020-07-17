FROM gcr.io/distroless/base-debian10
COPY draftcode/dashboard-server /app/dashboard-server
COPY draftcode/static /app/static
ENTRYPOINT ["/app/dashboard-server"]
