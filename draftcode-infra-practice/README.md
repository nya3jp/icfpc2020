# draftcode infra (practice)

This is infra practice code so that we can use some parts as a scaffold.

- `draftcode/pkg/secrets`: Read secrets from Secret Manager.
- `draftcode/pkg/storage`: Small wrapper around Cloud Storage to upload,
  download, and list artifacts.
- `draftcode/protos`: The protobuf package. See below to generate Go protobuf
  code.
- `draftcode/cmd/api-server`: gRPC API server. See below for details.
- `draftcode/cmd/dashboard-server`: HTTP server. See below for details.

## TODO

- Cloud SQL Postgres server setup & DB migrations
- Small task queue system
- Cloud Build config for Bazel & task triggering

## Dev Setup

Requires gcloud command. Some commands require Application Default Credentials.
Run:

```
CLOUD_PROJECT=...
gcloud auth login
gcloud --project "$CLOUD_PROJECT" auth application-default login
```

These commands obtains a credential for gcloud and a credential for GCP
libraries.

## `draftcode/pkg/secrets`

Read secrets from https://cloud.google.com/secret-manager/docs. Small
credentials can be stored in
https://console.cloud.google.com/security/secret-manager and they can be
retrieved via this library.

## `draftcode/pkg/storage`

If we run tests in a remote machine, we would like to save some artifacts. This
library is to upload those artifacts. The API servers or the HTTP servers can
read them via the library. If the GCS bucket is open to public, the resources
should be accessible directly.

## `draftcode/protos`

- Check in the generated protobuf code.
- Install protoc (https://grpc.io/docs/protoc-installation/)
- Install protoc-gen-go via
  `go get -u github.com/golang/protobuf/protoc-gen-go`.
- Run `draftcode/scripts/protoc.sh`.

## `draftcode/cmd/api-server`

- gRPC service definition is in `draftcode/protos/infra.proto`.
- The binary is built with Cloud Build. The config is in
  `draftcode/cloudbuilds/api-server.yaml`.
- See https://cloud.google.com/cloud-build/docs/build-config for the format.
- The server Dockerfile is in `draftcode/dockerfiles/api-server.dockerfile`.
- The Cloud Build run can be started with
  `gcloud builds submit --config draftcode/cloudbuilds/api-server.yaml`.
  This creates a new Docker image in Container Registry. You can run this server
  from Cloud Run from the console easily.
- It seems that we can use Cloud Run's authz for gRPC servers, but the IDToken
  generated with gcloud's Application Default Credential cannot be used with it.
  The IDTokens created with `gcloud auth print-identity-token` work. So far, in
  order to use this gRPC API server, it's probably easier to use without auth or
  invoke `gcloud` to obtain creds.

## `draftcode/cmd/dashboard-server`

- The binary is built with Cloud Build. The config is in
  `draftcode/cloudbuilds/dashboard-server.yaml`.
- The server Dockerfile is in
  `draftcode/dockerfiles/dashboard-server.dockerfile`.
- The Cloud Build run can be started with
  `gcloud builds submit --config draftcode/cloudbuilds/dashboard-server.yaml`.
  Same as the gRPC server.
- I tried to merge api-server and dashboard-server. It seems that if a server
  runs behind Cloud Run, it cannot receive requests as HTTP/2 (then why the
  standalone gRPC server can receive requests?). I cannot make a gRPC/HTTP
  hybrid server work.
- Files in `draftcode/static` are served from `/static/`.
