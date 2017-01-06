# Speicherstadt

A system for storing data.

Heavily inspired by [Camlistore][]. If you're looking for a system
that's currently usable, check out Camlistore. Speicherstadt will
follow many of the same ideas, but is being implemented as a set of
microservices that communicate via HTTP. I hope this will allow for
easy extensibility. It might also be a horrible idea in terms of
performance - this remains to be seen.

[Camlistore]: http://camlistore.org/

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

## Components

This repository doesn't contain any code, but acts as a starting point
and guide to the different components that make up the system. The
following services currently exist:

- [speicherstadt-chunks-fs][]: An implementation of the chunk storage
  service that stores data on the local filesystem.
  [![Travis](https://img.shields.io/travis/christianberg/speicherstadt-chunks-fs.svg)](https://travis-ci.org/christianberg/speicherstadt-chunks-fs)
  [![Docker Automated Build](https://img.shields.io/docker/automated/christianberg/speicherstadt-chunks-fs.svg)](https://hub.docker.com/r/christianberg/speicherstadt-chunks-fs/)

[speicherstadt-chunks-fs]: https://github.com/christianberg/speicherstadt-chunks-fs

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


## Legal

Copyright © 2016,2017 Christian Berg

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
