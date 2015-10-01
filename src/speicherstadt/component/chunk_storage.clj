(ns speicherstadt.component.chunk-storage)

(defprotocol Store
  (retrieve [store id])
  (store [store id content])
  (list-all [store]))
