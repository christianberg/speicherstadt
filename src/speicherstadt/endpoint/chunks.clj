(ns speicherstadt.endpoint.chunks
  (:require [compojure.core :refer :all]
            [ring.util.response :refer :all]
            [speicherstadt.component.chunk-storage :as storage])
  (:import [java.math BigInteger]
           [java.security DigestInputStream MessageDigest]))

(defn calculate-hash [stream]
  (->>
   (.. stream
       getMessageDigest
       digest)
   (BigInteger. 1)
   (format "sha256-%064x")))

(defn chunks-endpoint [{:keys [store]}]
  (routes
   (context "/chunks" []
            (GET "/" []
                 (fn [_]
                   (-> (response (or (storage/list-all store) []))
                       (content-type "application/json"))))
            (GET "/:id" [id]
                 (fn [_]
                   (if-let [value (storage/retrieve store id)]
                     (response value)
                     (not-found "Chunk not found"))))
            (PUT "/:id" [id]
                 (fn [request]
                   (let [wrapped-stream (-> (:body request)
                                            (DigestInputStream.
                                             (MessageDigest/getInstance "SHA-256")))
                         stream-hash (delay (let [hash (calculate-hash wrapped-stream)]
                                              (when (= hash id) hash)))]
                     (storage/store store stream-hash wrapped-stream)
                     (if (force stream-hash)
                       (created (str "/chunks/" (force stream-hash)))
                       (status {} 400)))))
            (POST "/" []
                  (fn [request]
                    (let [wrapped-stream (-> (:body request)
                                             (DigestInputStream.
                                              (MessageDigest/getInstance "SHA-256")))
                          stream-hash (delay (calculate-hash wrapped-stream))]
                      (storage/store store stream-hash wrapped-stream)
                      (created (str "/chunks/" (force stream-hash)))))))))
