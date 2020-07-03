// Package storage provides an API to upload / download files to Google Cloud
// Storage.
package storage

import (
	"context"
	"io"
	"log"
	"os"
	"path/filepath"
	"strings"
	"sync"

	"cloud.google.com/go/storage"
	"github.com/google/uuid"
	"github.com/nya3jp/icfpc2020/draftcode/pkg/configs"
	"google.golang.org/api/iterator"
)

var (
	storageClient     *storage.Client
	storageClientInit sync.Once

	bucket     *storage.BucketHandle
	bucketInit sync.Once
)

func getStorageClient() *storage.Client {
	storageClientInit.Do(func() {
		var err error
		storageClient, err = storage.NewClient(context.Background())
		if err != nil {
			log.Fatalf("Cannot create a Cloud Storage client: %v", err)
		}
	})
	return storageClient
}

// Folder represents a second-level directory in Cloud Storage.
//
// Cloud Storage doesn't have a notion of directory, but this comes handy. You
// can create a folder for each test run, move all test outputs to a specific
// folder, and upload them with UploadLocalObjects from a test runner.
type Folder struct {
	folderID string
}

// NewFolder generates a new ID for a folder and returns a Folder object.
func NewFolder() *Folder {
	return &Folder{folderID: uuid.New().String()}
}

// Open returns a Folder object for the specified folder.
func Open(folderID string) *Folder {
	return &Folder{folderID: folderID}
}

// FolderID returns the folder ID.
func (f *Folder) FolderID() string { return f.folderID }

// NewReader opens a new Reader for the specified object.
func (f *Folder) NewReader(ctx context.Context, objectName string) (*storage.Reader, error) {
	return getStorageClient().Bucket(configs.StorageBucket).Object("/" + f.folderID + "/" + objectName).NewReader(ctx)
}

// NewWriter opens a new Writer for the specified object.
func (f *Folder) NewWriter(ctx context.Context, objectName string) *storage.Writer {
	return getStorageClient().Bucket(configs.StorageBucket).Object("/" + f.folderID + "/" + objectName).NewWriter(ctx)
}

// ListObjects returns all object names under the folder. They can be read with
// NewReader.
func (f *Folder) ListObjects(ctx context.Context) ([]string, error) {
	prefix := "/" + f.folderID + "/"
	it := getStorageClient().Bucket(configs.StorageBucket).Objects(ctx, &storage.Query{Prefix: prefix})
	names := []string{}
	for {
		attrs, err := it.Next()
		if err == iterator.Done {
			break
		}
		if err != nil {
			return nil, err
		}
		names = append(names, strings.TrimPrefix(attrs.Name, prefix))
	}
	return names, nil
}

// UploadLocalObjects uploads all objects under the specified dir to the folder.
func (f *Folder) UploadLocalObjects(ctx context.Context, localDir string) error {
	prefix := filepath.ToSlash(localDir)
	return filepath.Walk(localDir, func(p string, info os.FileInfo, err error) error {
		rd, err := os.Open(p)
		if err != nil {
			return err
		}
		defer rd.Close()
		objectName := strings.TrimPrefix(filepath.ToSlash(p), prefix)
		wt := f.NewWriter(ctx, objectName)
		defer wt.Close()
		_, err = io.Copy(wt, rd)
		return err
	})
}
