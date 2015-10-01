(ns speicherstadt.acceptance.chunks
  (:require [speicherstadt.system :as system]
            [com.stuartsierra.component :as component]
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
    (testing "PUT a new chunk"
      (is (= (-> {:uri "/chunks/dead-beef"
                  :request-method :put
                  :body "Hello World"}
                 handler
                 :status)
             204)))
    (testing "GET an existing chunk"
      (let [response (handler {:uri "/chunks/dead-beef"
                               :request-method :get})]
        (is (= (:body response) "Hello World"))
        (is (= (:status response) 200))))
    (testing "GET a non-existing chunk"
      (is (= (-> {:uri "/chunks/i-dont-exist"
                  :request-method :get}
                 handler
                 :status)
             404)))
    (testing "GET a list of all chunks"
      (let [response (handler {:uri "/chunks"
                               :request-method :get})]
        (is (= (:status response) 200))
        (is (= (:body response) "[\"dead-beef\"]"))))))
