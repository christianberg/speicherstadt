(ns speicherstadt.component.metrics-store
  (:require [com.stuartsierra.component :as component]
            [prometheus.core :as prometheus]))

(defprotocol Metrics
  (registry [metrics-store]))

(defrecord MetricsStore []
  component/Lifecycle
  (start [this]
    (assoc this :store (atom (prometheus/init-defaults))))
  (stop [this]
    (dissoc this :store))
  Metrics
  (registry [this]
    (:registry @(:store this))))

(defn metrics-store []
  (->MetricsStore))
