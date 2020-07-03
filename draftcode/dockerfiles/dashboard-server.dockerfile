FROM gcr.io/distroless/base-debian10
COPY draftcode/dashboard-server /app/dashboard-server
ENTRYPOINT ["/app/dashboard-server"]
