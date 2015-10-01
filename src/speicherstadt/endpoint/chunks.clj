(ns speicherstadt.endpoint.chunks
  (:require [compojure.core :refer :all]
            [speicherstadt.component.chunk-storage :as storage]))

(defn chunks-endpoint [{:keys [store]}]
  (routes
   (context "/chunks" []
            (GET "/" [] (fn [_]
                          {:status 200
                           :body (str (vec (storage/list-all store)))}))
            (GET "/:id" [id] (fn [_]
                               (if-let [value (storage/retrieve store id)]
                                 {:status 200 :body value}
                                 {:status 404})))
            (PUT "/:id" [id] (fn [request]
                               (storage/store store id (slurp (:body request)))
                               {:status 204})))))
