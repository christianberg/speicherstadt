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
      (is (= (-> {:uri "/chunks/dead-beef"
                  :request-method :put
                  :headers {"Content-Type" "application/octet-stream"}
                  :body (java.io.ByteArrayInputStream. (.getBytes "Hello World"))}
                 handler
                 :status)
             204)))
    (testing "GET an existing chunk"
      (let [response (handler {:uri "/chunks/dead-beef"
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
               ["dead-beef"]))))
    (testing "GET a list of two chunks"
      (handler {:uri "/chunks/abc"
                :request-method :put
                :headers {"Content-Type" "application/octet-stream"}
                :body (java.io.ByteArrayInputStream. (.getBytes "Hello Foo"))})
      (let [response (handler {:uri "/chunks"
                               :request-method :get})]
        (is (= (:status response) 200))
        (is (= (get-in response [:headers "Content-Type"])
               "application/json"))
        (is (= (json/parse-string (:body response) true)
               ["abc"
                "dead-beef"]))))
    (testing "PUT and GET a binary chunk"
      (let [size 1000
            upload (byte-array (take size (repeatedly #(- (rand-int 256) 128))))
            download (byte-array size)
            up-response (handler {:uri "/chunks/binary"
                                  :request-method :put
                                  :headers {"Content-Type" "application/octet-stream"}
                                  :body (io/input-stream upload)})
            down-response (handler {:uri "/chunks/binary"
                                    :request-method :get})]
        (.read (:body down-response) download 0 size)
        (is (= (seq upload) (seq download)))))))
