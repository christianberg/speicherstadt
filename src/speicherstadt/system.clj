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
            [speicherstadt.component.chunk-storage :refer [new-chunk-store]]
            speicherstadt.component.chunk-storage.memory
            speicherstadt.component.chunk-storage.file))

(def base-config
  {:app {:middleware [[wrap-not-found :not-found]
                      [wrap-defaults :defaults]
                      [wrap-json-response]]
         :not-found  "Resource Not Found"
         :defaults   (meta-merge api-defaults {})}
   :chunk-storage {:type :memory}})

(defn new-system [config]
  (let [config (meta-merge base-config config)]
    (-> (component/system-map
         :app  (handler-component (:app config))
         :http (jetty-server (:http config))
         :chunks (endpoint-component chunks-endpoint)
         :store (new-chunk-store (:chunk-storage config)))
        (component/system-using
         {:http [:app]
          :app  [:chunks]
          :chunks [:store]}))))
