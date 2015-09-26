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
    (is (= (-> {:uri "/chunks/dead-beef"
                :request-method :put}
               handler
               :status)
           204))))
