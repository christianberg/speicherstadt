(ns speicherstadt.middleware.metrics
  (:require [prometheus.core :as prometheus]))

(defn wrap-metrics [handler app-name metrics-store]
  (prometheus/instrument-handler handler
                                 app-name
                                 (:registry @(:store metrics-store))))
