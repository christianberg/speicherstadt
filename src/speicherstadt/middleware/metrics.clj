(ns speicherstadt.middleware.metrics
  (:require [prometheus.core :as prometheus]
            [speicherstadt.component.metrics-store :as store])
  (:import [clojure.lang IObj]))

(defn wrap-metrics [handler app-name metrics-store]
  (store/middleware metrics-store handler app-name))

(defn with-path
  "Adds the matched compojure route as the :path response metadata attribute"
  [request response]
  (if-let [route (last (:compojure/route request))]
    (if (instance? IObj response)
      (let [response-meta (or (meta response) {})
            context (or (:context request) "")]
        (with-meta response (assoc response-meta :path (str context route))))
      response)
    response))

(defn wrap-path [handler]
  (fn [request]
    (handler (assoc request
                    :route-middleware
                    (fn [hndlr]
                      (fn [req]
                        (let [response (hndlr req)]
                          (with-path req response))))))))
