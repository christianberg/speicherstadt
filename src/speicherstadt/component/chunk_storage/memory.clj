(ns speicherstadt.component.chunk-storage.memory
  (:require [com.stuartsierra.component :as component]
            [speicherstadt.component.chunk-storage :as storage]))

(defrecord MemoryStorageComponent []
  component/Lifecycle
  (start [component]
    (assoc component :chunk-data (atom {})))
  (stop [component]
    (dissoc component :chunk-data))
  storage/Store
  (retrieve [component id]
    (get @(:chunk-data component) id))
  (store [component id content]
    (swap! (:chunk-data component) assoc id content))
  (list-all [component]
    (keys @(:chunk-data component))))
