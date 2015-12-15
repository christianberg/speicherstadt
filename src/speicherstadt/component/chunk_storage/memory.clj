(ns speicherstadt.component.chunk-storage.memory
  (:require [com.stuartsierra.component :as component]
            [clojure.java.io :as io]
            [speicherstadt.component.chunk-storage :as storage]))

(defn slurp-bytes
  "Slurp the bytes from a slurpable thing"
  [in]
  (with-open [out (java.io.ByteArrayOutputStream.)]
    (io/copy (io/input-stream in) out)
    (.toByteArray out)))

(defrecord MemoryStorageComponent []
  component/Lifecycle
  (start [component]
    (assoc component :chunk-data (atom (sorted-map))))
  (stop [component]
    (dissoc component :chunk-data))
  storage/Store
  (retrieve [component id]
    (when-let [value (get @(:chunk-data component) id)]
      (io/input-stream value)))
  (store [component id content]
    (assert (instance? java.io.InputStream content))
    (let [bytes (slurp-bytes content)]
      (when (force id)
        (swap! (:chunk-data component) assoc (force id) bytes))))
  (list-all [component]
    (keys @(:chunk-data component))))

(defmethod storage/new-chunk-store :memory [config]
  (->MemoryStorageComponent))
