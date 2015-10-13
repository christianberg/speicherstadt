(ns speicherstadt.endpoint.chunks
  (:require [compojure.core :refer :all]
            [ring.util.response :refer :all]
            [speicherstadt.component.chunk-storage :as storage])
  (:import [java.math BigInteger]
           [java.io BufferedInputStream]
           [java.security DigestInputStream MessageDigest]))

(defn read-all-bytes [stream]
  (if (= (.read stream) -1)
    stream
    (recur stream)))

(defn calculate-hash! [stream]
  (doto stream
    (.mark 1000000000)
    read-all-bytes
    .reset)
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
                                            BufferedInputStream.
                                            (DigestInputStream.
                                             (MessageDigest/getInstance "SHA-256")))
                         stream-hash (calculate-hash! wrapped-stream)]
                     (if (= stream-hash id)
                       (do
                         (storage/store store id wrapped-stream)
                         (status {} 204))
                       (status {} 400))))))))
