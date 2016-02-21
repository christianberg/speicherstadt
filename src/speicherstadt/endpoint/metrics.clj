(ns speicherstadt.endpoint.metrics
  (:require [compojure.core :refer :all]
            [prometheus.core :as prometheus]))

(defn metrics-endpoint [config]
  (routes
   (context "/metrics" []
            (GET "/" [] (prometheus/dump-metrics (-> config
                                                     :metrics-store
                                                     :store
                                                     deref
                                                     :registry))))))
