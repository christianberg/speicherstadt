(ns speicherstadt.endpoint.chunks
  (:require [compojure.core :refer :all]
            [ring.util.response :refer :all]
            [speicherstadt.component.chunk-storage :as storage]))

(defn chunks-endpoint [{:keys [store]}]
  (routes
   (context "/chunks" []
            (GET "/" [] (fn [_]
                          (-> (response (or (storage/list-all store) []))
                              (content-type "application/json"))))
            (GET "/:id" [id] (fn [_]
                               (if-let [value (storage/retrieve store id)]
                                 (response value)
                                 (not-found "Chunk not found"))))
            (PUT "/:id" [id] (fn [request]
                               (storage/store store id (slurp (:body request)))
                               (status {} 204))))))
