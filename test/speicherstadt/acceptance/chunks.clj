(ns speicherstadt.acceptance.chunks
  (:require [speicherstadt.system :as system]
            [com.stuartsierra.component :as component]
            [cheshire.core :as json]
            [clojure.java.io :as io]
            [clojure.test :refer :all]))

(def ^:dynamic *system* nil)

(defn setup-system [f]
  (binding [*system* (component/start
                      (system/new-system {:http {:port 3333}}))]
    (f)
    (component/stop *system*)))

(use-fixtures :each setup-system)

(deftest chunk-acceptance
  (let [handler (-> *system* :app :handler)]
    (testing "List of chunks is empty before uploading any chunks"
      (let [response (handler {:uri "/chunks"
                               :request-method :get})]
        (is (= (:status response) 200))
        (is (= (get-in response [:headers "Content-Type"])
               "application/json"))
        (is (= (json/parse-string (:body response) true) []))))
    (testing "PUT a new chunk"
      (is (= (-> {:uri "/chunks/sha256-a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e"
                  :request-method :put
                  :headers {"Content-Type" "application/octet-stream"}
                  :body (java.io.ByteArrayInputStream. (.getBytes "Hello World"))}
                 handler
                 :status)
             204)))
    (testing "PUTting a chunk with wrong hash fails"
      (is (= (-> {:uri "/chunks/sha256-12345"
                  :request-method :put
                  :body (java.io.ByteArrayInputStream. (.getBytes "Hello World"))}
                 handler
                 :status)
             400)))
    (testing "GET an existing chunk"
      (let [response (handler {:uri "/chunks/sha256-a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e"
                               :request-method :get})]
        (is (= (slurp (:body response)) "Hello World"))
        (is (= (:status response) 200))
        (is (= (get-in response [:headers "Content-Type"])
               "application/octet-stream"))))
    (testing "GET a non-existing chunk"
      (is (= (-> {:uri "/chunks/i-dont-exist"
                  :request-method :get}
                 handler
                 :status)
             404)))
    (testing "GET a list of one chunk"
      (let [response (handler {:uri "/chunks"
                               :request-method :get})]
        (is (= (:status response) 200))
        (is (= (get-in response [:headers "Content-Type"])
               "application/json"))
        (is (= (json/parse-string (:body response) true)
               ["sha256-a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e"]))))
    (testing "GET a list of two chunks"
      (handler {:uri "/chunks/sha256-c3a26588bb78f6c08a0ef07ad88a5a0f9ff3f66940606c656d44cb6a239c6343"
                :request-method :put
                :headers {"Content-Type" "application/octet-stream"}
                :body (java.io.ByteArrayInputStream. (.getBytes "Hello Foo"))})
      (let [response (handler {:uri "/chunks"
                               :request-method :get})]
        (is (= (:status response) 200))
        (is (= (get-in response [:headers "Content-Type"])
               "application/json"))
        (is (= (json/parse-string (:body response) true)
               ["sha256-a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e"
                "sha256-c3a26588bb78f6c08a0ef07ad88a5a0f9ff3f66940606c656d44cb6a239c6343"]))))
    (testing "PUT and GET a binary chunk"
      (let [size 1000
            upload (byte-array (take size (repeatedly #(- (rand-int 256) 128))))
            download (byte-array size)
            digest (java.math.BigInteger. 1 (.digest (java.security.MessageDigest/getInstance "SHA-256") upload))
            uri (format "/chunks/sha256-%064x" digest)
            up-response (handler {:uri uri
                                  :request-method :put
                                  :headers {"Content-Type" "application/octet-stream"}
                                  :body (io/input-stream upload)})
            down-response (handler {:uri uri
                                    :request-method :get})]
        (.read (:body down-response) download 0 size)
        (is (= (seq upload) (seq download)))))))
