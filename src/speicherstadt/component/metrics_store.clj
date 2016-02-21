(ns speicherstadt.component.metrics-store
  (:require [com.stuartsierra.component :as component]
            [prometheus.core :as prometheus]))

(defrecord MetricsStore []
  component/Lifecycle
  (start [this]
    (assoc this :store (atom (prometheus/init-defaults))))
  (stop [this]
    (dissoc this :store)))

(defn metrics-store []
  (->MetricsStore))
