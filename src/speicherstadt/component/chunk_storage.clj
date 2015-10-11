(ns speicherstadt.component.chunk-storage)

(defprotocol Store
  (retrieve [store id] "Retrieves the data previously stored under the key id. Returns an instance of java.io.OutputStream.")
  (store [store id content] "Stores the data from content under the key id. content must implement the java.io.InputStream interface.")
  (list-all [store] "Returns a sequence of all ids for which data is stored."))
