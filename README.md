# speicherstadt

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

### Generators

This project has several [generators][] to help you create files.

* `lein gen endpoint <name>` to create a new endpoint
* `lein gen component <name>` to create a new component

[generators]: https://github.com/weavejester/lein-generate

## Deploying

FIXME: steps to deploy

## Legal

Copyright © 2015 FIXME
