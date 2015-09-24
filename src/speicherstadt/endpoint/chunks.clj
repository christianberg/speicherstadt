(ns speicherstadt.endpoint.chunks
  (:require [compojure.core :refer :all]))

(defn chunks-endpoint [config]
  (routes
   (context "/chunks" []
            (PUT "/:id" [id] (fn [_] {:status 204})))))
