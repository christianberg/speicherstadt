(ns speicherstadt.system
  (:require [com.stuartsierra.component :as component]
            [duct.component.endpoint :refer [endpoint-component]]
            [duct.component.handler :refer [handler-component]]
            [duct.middleware.not-found :refer [wrap-not-found]]
            [meta-merge.core :refer [meta-merge]]
            [ring.component.jetty :refer [jetty-server]]
            [ring.middleware.defaults :refer [wrap-defaults api-defaults]]
            [ring.middleware.json :refer [wrap-json-response]]
            [speicherstadt.endpoint.chunks :refer [chunks-endpoint]]
            [speicherstadt.endpoint.blobs :refer [blobs-endpoint]]
            [speicherstadt.component.chunk-storage :refer [new-chunk-store]]
            speicherstadt.component.chunk-storage.memory
            speicherstadt.component.chunk-storage.file
            [speicherstadt.component.metrics-store :refer [metrics-store]]
            [speicherstadt.endpoint.metrics :refer [metrics-endpoint]]
            [speicherstadt.middleware.metrics :refer [wrap-metrics wrap-path]]))

(def base-config
  {:app {:middleware [[wrap-not-found :not-found]
                      [wrap-defaults :defaults]
                      [wrap-json-response]
                      [wrap-metrics :app-name :metrics-store]
                      [wrap-path]]
         :app-name "speicherstadt"
         :not-found  "Resource Not Found"
         :defaults   (meta-merge api-defaults {})}
   :chunk-storage {:type :memory}})

(defn new-system [config]
  (let [config (meta-merge base-config config)]
    (-> (component/system-map
         :app  (handler-component (:app config))
         :http (jetty-server (:http config))
         :chunks (endpoint-component chunks-endpoint)
         :blobs (endpoint-component blobs-endpoint)
         :store (new-chunk-store (:chunk-storage config))
         :metrics-store (metrics-store)
         :metrics (endpoint-component metrics-endpoint))
        (component/system-using
         {:http [:app]
          :app  [:chunks :blobs :metrics :metrics-store]
          :chunks [:store]
          :metrics [:metrics-store]}))))
