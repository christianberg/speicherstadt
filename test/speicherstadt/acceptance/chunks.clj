(ns speicherstadt.acceptance.chunks
  (:require [speicherstadt.system :as system]
            [com.stuartsierra.component :as component]
            [ring.util.response :as r]
            [cheshire.core :as json]
            [clojure.java.io :as io]
            [clojure.test :refer :all]
            [me.raynes.fs :as fs]
            [ring.mock.request :as mock])
  (:import [java.io ByteArrayInputStream]
           [java.math BigInteger]
           [java.security MessageDigest]))

(def ^:dynamic *system* nil)

(defn string->stream [s]
  (ByteArrayInputStream. (.getBytes s)))

(deftest chunk-acceptance
  (let [hash-of {"Hello World" "sha256-a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e"
                 "Hello Foo" "sha256-c3a26588bb78f6c08a0ef07ad88a5a0f9ff3f66940606c656d44cb6a239c6343"}
        handler (-> *system* :app :handler)]
    (testing "List of chunks is empty before uploading any chunks"
      (let [response (handler (mock/request :get "/chunks"))]
        (is (= (:status response) 200))
        (is (= (r/get-header response "Content-Type")
               "application/json"))
        (is (= (json/parse-string (:body response) true) []))))
    (testing "PUT a new chunk"
      (is (= (-> (mock/request :put (str "/chunks/" (hash-of "Hello World")))
                 (mock/header "Content-Type" "application/octet-stream")
                 (mock/body "Hello World")
                 handler
                 :status)
             201)))
    (testing "PUTting a chunk with wrong hash fails"
      (is (= (-> (mock/request :put "/chunks/sha256-12345")
                 (mock/body "Hello World")
                 handler
                 :status)
             400)))
    (testing "GET an existing chunk"
      (let [response (handler (mock/request :get
                                            (str "/chunks/" (hash-of "Hello World"))))]
        (is (= (slurp (:body response)) "Hello World"))
        (is (= (:status response) 200))
        (is (= (r/get-header response "Content-Type")
               "application/octet-stream"))))
    (testing "GET a non-existing chunk"
      (is (= (-> (mock/request :get "/chunks/i-dont-exist")
                 handler
                 :status)
             404)))
    (testing "GET a list of one chunk"
      (let [response (handler (mock/request :get "/chunks"))]
        (is (= (:status response) 200))
        (is (= (r/get-header response "Content-Type")
               "application/json"))
        (is (= (json/parse-string (:body response) true)
               [(hash-of "Hello World")]))))
    (testing "POST a chunk"
      (let [response (-> (mock/request :post "/chunks")
                         (mock/body "Hello Foo")
                         handler)]
        (is (= (:status response) 201))
        (is (= (r/get-header response "Location")
               (str "/chunks/" (hash-of "Hello Foo"))))))
    (testing "GET a list of two chunks"
      (let [response (handler (mock/request :get "/chunks"))]
        (is (= (:status response) 200))
        (is (= (r/get-header response "Content-Type")
               "application/json"))
        (is (= (json/parse-string (:body response) true)
               (map hash-of ["Hello World" "Hello Foo"])))))
    (testing "PUT and GET a binary chunk"
      (let [size 1000
            upload (byte-array (take size (repeatedly #(- (rand-int 256) 128))))
            download (byte-array size)
            digest (BigInteger. 1 (.digest (MessageDigest/getInstance "SHA-256")
                                           upload))
            uri (format "/chunks/sha256-%064x" digest)
            up-response (-> (mock/request :put uri)
                            (mock/header "Content-Type" "application/octet-stream")
                            (assoc :body (io/input-stream upload))
                            handler)
            down-response (handler (mock/request :get uri))]
        (.read (:body down-response) download 0 size)
        (is (= (seq upload) (seq download)))))))

(deftest chunk-memory-storage
  (binding [*system* (component/start
                      (system/new-system {:http {:port 3333}
                                          :chunk-storage {:type :memory}}))]
    (chunk-acceptance)
    (component/stop *system*)))

(deftest chunk-file-storage
  (let [basedir (fs/temp-dir "chunktest")]
    (binding [*system* (component/start
                        (system/new-system {:http {:port 3333}
                                            :chunk-storage {:type :file
                                                            :basedir basedir}}))]
      (chunk-acceptance)
      (component/stop *system*)
      (fs/delete-dir basedir))))

(defn test-ns-hook []
  (chunk-memory-storage)
  (chunk-file-storage))
