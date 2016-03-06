(ns speicherstadt.middleware.metrics
  (:require [prometheus.core :as prometheus]
            [speicherstadt.component.metrics-store :as store]))

(defn wrap-metrics [handler app-name metrics-store]
  (store/middleware metrics-store handler app-name))

(defn wrap-path [handler]
  (fn [request]
    (handler (assoc request
                    :route-middleware
                    (fn [hndlr]
                      (fn [req]
                        (let [response (hndlr req)]
                          (prometheus/with-path req response))))))))
