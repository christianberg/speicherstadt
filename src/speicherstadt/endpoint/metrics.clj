(ns speicherstadt.endpoint.metrics
  (:require [speicherstadt.component.metrics-store :as metrics-store]
            [compojure.core :refer :all]))

(defn metrics-endpoint [{:keys [metrics-store]}]
  (routes
   (context "/metrics" []
            (GET "/" []
                 (metrics-store/dump-metrics metrics-store)))))
