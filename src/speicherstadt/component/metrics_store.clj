(ns speicherstadt.component.metrics-store
  (:require [com.stuartsierra.component :as component]
            [prometheus.core :as prometheus]))

(defprotocol Metrics
  (dump-metrics [metrics-store]))

(defrecord MetricsStore []
  component/Lifecycle
  (start [this]
    (assoc this :store (atom (prometheus/init-defaults))))
  (stop [this]
    (dissoc this :store))
  Metrics
  (dump-metrics [this]
    (prometheus/dump-metrics
     (:registry @(:store this)))))

(defn metrics-store []
  (->MetricsStore))
