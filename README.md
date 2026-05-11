# K-cloud-sync-rust

## An easy way to backup and download files 

Create a single JSON config file to sinc your files in a single command to any k-cloud-backend intance

## build

To build a release version of the program run:

```bash
./build-release.sh
```

## Setup

Create a JSON file with the next structure:

```json
{
  "base_url": "http://192.168.122.125:5000/api",
  "api_key": "API_KEY", 
  "dirs": [
    {
      "remote_path": "example-dir/files-get",
      "local_path": "/home/alpine/example-get-dir/",
      "sync_mode": "get"
    },
    {
      "remote_path": "example-dir/files-send",
      "local_path": "/home/alpine/example-send-dir/",
      "sync_mode": "send"
    },
    {
      "remote_path": "example-dir/files-bidirectional",
      "local_path": "/home/alpine/example-bid-dir/",
      "sync_mode": "bidirectional"
    }
  ]
}
```

where the config itself is self explanatory

Note: the next API scopes are mandatory: `files:read` and `files:write`

## Usage

Once you created a config file, there is 2 ways to use it:

* synchronize all directories of the JSON file:

```bash
./k-cloud-sync-rust sync example.json
```

* synchronize one directorie: 

this command only sinchronizes the first directory

```bash
./k-cloud-sync-rust sync example.json 0
```

