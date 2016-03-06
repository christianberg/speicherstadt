(ns speicherstadt.component.metrics-store
  (:require [com.stuartsierra.component :as component]
            [prometheus.core :as prometheus]
            [clojure.string :as string]))

(defprotocol Metrics
  (dump-metrics [metrics-store])
  (middleware [metrics-store handler app-name]))

(def request-latency-histogram-buckets (atom [0.001, 0.005, 0.010, 0.020, 0.050, 0.100, 0.200, 0.300, 0.500, 0.750, 1, 5]))

(defn- record-request-metric [metrics-store app-name request-method response-status request-time response-path]
  (let [status-class (str (int (/ response-status 100)) "XX")
        method-label (string/upper-case (name request-method))
        labels [method-label (str response-status) status-class response-path]]
    (prometheus/track-observation metrics-store app-name "http_request_latency_seconds" request-time labels)
    (prometheus/increase-counter metrics-store app-name "http_requests_total" labels)))

(defrecord MetricsStore []
  component/Lifecycle
  (start [this]
    (assoc this :store (atom (prometheus/init-defaults))))
  (stop [this]
    (if-let [store (:store this)]
      (.clear (:registry @store)))
    (dissoc this :store))
  Metrics
  (dump-metrics [this]
    (prometheus/dump-metrics
     (:registry @(:store this))))
  (middleware [this handler app-name]
    (swap! (:store this)
           prometheus/register-counter
           app-name
           "http_requests_total"
           "A counter of the total number of HTTP requests processed."
           ["method" "status" "statusClass" "path"])
    (swap! (:store this)
           prometheus/register-histogram
           app-name
           "http_request_latency_seconds"
           "A histogram of the response latency for HTTP requests in seconds."
           ["method" "status" "statusClass" "path"]
           @request-latency-histogram-buckets)
    (fn [request]
      (let [request-method (:request-method request)
            start-time (System/currentTimeMillis)
            response (handler request)
            finish-time (System/currentTimeMillis)
            response-status (get response :status 404)
            response-path (get (meta response) :path "unspecified")
            request-time (/ (double (- finish-time start-time)) 1000.0)]
        (record-request-metric @(:store this) app-name request-method response-status request-time response-path)
        response))))

(defn metrics-store []
  (->MetricsStore))
