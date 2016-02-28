(ns speicherstadt.endpoint.metrics
  (:require [speicherstadt.component.metrics-store :as metrics-store]
            [compojure.core :refer :all]
            [prometheus.core :as prometheus]))

(defn metrics-endpoint [{:keys [metrics-store]}]
  (routes
   (context "/metrics" []
            (GET "/" [] (prometheus/dump-metrics
                         (metrics-store/registry metrics-store))))))
