// Package secrets gets stored secret data from Secret Manager.
//
// To create a new secret:
//
// $ gcloud --project $CLOUD_PROJECT secrets create $SECRET_NAME \
//       --replication-policy automatic
// $ gcloud --project $CLOUD_PROJECT secrets version add $SECRET_NAME \
//       --data-file "/path/to/file.txt"
//
// To access the stored secret:
//
// bs, err := secrets.GetSecret(context.Background(), secretName)
//
// See
// https://cloud.google.com/secret-manager/docs/creating-and-accessing-secrets
package secrets

import (
	"context"
	"fmt"
	"log"
	"sync"

	secretmanager "cloud.google.com/go/secretmanager/apiv1"
	"github.com/nya3jp/icfpc2020/draftcode/pkg/configs"
	secretmanagerpb "google.golang.org/genproto/googleapis/cloud/secretmanager/v1"
)

var (
	smClient     *secretmanager.Client
	smClientInit sync.Once
)

func getSecretManagerClient() *secretmanager.Client {
	smClientInit.Do(func() {
		var err error
		smClient, err = secretmanager.NewClient(context.Background())
		if err != nil {
			log.Fatalf("Cannot create a SecretManager client: %v", err)
		}
	})
	return smClient
}

func GetSecret(ctx context.Context, name string) ([]byte, error) {
	req := &secretmanagerpb.AccessSecretVersionRequest{
		Name: fmt.Sprintf("projects/%s/secrets/%s/versions/latest", configs.CloudProject, name),
	}
	resp, err := getSecretManagerClient().AccessSecretVersion(ctx, req)
	if err != nil {
		return nil, fmt.Errorf("failed to access secret version: %v", err)
	}
	return resp.GetPayload().GetData(), nil
}
