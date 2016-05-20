# speicherstadt

A system for storing data.

Heavily inspired by [Camlistore][]. If you're looking for a system
that's currently usable, check out Camlistore. Speicherstadt will
follow many of the same ideas, but is being implemented as a set of
microservices that communicate via HTTP. I hope this will allow for
easy extensibility. It might also be a horrible idea in terms of
performance - this remains to be seen.

[Camlistore]: http://camlistore.org/

[![Build Status](https://travis-ci.org/christianberg/speicherstadt.svg?branch=master)](https://travis-ci.org/christianberg/speicherstadt) [![Dependency Status](https://www.versioneye.com/user/projects/568189c3eb4f47003c0008bd/badge.svg?style=flat)](https://www.versioneye.com/user/projects/568189c3eb4f47003c0008bd)

## Current status

Not much is there yet. Currently only the `chunks` service exists,
which allows for pieces of arbitrary data to be stored and then
retrieved by their hash sums. The current implementation uses the
local filesystem for storage, but other storage backends (e.g. S3) are
easy to implement. Chunk size is only limited by the underlying
storage backend, but the idea is for chunks to be relatively small (~
100-500 kB max) and to split larger pieces of data into chunks (hence
the name). This will be done by another service, yet to be
implemented.

## Examples

Create a new chunk with a string value (and print the response
headers):

```sh
curl -D - -X POST -H "Content-Type: application/octet-stream" -d "Hello World" http://localhost:3000/chunks
```

```
HTTP/1.1 201 Created
Date: Sun, 10 Jan 2016 15:50:04 GMT
Location: /chunks/sha256-a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e
Content-Type: application/octet-stream
Content-Length: 0
Server: Jetty(9.2.10.v20150310)
```

List all chunks:

```sh
curl http://localhost:3000/chunks
```

```
["sha256-a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e"]
```

Get the content of a single chunk:

```sh
curl http://localhost:3000/chunks/sha256-a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e
```

```
Hello World
```

## Developing

### Setup

When you first clone this repository, run:

```sh
lein setup
```

This will create files for local configuration, and prep your system
for the project.

### Environment

To begin developing, start with a REPL.

```sh
lein repl
```

Run `go` to initiate and start the system.

```clojure
user=> (go)
:started
```

By default this creates a web server at <http://localhost:3000>.

When you make changes to your source files, use `reset` to reload any
modified files and restart the server.

```clojure
user=> (reset)
:reloading (...)
:started
```

## Deploying

FIXME: steps to deploy

## Legal

Copyright © 2015 FIXME
