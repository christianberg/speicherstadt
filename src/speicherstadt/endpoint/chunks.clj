(ns speicherstadt.endpoint.chunks
  (:require [compojure.core :refer :all]
            [ring.util.response :refer :all]
            [clojure.java.io :as io]
            [speicherstadt.component.chunk-storage :as storage]))

(defn slurp-bytes
  "Slurp the bytes from a slurpable thing"
  [in]
  (with-open [out (java.io.ByteArrayOutputStream.)]
    (io/copy (io/input-stream in) out)
    (.toByteArray out)))

(defn chunks-endpoint [{:keys [store]}]
  (routes
   (context "/chunks" []
            (GET "/" [] (fn [_]
                          (-> (response (or (storage/list-all store) []))
                              (content-type "application/json"))))
            (GET "/:id" [id] (fn [_]
                               (if-let [value (storage/retrieve store id)]
                                 (response (io/input-stream value))
                                 (not-found "Chunk not found"))))
            (PUT "/:id" [id] (fn [request]
                               (storage/store store id (slurp-bytes (:body request)))
                               (status {} 204))))))
