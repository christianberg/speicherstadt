(ns speicherstadt.component.chunk-storage.file
  (:require [com.stuartsierra.component :as component]
            [speicherstadt.component.chunk-storage :as storage]
            [me.raynes.fs :as fs]
            [clojure.java.io :as io]))

(defrecord FileStorageComponent [basedir]
  component/Lifecycle
  (start [component]
    component)
  (stop [component])
  storage/Store
  (retrieve [component id]
    (let [path (fs/file basedir id)]
      (when (fs/file? path)
        (io/input-stream path))))
  (store [component id content]
    (let [path (fs/file basedir id)]
      (io/copy content path)))
  (list-all [component]
    (map fs/base-name (fs/list-dir basedir))))

(defmethod storage/new-chunk-store :file [config]
  (->FileStorageComponent (:basedir config)))
